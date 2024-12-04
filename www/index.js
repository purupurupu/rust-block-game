import init, { Game } from '../pkg/rust_block_game.js';

async function run() {
    await init();
    const game = new Game();
    // game.draw_test();
    game.test_fill();
}

run();