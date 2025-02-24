use crate::cell::Cell;
use crate::config::Configuration;
use rand_core::RngCore;
use std::fmt;

#[derive(Clone, PartialEq)]
pub struct Row {
    /// The cells in the row
    pub cells: Vec<Cell>,
    /// The time step
    pub t: usize,
}

impl Row {
    pub fn new(cells: Vec<Cell>) -> Self {
        Row { cells: cells, t: 0 }
    }

    /// Update the row of cells
    pub fn step(&self, rng: &mut dyn RngCore, config: &dyn Configuration) -> Row {
        Row {
            cells: config.get_update_pattern().update(self, rng, config),
            t: self.t + 1,
        }
    }

    /// Calculate the density
    pub fn density(&self) -> f64 {
        self.cells.iter().filter(|&&x| x.state).count() as f64 / self.cells.len() as f64
    }

    /// Get the number of cells
    pub fn get_size(&self) -> usize {
        self.cells.len()
    }

    /// Compute the value of the row as the sum of omega / big_omega
    pub fn get_value(&self, config: &dyn Configuration) -> f64 {
        self.cells
            .iter()
            .enumerate()
            .map(|(j, c)| c.omega / config.get_big_omega(j))
            .sum()
    }

    /// Get the number of cells in state on
    pub fn get_ones(&self) -> usize {
        self.cells.iter().filter(|&c| c.state).count()
    }
}

impl fmt::Display for Row {
    /// Implement the Display trait for Row to print the row
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for cell in &self.cells {
            write!(f, "{}", cell)?;
        }
        Ok(())
    }
}
