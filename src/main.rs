#[macro_use]
extern crate stdweb;
#[macro_use]
extern crate log;
extern crate console_log;

fn main() {
    stdweb::initialize();
    console_log::init_with_level(::log::Level::Debug);

    info!("Hello world!");

    stdweb::event_loop();
}
