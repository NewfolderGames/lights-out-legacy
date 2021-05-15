import * as wasm from "lights_out";

let game;
let gameInterval;

window.addEventListener("load", (event) => {

	game = new wasm.Game();
	gameInterval = setInterval(() => game.tick(), 250);

});