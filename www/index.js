import init, { Game } from '../pkg/rust_block_game.js';

async function run() {
    await init();
    const game = new Game();
    // game.draw_test();
    // game.test_fill();
    game.spawn_test_mino();


    // キーボードイベントの処理
    document.addEventListener('keydown', (event) => {
        switch (event.key) {
            case 'ArrowLeft':
                game.move_left();
                break;
            case 'ArrowRight':
                game.move_right();
                break;
            case 'ArrowDown':
                game.move_down();
                break;
            case 'ArrowUp':    // 上キーで回転
                game.rotate();
                break;
        }
    });

}

run();