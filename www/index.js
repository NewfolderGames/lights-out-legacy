import * as wasm from "lights_out";

window.game = new wasm.Game();
window.game.load();
window.game.load_save();
window.game.resume();

window.gameInterval = setInterval(() => game.tick(), 200);
