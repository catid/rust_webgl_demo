// This file is prepended to the Javascript produced by Cargo-Web

var canvas = document.getElementById('canvas');

// Handle taps:

canvas.addEventListener("touchstart", onTouch, false);
canvas.addEventListener("touchmove", onTouch, false);

function onTouch(ev) {
    var len = ev.changedTouches.length;
    for (var i = 0; i < len; i++) {
        var touch = ev.changedTouches.item(i);
        Module.exports.js_on_tap(
            touch.clientX - canvas.offsetTop,
            touch.clientY - canvas.offsetLeft);
    }
}

canvas.addEventListener("pointerdown", onPointer, false);
canvas.addEventListener("pointermove", onPointer, false);

function onPointer(ev) {
    Module.exports.js_on_tap(
        ev.clientX - canvas.offsetTop,
        ev.clientY - canvas.offsetLeft);
}

canvas.addEventListener("mousedown", onMouse, false);
canvas.addEventListener("mousemove", onMouse, false);

function onMouse(ev) {
    Module.exports.js_on_tap(ev.clientX, ev.clientY);
}
