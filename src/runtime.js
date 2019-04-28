// This file is prepended to the Javascript produced by Cargo-Web

var canvas = document.getElementById('canvas');

function invoke_js_on_touch(canvas_x, canvas_y) {
    var width = canvas.offsetWidth, height = canvas.offsetHeight;
    if (width <= 0 || height <= 0) {
        return;
    }
    var gfx_x = canvas_x - canvas.offsetLeft;
    var gfx_y = canvas_y - canvas.offsetTop;
    Module.exports.js_ontouch(gfx_x, gfx_y, width, height);
}

// Handle taps:

canvas.addEventListener("touchstart", onTouch, false);
canvas.addEventListener("touchmove", onTouch, false);

function onTouch(ev) {
    var len = ev.changedTouches.length;
    for (var i = 0; i < len; i++) {
        var touch = ev.changedTouches.item(i);
        invoke_js_on_touch(touch.clientX, touch.clientY);
    }
}

canvas.addEventListener("pointerdown", onPointer, false);
canvas.addEventListener("pointermove", onPointer, false);

function onPointer(ev) {
    invoke_js_on_touch(ev.clientX, ev.clientY);
}

canvas.addEventListener("mousedown", onMouse, false);
canvas.addEventListener("mousemove", onMouse, false);

function onMouse(ev) {
    invoke_js_on_touch(ev.clientX, ev.clientY);
}
