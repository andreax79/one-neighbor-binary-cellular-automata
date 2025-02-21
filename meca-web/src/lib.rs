use meca::boundaries::Boundaries;
use meca::color_scheme::ColorScheme;
use meca::initial_state::InitialState;
use meca::row::Row;
use meca::rule::Rule;
use meca::stats::Stats;
use meca::update_pattern::UpdatePattern;
use rand_core::SeedableRng;
use rand_pcg::Lcg128Xsl64;
use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;
use web_sys::ImageData;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    Ok(())
}

#[wasm_bindgen]
pub fn draw(
    ctx: &CanvasRenderingContext2d,
    size: usize,
    steps: usize,
    rule_number: usize,
    boundaries: &str,
    initial_state: &str,
    update_pattern: &str,
    alpha: f64,
    color_scheme: &str,
) -> Result<(), JsValue> {
    let seed: u64 = 0;
    let mut rng = Lcg128Xsl64::seed_from_u64(seed);

    let rule = Rule::new(rule_number as u8)?;
    let boundaries = boundaries.parse::<Boundaries>()?;
    let update_pattern = update_pattern.parse::<UpdatePattern>()?;
    let initial_state = InitialState::String(initial_state.to_string());
    let color_scheme = color_scheme.parse::<ColorScheme>()?;
    let cell_size = 1;

    let mut stats = Stats::new();
    let mut row = Row::new(
        size,
        rule,
        alpha,
        boundaries,
        update_pattern,
        &initial_state,
        &mut rng,
    );

    let mut data = ImageData::new_with_sw(size as u32, steps as u32)?.data();

    for _ in 0..steps {
        // Calculate density
        stats.update(row.density());

        // Draw the row
        row.cells.iter().enumerate().for_each(|(j, cell)| {
            let color = color_scheme.get_color(&cell);
            let x = j * cell_size;
            let y = row.t * cell_size;
            let index = (y * size + x) * 4;
            data[index] = color[0]; // Red
            data[index + 1] = color[1]; // Green
            data[index + 2] = color[2]; // Blue
            data[index + 3] = 255; // Alpha
        });

        // Update the row
        row = row.step(&mut rng);
    }

    let image_data =
        ImageData::new_with_u8_clamped_array(wasm_bindgen::Clamped(&data), size as u32)?;
    ctx.put_image_data(&image_data, 0.0, 0.0)?;
    Ok(())
}

fn to_js_values(values: &[&str]) -> Vec<JsValue> {
    values
        .iter()
        .map(|&s| JsValue::from_str(s))
        .collect::<Vec<JsValue>>()
}

/// Get the boundaries
#[wasm_bindgen]
pub fn get_boundaries() -> Vec<JsValue> {
    to_js_values(&Boundaries::VALID_VALUES)
}

/// Get the initial states
#[wasm_bindgen]
pub fn get_initial_states() -> Vec<JsValue> {
    to_js_values(&InitialState::VALID_VALUES)
}

/// Get the update patterns
#[wasm_bindgen]
pub fn get_update_patterns() -> Vec<JsValue> {
    to_js_values(&UpdatePattern::VALID_VALUES)
}

/// Get the color schemes
#[wasm_bindgen]
pub fn get_color_schemes() -> Vec<JsValue> {
    to_js_values(&ColorScheme::VALID_VALUES)
}
