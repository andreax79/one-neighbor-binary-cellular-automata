use crate::vue_state::VueState;
use crate::vue_stats::VueStats;
use meca::row::Row;
use meca::stats::Stats;
use rand_core::SeedableRng;
use rand_pcg::Lcg128Xsl64;
use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;
use web_sys::ImageData;

mod vue_state;
mod vue_stats;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    Ok(())
}

#[wasm_bindgen]
pub fn draw(ctx: &CanvasRenderingContext2d, state: JsValue) -> Result<JsValue, JsValue> {
    let state: VueState = serde_wasm_bindgen::from_value(state).unwrap();
    let mut rng = Lcg128Xsl64::seed_from_u64(state.seed);
    let color_scheme = state.get_color_scheme()?;

    let mut stats = Stats::new();
    let mut row = Row::new(
        state.size,
        state.get_rule()?,
        state.alpha,
        state.get_boundaries()?,
        state.get_update_pattern()?,
        &state.get_initial_state()?,
        &mut rng,
    );

    let mut data = ImageData::new_with_sw(state.size as u32, state.steps as u32)?.data();

    for _ in 0..state.steps {
        // Calculate density
        stats.update(row.density());

        // Draw the row
        row.cells.iter().enumerate().for_each(|(j, cell)| {
            let color = color_scheme.get_color(&cell);
            let x = j * state.cell_size;
            let y = row.t * state.cell_size;
            let index = (y * state.size + x) * 4;
            data[index] = color[0]; // Red
            data[index + 1] = color[1]; // Green
            data[index + 2] = color[2]; // Blue
            data[index + 3] = 255; // Alpha
        });

        // Update the row
        row = row.step(&mut rng);
    }

    // Draw the image
    ctx.put_image_data(
        &&ImageData::new_with_u8_clamped_array(wasm_bindgen::Clamped(&data), state.size as u32)?,
        0.0,
        0.0,
    )?;
    Ok(serde_wasm_bindgen::to_value(&VueStats::from_stats(&stats)).unwrap())
}

/// Get the initial Vue state
#[wasm_bindgen]
pub fn get_initial_state() -> JsValue {
    serde_wasm_bindgen::to_value(&VueState::new()).unwrap()
}
