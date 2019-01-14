pub fn js_log(message: String) {
    js! {
        console.log(@{message});
    };
}
