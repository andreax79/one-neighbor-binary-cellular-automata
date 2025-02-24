use crate::vue_state::VueState;
use crate::vue_stats::VueStats;
use anyhow;
use meca::config::Configuration;
use meca::row::Row;
use meca::stats::Stats;
use rand_core::SeedableRng;
use rand_pcg::Lcg128Xsl64;
use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;
use web_sys::HtmlCanvasElement;
use web_sys::ImageData;

mod vue_state;
mod vue_stats;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    Ok(())
}

fn anyhow_to_jsvalue(err: anyhow::Error) -> JsValue {
    JsValue::from_str(&err.to_string())
}

fn draw_image(
    canvas: &HtmlCanvasElement,
    image_data: &web_sys::ImageData,
    cell_size: usize,
) -> Result<(), JsValue> {
    let width = image_data.width();
    let height = image_data.height();

    // Create an offscreen canvas
    let offscreen_canvas = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .create_element("canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()?;

    offscreen_canvas.set_width(width);
    offscreen_canvas.set_height(height);

    let offscreen_ctx = offscreen_canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()?;

    // Draw the original image onto the offscreen canvas
    offscreen_ctx.put_image_data(image_data, 0.0, 0.0)?;

    // Set the main canvas size
    let canvas_width = width * cell_size as u32;
    let canvas_height = height * cell_size as u32;
    canvas.set_width(canvas_width);
    canvas.set_height(canvas_height);

    // Draw the scaled image on the main canvas
    let ctx = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()?;
    ctx.set_image_smoothing_enabled(false); // Apply nearest-neighbor scaling
    ctx.draw_image_with_html_canvas_element_and_dw_and_dh(
        &offscreen_canvas,
        0.0,
        0.0,
        canvas_width as f64,
        canvas_height as f64,
    )?;
    Ok(())
}

#[wasm_bindgen]
pub fn draw(canvas: &HtmlCanvasElement, state: JsValue) -> Result<JsValue, JsValue> {
    let vue_state: VueState = match serde_wasm_bindgen::from_value(state) {
        Ok(state) => state,
        Err(e) => return Err(JsValue::from(e)),
    };
    let mut rng = Lcg128Xsl64::seed_from_u64(vue_state.seed);
    let config = vue_state.get_config(&mut rng).map_err(anyhow_to_jsvalue)?;
    let color_scheme = vue_state.get_color_scheme().map_err(anyhow_to_jsvalue)?;
    let mut row = Row::new(config.prepare_initial_state(&mut rng));
    let mut data = match ImageData::new_with_sw((vue_state.size) as u32, (vue_state.steps) as u32) {
        Ok(img) => img.data(),
        Err(e) => return Err(JsValue::from(e)),
    };
    let mut stats = Stats::new();

    for _ in 0..vue_state.steps {
        // Update the stats
        stats.update(row.density());

        // Draw the row
        row.cells.iter().enumerate().for_each(|(x, cell)| {
            let color = color_scheme.get_color(&cell, x, &config);
            let index = (row.t * vue_state.size + x) * 4;
            data[index] = color[0]; // Red
            data[index + 1] = color[1]; // Green
            data[index + 2] = color[2]; // Blue
            data[index + 3] = 255; // Alpha
        });

        // Update the row
        row = row.step(&mut rng, &config);
    }

    // Draw the image on the canvas
    draw_image(
        &canvas,
        &ImageData::new_with_u8_clamped_array(wasm_bindgen::Clamped(&data), vue_state.size as u32)?,
        vue_state.cell_size,
    )?;

    Ok(serde_wasm_bindgen::to_value(&VueStats::from_stats(&stats)).unwrap())
}

/// Get the initial Vue state
#[wasm_bindgen]
pub fn get_initial_state() -> JsValue {
    serde_wasm_bindgen::to_value(&VueState::new()).unwrap()
}
