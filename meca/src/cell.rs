use crate::activation::Activation;
use crate::config::Configuration;
use std::fmt;

#[derive(Clone, Copy, PartialEq)]
pub struct Cell {
    pub state: bool,
    pub omega: f64,
    pub activation: Activation,
}

impl Cell {
    /// Create a new cell
    pub fn new(alpha: f64, state: bool) -> Self {
        Self {
            state: state,
            activation: if state {
                Activation::AllOn
            } else {
                Activation::NewStateOff
            },
            omega: if state { 1.0 / (1.0 - alpha) } else { 0.0 },
        }
    }

    /// Update the cell
    pub fn update(
        &self,
        cells: &Vec<Cell>,
        i: usize,
        time: usize,
        config: &dyn Configuration,
    ) -> Self {
        let (mut new_state, left_neighbor_state, right_neighbor_state) =
            config
                .get_rule(i)
                .step(cells, i, time, config.get_boundaries());

        let alpha = config.get_alpha(i);
        let omega = (self.omega * alpha) + if new_state { 1.0 } else { 0.0 };
        if omega != 0.5 {
            let big_omega = 1.0 / (1.0 - alpha);
            new_state = (omega / big_omega) > 0.5;
        }

        Self {
            state: new_state,
            omega: omega,
            activation: Activation::from_state(
                left_neighbor_state,
                self.state,
                right_neighbor_state,
                new_state,
            ),
        }
    }
}

impl fmt::Display for Cell {
    /// Implement the Display trait for Cell to print the state
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", if self.state { "1" } else { "0" })
    }
}
