#![recursion_limit="256"]

use std::cell::RefCell;
use std::rc::Rc;

#[macro_use]
extern crate stdweb;
//use stdweb::unstable::TryInto;

extern crate nalgebra_glm as glm;

extern crate sample;
use sample::Signal;

//use stdweb::web::INonElementParentNode;


// Buffer an audio buffer sample to the given channel
fn js_play_buffer(js_ctx: &stdweb::Value, sample_buffer: &Vec<f32>) {
    js! {
        var h = @{js_ctx};
        var samples = @{unsafe { stdweb::UnsafeTypedArray::new(sample_buffer) }};

        var sample_count = samples.length;
        var sample_rate = 48000;

        var audio_buffer = h.audio.createBuffer(1, sample_count, sample_rate);

        audio_buffer.getChannelData(0).set(samples);

        var node = h.audio.createBufferSource();
        node.connect(h.audio.destination);
        node.buffer = audio_buffer;

        var latency = 0.1;
        var play_timestamp = h.audio.currentTime + latency;
        node.start(play_timestamp);
    }
}

const SAMPLE_RATE: usize = 48_000;
const SAMPLE_HZ: f64 = SAMPLE_RATE as f64;
const BEEP_SAMPLES: usize = SAMPLE_RATE/10;

struct AudioState {
    js_ctx: stdweb::Value,
    old_timestamp: f64,
    beep_a: Vec<f32>,
    beep_b: Vec<f32>,
    beep_c: Vec<f32>,
}

impl AudioState {
    fn new() -> AudioState {
        let element_audio = js! {
            return {
                audio: new AudioContext()
            };
        };

        let signal0 = sample::signal::rate(SAMPLE_HZ).const_hz(300.0).square().scale_amp(0.05);
        let signal1 = sample::signal::rate(SAMPLE_HZ).const_hz(400.0).square().scale_amp(0.05);
        let signal2 = sample::signal::rate(SAMPLE_HZ).const_hz(500.0).sine().scale_amp(0.05);
        let buffer_a: Vec<_> = signal0.add_amp(signal1).add_amp(signal2)
            .take(BEEP_SAMPLES)
            .map(|x|{x[0] as f32}).collect();

        let signal3 = sample::signal::rate(SAMPLE_HZ).const_hz(400.0).square().scale_amp(0.05);
        let signal4 = sample::signal::rate(SAMPLE_HZ).const_hz(500.0).square().scale_amp(0.05);
        let signal5 = sample::signal::rate(SAMPLE_HZ).const_hz(600.0).sine().scale_amp(0.05);
        let buffer_b: Vec<_> = signal3.add_amp(signal4).add_amp(signal5)
            .take(BEEP_SAMPLES)
            .map(|x|{x[0] as f32}).collect();

        let signal6 = sample::signal::rate(SAMPLE_HZ).const_hz(500.0).square().scale_amp(0.05);
        let signal7 = sample::signal::rate(SAMPLE_HZ).const_hz(600.0).square().scale_amp(0.05);
        let signal8 = sample::signal::rate(SAMPLE_HZ).const_hz(700.0).sine().scale_amp(0.05);
        let buffer_c: Vec<_> = signal6.add_amp(signal7).add_amp(signal8)
            .take(BEEP_SAMPLES)
            .map(|x|{x[0] as f32}).collect();

        AudioState {
            js_ctx: element_audio,
            old_timestamp: 0.0,
            beep_a: buffer_a,
            beep_b: buffer_b,
            beep_c: buffer_c,
        }
    }

    fn update(&mut self, timestamp: f64) {
        let delta = timestamp - self.old_timestamp;
        if delta < 1000.0 {
            return;
        }
        self.old_timestamp = timestamp;

        let beep: &Vec<f32>;

        if delta as u32 % 3 == 0 {
            beep = &self.beep_a;
        }
        else if delta as u32 % 3 == 1 {
            beep = &self.beep_b;
        }
        else {
            beep = &self.beep_c;
        }

        js_play_buffer(
            &self.js_ctx,
            beep);
    }
}


struct RenderState {
    last_timestamp: f64,
}

impl RenderState {
    fn draw(&mut self, timestamp: f64) {
        let _delta_msec = timestamp - self.last_timestamp;
        self.last_timestamp = timestamp;
    }
}


struct GameLoop {
    render: RenderState,
    audio: AudioState,
}

impl GameLoop {
    fn new() -> GameLoop {
        //let element_canvas = stdweb::web::document().get_element_by_id("game_canvas").unwrap();

        GameLoop {
            audio: AudioState::new(),
            render: RenderState {
                last_timestamp: 0.0,
            },
        }
    }
}

fn render_loop(looper: Rc<RefCell<GameLoop>>) {
    stdweb::web::window().request_animation_frame(move |timestamp: f64| {
        {
            let mut mlooper = looper.borrow_mut();
            mlooper.render.draw(timestamp);
            mlooper.audio.update(timestamp);
        }
        render_loop(looper);
    });
}


#[js_export]
fn handle_play_click(_e: stdweb::Value) {
    js! {
        console.log("TEST");
    };

    let looper = Rc::new(RefCell::new(GameLoop::new()));
    render_loop(looper);
}


fn main() {
    stdweb::initialize();
    stdweb::event_loop();
}
