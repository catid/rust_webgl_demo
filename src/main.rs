#[macro_use]
extern crate stdweb;

fn main() {
    stdweb::initialize();

    let message = "Hello World";
    let result = js! {
        console.log( @{message} );
        return 2;
    };
    js! {
        console.log( @{result} );
        return 2;
    };

    stdweb::event_loop();
}
