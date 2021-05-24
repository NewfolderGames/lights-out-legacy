import * as wasm from "lights_out";

window.Game = new wasm.Game();
window.Game.load();

window.GameTickInterval = setInterval(() => window.Game.tick(), 200);
window.GameSaveInterval = setInterval(() => window.Game.save(), 300000);
