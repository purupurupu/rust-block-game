use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

#[derive(Clone, Copy, PartialEq)]
pub enum Cell {
    Empty,
    Filled(u8),
}

#[wasm_bindgen]
pub struct Game {
    context: CanvasRenderingContext2d,
    canvas: HtmlCanvasElement,
    board: Vec<Vec<Cell>>,
    board_width: usize,
    board_height: usize,
    cell_size: f64,
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
        // デバッグ情報を出力
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

    pub fn test_fill(&mut self) {
        self.board[0][0] = Cell::Filled(0); // cyan
        self.board[1][1] = Cell::Filled(1); // yellow
        self.board[2][2] = Cell::Filled(2); // purple
        self.draw();
    }
}
