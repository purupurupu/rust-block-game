use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

#[wasm_bindgen]
// pub fn greet(name: &str) -> String {
//     format!("Hello, {}!", name)
// }
pub struct Game {
    context: CanvasRenderingContext2d,
    canvas: HtmlCanvasElement,
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<Game, JsValue> {
        // canvas要素を取得
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id("game-canvas").unwrap();
        let canvas: HtmlCanvasElement = canvas
            .dyn_into::<HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();

        // キャンバスのサイズを設定
        canvas.set_width(300);
        canvas.set_height(600);

        // 描画コンテキストを取得
        let context = canvas
            .get_context("2d")?
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()
            .unwrap();

        Ok(Game { context, canvas })
    }

    // テスト描画用のメソッド
    pub fn draw_test(&self) {
        // 背景を黒に設定
        self.context.set_fill_style_str("black");
        self.context.fill_rect(0.0, 0.0, 300.0, 600.0);

        // テスト用の四角形を描画（青色）
        self.context.set_fill_style_str("blue");
        self.context.fill_rect(100.0, 100.0, 30.0, 30.0);

        // canvas の幅を再取得して使用
        let width = self.canvas.width();
        self.context
            .fill_text(&format!("Width: {}", width), 10.0, 20.0)
            .expect("Failed to fill text");
    }
}
