import * as wasm from "lights_out";

let game;
let gameInterval;

window.addEventListener("load", (event) => {

	game = new wasm.Game();
	game.load_assets();
	game.load_save();

	gameInterval = setInterval(() => game.tick(), 250);

});