import * as wasm from "lights_out";

let game;
let gameInterval;

game = new wasm.Game();
game.load_assets();
game.load_save();
game.resume();

gameInterval = setInterval(() => game.tick(), 200);
