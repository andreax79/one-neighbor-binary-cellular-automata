use crate::color_scheme::{ColorScheme, BLUE, CYAN, ORANGE, RED, WHITE};
use crate::output::Output;
use crate::output::OutputType;
use crate::row::Row;
use image::{Rgb, RgbImage};
use std::error::Error;
use std::str;
use std::string::String;

pub const DEFAULT_CELL_SIZE: usize = 2;

pub struct ImageOutput {
    color_scheme: ColorScheme,
    cell_size: usize,
    filename: String,
    img: RgbImage,
    prev_value: f64,
    prev_delta: f64,
    prev_ones: usize,
    prev_ones_delta: usize,
    output_type: OutputType,
}

impl ImageOutput {
    pub fn new(
        filename: &str,
        size: usize,
        steps: usize,
        color_scheme: ColorScheme,
        output_type: OutputType,
    ) -> ImageOutput {
        let cell_size = match output_type {
            OutputType::TimeSpaceGraph(cell_size) => cell_size,
            OutputType::TimeSpace(cell_size) => cell_size,
            _ => DEFAULT_CELL_SIZE,
        };
        // Calculate the width and height of the image
        let width = if let OutputType::TimeSpaceGraph(_) = output_type {
            // Add space for the graphs
            ((cell_size * (size + 1)) as f64 * 1.25) as u32
        } else {
            (cell_size * (size + 1)) as u32
        };
        let height = (cell_size * (steps)) as u32;
        ImageOutput {
            color_scheme: color_scheme,
            cell_size: cell_size,
            filename: filename.to_string(),
            img: RgbImage::from_pixel(width, height, WHITE),
            prev_value: 0.0,
            prev_delta: 0.0,
            prev_ones: 0,
            prev_ones_delta: 0,
            output_type: output_type,
        }
    }

    fn draw_line(&mut self, (x0, y0): (u32, u32), (x1, y1): (u32, u32), color: Rgb<u8>) {
        // Draw a line on the image
        let (mut x0, mut y0) = (x0 as u32, y0 as u32);

        let dx = (x1 as i32 - x0 as i32).abs();
        let dy = (y1 as i32 - y0 as i32).abs();
        let mut err = dx - dy;

        loop {
            if x0 < self.img.width() && y0 < self.img.height() {
                self.img.put_pixel(x0, y0, color);
            }

            if x0 == x1 && y0 == y1 {
                break;
            }

            let e2 = 2 * err;
            if e2 > -dy {
                err -= dy;
                x0 = if x0 < x1 { x0 + 1 } else { x0 - 1 };
            }
            if e2 < dx {
                err += dx;
                y0 = if y0 < y1 { y0 + 1 } else { y0 - 1 };
            }
        }
    }
}

impl Output for ImageOutput {
    /// Draw the row on the image
    fn add_row(&mut self, row: &Row) {
        row.cells.iter().enumerate().for_each(|(j, cell)| {
            let color = self.color_scheme.get_color(&cell);
            for dx in 0..self.cell_size {
                for dy in 0..self.cell_size {
                    self.img.put_pixel(
                        (j * self.cell_size + dx) as u32,
                        (row.t * self.cell_size + dy) as u32,
                        color,
                    );
                }
            }
        });
        if let OutputType::TimeSpaceGraph(_) = self.output_type {
            let value: f64 = row.get_value();
            let delta: f64 = (value - self.prev_value).abs();
            let ones: usize = row.get_ones();
            let ones_delta: usize = (ones as isize - self.prev_ones as isize).abs() as usize;
            if row.t > 0 {
                let y0 = ((row.t - 1) * self.cell_size + self.cell_size / 2) as u32;
                let y1 = y0 + self.cell_size as u32;
                let delta_x = ((row.get_size() + 1) * self.cell_size) as u32;
                let cs = (self.cell_size + 1) as f64 / 4.0;
                // Draw the row value
                let x0 = (self.prev_value * cs) as u32 + delta_x;
                let x1 = (value * cs) as u32 + delta_x;
                self.draw_line((x0, y0), (x1, y1), RED);
                // Draw the number of cells in state on
                let x0 = (self.prev_ones as f64 * cs) as u32 + delta_x;
                let x1 = (ones as f64 * cs) as u32 + delta_x;
                self.draw_line((x0, y0), (x1, y1), BLUE);
                if row.t > 1 {
                    // Draw the row value delta
                    let x0 = (self.prev_delta * cs) as u32 + delta_x;
                    let x1 = (delta * cs) as u32 + delta_x;
                    self.draw_line((x0, y0), (x1, y1), ORANGE);
                    // Draw the number of cells in state on delta
                    let x0 = (self.prev_ones_delta as f64 * cs) as u32 + delta_x;
                    let x1 = (ones_delta as f64 * cs) as u32 + delta_x;
                    self.draw_line((x0, y0), (x1, y1), CYAN);
                }
            }
            self.prev_value = value;
            self.prev_delta = delta;
            self.prev_ones = ones;
            self.prev_ones_delta = ones_delta;
        }
    }

    /// Save the image
    fn close(&mut self) -> Result<(), Box<dyn Error>> {
        self.img.save(&self.filename)?;
        eprintln!("Image name: {}", self.filename);
        Ok(())
    }
}
