#![recursion_limit="256"]

use std::cell::RefCell;
use std::rc::Rc;

#[macro_use]
extern crate stdweb;
use stdweb::unstable::TryInto;

extern crate nalgebra_glm as glm;

extern crate sample;
use sample::{Signal, Sample, Frame};

//use stdweb::web::INonElementParentNode;

struct AudioState {
    js_ctx: stdweb::Value,
    audio_buffer: Vec<f32>,
    last_timestamp: f64,
}

// Buffer an audio buffer sample to the given channel
fn js_buffer_audio(js_ctx: &stdweb::Value, channel: u32, sample_buffer: &Vec<f32>) -> f64 {
    let result : f64 = js! {
        var channel = @{channel};
        var h = @{js_ctx};
        var samples = @{unsafe { stdweb::UnsafeTypedArray::new(sample_buffer) }};

        var sample_count = samples.length;
        var sample_rate = 48000;
        var latency = 1500.0 / sample_rate;

        var audio_buffer;
        if (h.empty_audio_buffers.length === 0) {
            audio_buffer = h.audio.createBuffer(1, sample_count, sample_rate);
        } else {
            audio_buffer = h.empty_audio_buffers.pop();
        }
        audio_buffer.getChannelData(channel).set(samples);

        var node = h.audio.createBufferSource();
        node.connect(h.audio.destination);
        node.buffer = audio_buffer;
        node.onended = function() {
            h.empty_audio_buffers.push(audio_buffer);
        };

        var buffered = h.play_timestamp - (h.audio.currentTime + latency);
        var play_timestamp = Math.max(h.audio.currentTime + latency, h.play_timestamp);
        node.start(play_timestamp);
        h.play_timestamp = play_timestamp + sample_count / sample_rate;

        return buffered;
    }.try_into().unwrap();
    js! {
        console.log("Buffered audio: " + @{result});
    };
    return result;
}

impl AudioState {
    fn new() -> AudioState {
        let element_audio = js! {
            return {
                audio: new AudioContext(),
                empty_audio_buffers: [],
                play_timestamp: 0.0,
            };
        };

        AudioState {
            js_ctx: element_audio,
            audio_buffer: Vec::new(),
            last_timestamp: 0.0,
        }
    }
    fn add_sample(&mut self, sample: f32) {
        self.audio_buffer.push(sample);
        if self.audio_buffer.len() >= 2048 {
            js_buffer_audio(
                &self.js_ctx,
                0,
                &self.audio_buffer);
            self.audio_buffer.clear();
        }
    }

    fn update(&mut self, timestamp: f64) {
        let delta_msec = timestamp - self.last_timestamp;
        self.last_timestamp = timestamp;

        js! {
            console.log("Audio frame: delta_msec=" + @{delta_msec});
        };

        let hz = sample::signal::rate(48_000.0).const_hz(440.0).sine();
        let one_sec = 48_000 as usize;
        let frame: Vec<_> = hz.take(one_sec).map(|x|{x[0] as f32}).collect();

        js_buffer_audio(
            &self.js_ctx,
            0,
            &frame);
/*
        for x in 0..100 {
            let t = timestamp + (x as f64);
            let s = (t * 3.14159 * 2.0) as f32;
            self.add_sample(s.sin());
        }*/
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
