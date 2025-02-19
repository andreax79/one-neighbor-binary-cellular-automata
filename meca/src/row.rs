use crate::boundaries::Boundaries;
use crate::cell::Cell;
use crate::initial_state::InitialState;
use crate::rng::generate_eq_clocked_times;
use crate::rng::generate_random_order_indexes;
use crate::rule::Rule;
use crate::update_pattern::UpdatePattern;
use rand_core::RngCore;
use std::fmt;

#[derive(Clone, PartialEq)]
pub struct Row {
    pub cells: Vec<Cell>,
    pub t: usize,
    pub boundaries: Boundaries,
    pub update_pattern: UpdatePattern,
    pub update_order: Option<Vec<usize>>,
}

impl Row {
    pub fn new(
        size: usize,
        rule: Rule,
        alpha: f64,
        boundaries: Boundaries,
        update_pattern: UpdatePattern,
        initial_state: &InitialState,
        rng: &mut dyn RngCore,
    ) -> Self {
        Row {
            boundaries,
            update_pattern,
            t: 0,
            cells: initial_state.prepare_initial_state(size, rule, alpha, rng),
            update_order: match update_pattern {
                UpdatePattern::OasCyclic => Some(generate_random_order_indexes(size, rng)),
                UpdatePattern::OasEqClocked(t) => Some(generate_eq_clocked_times(size, t, rng)),
                _ => None,
            },
        }
    }

    /// Update the row of cells
    pub fn step(&self, rng: &mut dyn RngCore) -> Row {
        Row {
            cells: self.update_pattern.update(self, rng),
            t: self.t + 1,
            boundaries: self.boundaries,
            update_pattern: self.update_pattern,
            update_order: self.update_order.clone(),
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
    pub fn get_value(&self) -> f64 {
        self.cells.iter().map(|c| c.omega / c.big_omega).sum()
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
