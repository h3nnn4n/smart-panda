/*jshint esversion: 6 */

import * as Rust from "smart-panda";
import {
    memory
} from "smart-panda/smart_panda_bg";
import * as Fps from "./js/fps.js";
import * as RandomPlayer from "./js/random_player.js";

// Lets use the js call from rust to js again just to make sure
// that everything (or at least this) is working
Rust.console_log("Starting");

var gamestate = Rust.get_new_gamestate();
gamestate.set_board_size(10, 20);

const width = gamestate.get_width();
const height = gamestate.get_height();

console.log('board size is: ', width, height);

const CELL_SIZE = 10; // px
const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";

const canvas = document.getElementById("game-of-life-canvas");
canvas.height = (CELL_SIZE + 1) * height + 2;
canvas.width = (CELL_SIZE + 1) * width + 2;

const ctx = canvas.getContext('2d');

const getIndex = (row, column) => {
    return column * height + row;
};

const drawGrid = () => {
    ctx.lineWidth = 1.0;
    ctx.beginPath();
    ctx.strokeStyle = GRID_COLOR;

    for (let i = 0; i <= width; i++) {
        ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
        ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1);
    }

    for (let j = 0; j <= height; j++) {
        ctx.moveTo(0, j * (CELL_SIZE + 1) + 1);
        ctx.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1);
    }

    ctx.stroke();
};

const drawCells = () => {
    const cellsPtr = gamestate.get_board_pointer();
    const cells = new Uint32Array(memory.buffer, cellsPtr, width * height);

    ctx.beginPath();

    for (let row = 0; row < height; row++) {
        for (let col = 0; col < width; col++) {
            const idx = getIndex(row, col);

            ctx.fillStyle = cells[idx] === 0 ?
                DEAD_COLOR :
                ALIVE_COLOR;

            ctx.fillRect(
                col * (CELL_SIZE + 1) + 1,
                row * (CELL_SIZE + 1) + 1,
                CELL_SIZE,
                CELL_SIZE
            );
        }
    }

    ctx.stroke();
};

const drawGameOver = () => {
    ctx.fillStyle = "White";
    ctx.fillRect(0, 0, canvas.width, canvas.height);

    ctx.fillStyle = "Gray";

    ctx.font = "18px Arial";
    ctx.textAlign = "center";

    ctx.fillText("Game Over", canvas.width / 2, canvas.height / 2);
};

const draw = () => {
    if (RandomPlayer.isGameOver()) {
        drawGameOver();
    } else {
        ctx.clearRect(0, 0, canvas.width, canvas.height);
        drawGrid();
        drawCells();
        // drawGameOver();
    }
};

const renderLoop = () => {
    Fps.fps.render();

    RandomPlayer.RandomAgent(gamestate);

    draw();

    requestAnimationFrame(renderLoop);
};

requestAnimationFrame(renderLoop);