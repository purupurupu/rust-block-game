use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

#[derive(Clone, Copy, PartialEq)]
pub enum Cell {
    Empty,
    Filled(u8),
}
// テトリミノの形状を定義
#[derive(Clone, Copy, PartialEq)]
pub enum BlockType {
    I,
    O,
    T,
    S,
    Z,
    J,
    L,
}

#[derive(Clone)]
pub struct BlockShape {
    block_type: BlockType,
    cells: Vec<Vec<bool>>, // trueなら埋まっているセル
    color: u8,
}

impl BlockShape {
    pub fn new(block_type: BlockType) -> Self {
        let (cells, color) = match block_type {
            BlockType::I => (
                vec![
                    vec![false, false, false, false],
                    vec![true, true, true, true],
                    vec![false, false, false, false],
                    vec![false, false, false, false],
                ],
                0, // cyan
            ),
            BlockType::O => (
                vec![vec![true, true], vec![true, true]],
                1, // yellow
            ),
            BlockType::T => (
                vec![
                    vec![false, true, false],
                    vec![true, true, true],
                    vec![false, false, false],
                ],
                2, // purple
            ),
            BlockType::S => (
                vec![
                    vec![false, true, true],
                    vec![true, true, false],
                    vec![false, false, false],
                ],
                5, // green
            ),
            BlockType::Z => (
                vec![
                    vec![true, true, false],
                    vec![false, true, true],
                    vec![false, false, false],
                ],
                6, // red
            ),
            BlockType::J => (
                vec![
                    vec![true, false, false],
                    vec![true, true, true],
                    vec![false, false, false],
                ],
                3, // blue
            ),
            BlockType::L => (
                vec![
                    vec![false, false, true],
                    vec![true, true, true],
                    vec![false, false, false],
                ],
                4, // orange
            ),
        };

        BlockShape {
            block_type,
            cells,
            color,
        }
    }

    // 時計回りに90度回転
    pub fn rotate(&mut self) {
        let n = self.cells.len();
        let mut rotated = vec![vec![false; n]; n];

        for i in 0..n {
            for j in 0..n {
                rotated[j][n - 1 - i] = self.cells[i][j];
            }
        }

        self.cells = rotated;
    }
}

#[wasm_bindgen]
pub struct Game {
    context: CanvasRenderingContext2d,
    canvas: HtmlCanvasElement,
    board: Vec<Vec<Cell>>,
    board_width: usize,
    board_height: usize,
    cell_size: f64,

    current_block: Option<BlockShape>,
    current_pos: (usize, usize), // (x, y)
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<Game, JsValue> {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id("game-canvas").unwrap();
        let canvas: HtmlCanvasElement = canvas
            .dyn_into::<HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();

        let canvas_width = 300.0;
        let canvas_height = 600.0;
        canvas.set_width(canvas_width as u32);
        canvas.set_height(canvas_height as u32);

        let context = canvas
            .get_context("2d")?
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();

        let board_width = 10;
        let board_height = 20;
        let cell_size = canvas_width / board_width as f64;

        let board = vec![vec![Cell::Empty; board_width]; board_height];

        Ok(Game {
            context,
            canvas,
            board,
            board_width,
            board_height,
            cell_size,
            current_block: None,
            current_pos: (0, 0),
        })
    }

    pub fn draw(&self) {
        // 背景を黒に
        self.context.set_fill_style_str("black");
        self.context.fill_rect(
            0.0,
            0.0,
            self.canvas.width() as f64,
            self.canvas.height() as f64,
        );

        // グリッドとセルを描画
        for (y, row) in self.board.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                match cell {
                    Cell::Empty => {
                        // 空のセルは灰色の枠線のみ
                        self.context.set_stroke_style_str("gray");
                        self.context.stroke_rect(
                            x as f64 * self.cell_size,
                            y as f64 * self.cell_size,
                            self.cell_size,
                            self.cell_size,
                        );
                    }
                    Cell::Filled(color) => {
                        // 塗りつぶされたセルは色付きで描画
                        let color_str = match color {
                            0 => "cyan",
                            1 => "yellow",
                            2 => "purple",
                            3 => "blue",
                            4 => "orange",
                            5 => "green",
                            6 => "red",
                            _ => "white",
                        };
                        self.context.set_fill_style_str(color_str);
                        self.context.fill_rect(
                            x as f64 * self.cell_size,
                            y as f64 * self.cell_size,
                            self.cell_size,
                            self.cell_size,
                        );
                    }
                }
            }
        }

        if let Some(block) = &self.current_block {
            for (i, row) in block.cells.iter().enumerate() {
                for (j, &is_filled) in row.iter().enumerate() {
                    if is_filled {
                        let x = self.current_pos.0 + j;
                        let y = self.current_pos.1 + i;

                        self.context.set_fill_style_str(match block.color {
                            0 => "cyan",
                            1 => "yellow",
                            2 => "purple",
                            3 => "blue",
                            4 => "orange",
                            5 => "green",
                            6 => "red",
                            _ => "white",
                        });

                        self.context.fill_rect(
                            x as f64 * self.cell_size,
                            y as f64 * self.cell_size,
                            self.cell_size,
                            self.cell_size,
                        );
                    }
                }
            }
        }

        // デバッグ用
        web_sys::console::log_1(
            &format!(
                "Debug Info:\nBoard: {}x{}\nCell Size: {}\nCanvas: {}x{}",
                self.board_width,
                self.board_height,
                self.cell_size,
                self.canvas.width(),
                self.canvas.height()
            )
            .into(),
        );
    }

    // 左に移動
    pub fn move_left(&mut self) {
        if let Some(_) = &self.current_block {
            if self.current_pos.0 > 0 {
                self.current_pos.0 -= 1;
                self.draw();
            }
        }
    }

    // 右に移動
    pub fn move_right(&mut self) {
        if let Some(block) = &self.current_block {
            if self.current_pos.0 + block.cells.len() < self.board_width {
                self.current_pos.0 += 1;
                self.draw();
            }
        }
    }

    // 下に移動
    pub fn move_down(&mut self) {
        if let Some(block) = &self.current_block {
            if self.current_pos.1 + block.cells.len() < self.board_height {
                self.current_pos.1 += 1;
                self.draw();
            }
        }
    }

    // ミノを回転
    pub fn rotate(&mut self) {
        if let Some(block) = &mut self.current_block {
            block.rotate();
            self.draw();
        }
    }

    // テスト用：ミノを生成して表示
    pub fn spawn_test_mino(&mut self) {
        let block = BlockShape::new(BlockType::T); // ミノをテスト用に生成
        self.current_block = Some(block);
        self.current_pos = (self.board_width / 2 - 2, 0); // 上端中央に配置
        self.draw();
    }

    pub fn test_fill(&mut self) {
        self.board[0][0] = Cell::Filled(0); // cyan
        self.board[1][1] = Cell::Filled(1); // yellow
        self.board[2][2] = Cell::Filled(2); // purple
        self.draw();
    }
}
