var wasm_module;

(() => import( /* webpackChunkName: "smart_panda_bg" */ '../pkg/smart_panda_bg').then(module => {
    wasm_module = module;
}))();

var active_piece;
var board_pointer;
var board_data;

const build_memory_buffer = (cellsPtr, length) => {
    return new Uint32Array(wasm_module.memory.buffer, cellsPtr, length);
};

const get_board_pointer = (gamestate) => {
    const cellsPtr = gamestate.get_board_pointer();
    const width = gamestate.get_width();
    const height = gamestate.get_height();

    return build_memory_buffer(cellsPtr, width * height);
};

export function store_board(gamestate) {
    board_pointer = get_board_pointer(gamestate);
    board_data = new Uint32Array(board_pointer);
    active_piece = gamestate.get_active_piece();
}

export function load_board(gamestate) {
    set_board(board_pointer, board_data);
    gamestate.build_active_piece(
        active_piece.get_id(),
        active_piece.get_x(),
        active_piece.get_y(),
        active_piece.get_rotation()
    );

    store_board(gamestate);
}

const set_board = (board_pointer, board_data) => {
    for (let index = 0; index < board_pointer.length; index++) {
        board_pointer[index] = board_data[index];
    }
};