mod logic;
use wasm_bindgen::Clamped;
use wasm_bindgen::prelude::*;
use web_sys::ImageData;

#[wasm_bindgen]
pub fn draw_mandelbrot_set() {
    const CANVAS_ID: &str = "canvas_wasm";
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas: web_sys::Element = document.get_element_by_id(CANVAS_ID).unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();
    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();
    let canvas_w = canvas.width() as usize;
    let canvas_h = canvas.height() as usize;
    const X_MIN: f64 = -1.5;
    const X_MAX: f64 = 0.5;
    const Y_MIN: f64 = -1.0;
    const Y_MAX: f64 = 1.0;
    const MAX_ITER: usize = 64;

    let result =
        logic::generate_mandelbrot_set(canvas_w, canvas_h, X_MIN, X_MAX, Y_MIN, Y_MAX, MAX_ITER);
    let data = ImageData::new_with_u8_clamped_array_and_sh(
        Clamped(&result),
        canvas.width(),
        canvas.height(),
    );
    if let Ok(data) = data {
        let _ = context.put_image_data(&data, 0.0, 0.0);
    }
}
