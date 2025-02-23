use crate::activation::Activation;
use crate::boundaries::Boundaries;
use crate::rule::Rule;
use std::fmt;

#[derive(Clone, Copy, PartialEq)]
pub struct Cell {
    pub rule: Rule,
    pub alpha: f64,
    pub big_omega: f64, // 1 / (1 - alpha)
    pub state: bool,
    pub omega: f64,
    pub activation: Activation,
}

impl Cell {
    pub fn new(rule: Rule, alpha: f64, state: bool) -> Self {
        // Constructor with rule, alpha and state
        Self {
            rule: rule,
            alpha: alpha,
            big_omega: 1.0 / (1.0 - alpha),
            state: state,
            activation: if state {
                Activation::AllOn
            } else {
                Activation::NewStateOff
            },
            omega: if state { 1.0 / (1.0 - alpha) } else { 0.0 },
        }
    }

    pub fn update(
        &self,
        cells: &Vec<Cell>,
        i: usize,
        time: usize,
        boundaries: &Boundaries,
    ) -> Self {
        // Update the cell
        let (mut new_state, left_neighbor_state, right_neighbor_state) =
            self.rule.step(cells, i, time, boundaries);

        let omega = (self.omega * self.alpha) + if new_state { 1.0 } else { 0.0 };
        if omega != 0.5 {
            new_state = (omega / self.big_omega) > 0.5;
        }

        Self {
            rule: self.rule.clone(),
            alpha: self.alpha,
            big_omega: self.big_omega,
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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Implement the Display trait for Cell to print the state
        write!(f, "{}", if self.state { "1" } else { "0" })
    }
}
