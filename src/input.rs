use tools::js_log;

pub struct InputState {
    // Nothing here yet.
}

#[js_export]
fn js_ontouch(x: i32, y: i32, w: i32, h: i32) {
    let norm_x = x as f32 / w as f32;
    let norm_y = y as f32 / h as f32;
    // FIXME: Convert x, y to normalized coordinates between -1..1
    js_log(format!("Tap at {}, {}", norm_x, norm_y));
}

impl InputState {
    pub fn new() -> InputState {
        InputState {
        }
    }

    pub fn CheckFingerTips(&mut self, _nowSeconds: f64) {

    }
}
//#[cfg(feature="jsexports")]
