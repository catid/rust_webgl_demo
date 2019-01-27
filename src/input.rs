pub struct InputState {
    // Nothing here yet.
}

#[cfg(feature="jsexports")]
#[js_export]
fn js_on_tap(x: i32, y: i32) {
    js_log(format!("Tap at {}, {}", x, y));
}

impl InputState {
    pub fn new() -> InputState {
        InputState {
        }
    }
}
