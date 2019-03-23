import * as FeatureFunctions from "./feature_functions.js";
import * as GameScore from "./game_score.js";
import * as Fps from "./fps.js";
import {
    memory
} from "smart-panda/smart_panda_bg";

const CELL_SIZE = 10; // px
const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const ACTIVE_COLOR = "#BA946A";
const PLACED_COLOR = "#000000";

var gamestate;
var width;
var height;

const canvas = document.getElementById("tetris-canvas");
const ctx = canvas.getContext('2d');

const getIndex = (row, column) => {
    return column * height + row;
};

export function initCanvas(gamestate_) {
    gamestate = gamestate_;
    width = gamestate.get_width();
    height = gamestate.get_height();

    canvas.height = (CELL_SIZE + 1) * height + 2;
    canvas.width = (CELL_SIZE + 1) * width + 2;
}

function drawGrid() {
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
}

function drawCells() {
    const cellsPtr = gamestate.get_board_pointer();
    const cells = new Uint32Array(memory.buffer, cellsPtr, width * height);

    ctx.beginPath();

    for (let row = 0; row < height; row++) {
        for (let col = 0; col < width; col++) {
            const idx = getIndex(row, col);

            switch (cells[idx]) {
                case 0:
                    ctx.fillStyle = DEAD_COLOR;
                    break;
                default:
                    if (cells[idx] > 10) {
                        ctx.fillStyle = ACTIVE_COLOR;
                    } else {
                        ctx.fillStyle = PLACED_COLOR;
                    }
            }

            ctx.fillRect(
                col * (CELL_SIZE + 1) + 1,
                row * (CELL_SIZE + 1) + 1,
                CELL_SIZE,
                CELL_SIZE
            );
        }
    }

    ctx.stroke();
}

function drawGameOver() {
    ctx.fillStyle = "White";
    ctx.fillRect(0, 0, canvas.width, canvas.height);

    ctx.fillStyle = "Black";

    ctx.font = "18px Arial";
    ctx.textAlign = "center";

    ctx.fillText("Game Over", canvas.width / 2, canvas.height / 2);
}

export function draw() {
    if (gamestate.is_game_over()) {
        drawGameOver();
    } else {
        ctx.clearRect(0, 0, canvas.width, canvas.height);
        drawGrid();
        drawCells();
        Fps.fps.render();
        FeatureFunctions.drawFeatures(gamestate);
        GameScore.drawScore(gamestate);
    }
}