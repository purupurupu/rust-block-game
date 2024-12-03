import init, { greet } from '../pkg/rust_block_game.js';

async function run() {
    await init();
    const result = greet("Rust");
    console.log(result);
}

run();