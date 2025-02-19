use crate::activation::Activation;
use crate::boundaries::Boundaries;
use crate::rule::Rule;
use std::fmt;

#[derive(Clone, Copy, PartialEq)]
pub struct Cell {
    pub state: bool,
    pub rule: Rule,
    pub activation: Activation,
    pub omega: f64,
    pub big_omega: f64,
    pub alpha: f64,
}

fn neighbor_index(i: usize, time: usize) -> isize {
    let offset = if time % 2 == 1 { 1 } else { -1 };
    i as isize + offset
}

impl Cell {
    pub fn new(rule: Rule, alpha: f64, state: bool) -> Self {
        // Constructor with rule, alpha and state
        Self {
            state: state,
            rule: rule,
            activation: if state {
                Activation::BothOn
            } else {
                Activation::NewStateOff
            },
            omega: if state { 1.0 / (1.0 - alpha) } else { 0.0 },
            big_omega: 1.0 / (1.0 - alpha),
            alpha: alpha,
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
        let n = neighbor_index(i, time);
        let neighbor_state = boundaries.get_state(cells, n);
        let mut new_state = self.rule.compute(self.state, neighbor_state);
        let omega = (self.omega * self.alpha) + if new_state { 1.0 } else { 0.0 };
        let big_omega = self.big_omega;

        if omega != 0.5 {
            new_state = (omega / big_omega) > 0.5;
        }

        Self {
            state: new_state,
            rule: self.rule.clone(),
            activation: Activation::from_state(self.state, neighbor_state, new_state),
            omega: omega,
            big_omega: big_omega,
            alpha: self.alpha,
        }
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Implement the Display trait for Cell to print the state
        write!(f, "{}", if self.state { "1" } else { "0" })
    }
}
