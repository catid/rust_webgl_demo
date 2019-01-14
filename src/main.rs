#![recursion_limit="256"]

#[macro_use]
extern crate stdweb;
extern crate nalgebra_glm as glm;
extern crate sample;

mod input;
mod graphics;
mod audio;
mod tools;

use std::cell::RefCell;
use std::rc::Rc;

struct GameLoop {
    inst_graphics: graphics::GraphicsState,
    inst_audio: audio::AudioState,
    inst_input: input::InputState,
}

impl GameLoop {
    fn new() -> GameLoop {
        GameLoop {
            inst_audio: audio::AudioState::new(),
            inst_graphics: graphics::GraphicsState::new(),
            inst_input: input::InputState::new(),
        }
    }
}

fn render_loop(looper: Rc<RefCell<GameLoop>>) {
    stdweb::web::window().request_animation_frame(move |timestamp: f64| {
        {
            let mut mlooper = looper.borrow_mut();
            mlooper.inst_graphics.update(timestamp);
            mlooper.inst_audio.update(timestamp);
            mlooper.inst_input.update(timestamp);
        }
        render_loop(looper);
    });
}

fn main() {
    stdweb::initialize();

    let looper = Rc::new(RefCell::new(GameLoop::new()));

    render_loop(looper);

    stdweb::event_loop();
}
