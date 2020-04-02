import * as wasm from "life-rs";

const WIDTH = 30;
const HEIGHT = 25;

var board = wasm.WasmLife.new(WIDTH, HEIGHT);

document.getElementById('tick').onclick = function tick() {
    document.getElementById('grid').innerText = board.tick();
};
