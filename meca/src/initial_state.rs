use crate::cell::Cell;
use crate::rule::Rule;
use rand::prelude::*;
use std::fmt;

#[derive(Clone, PartialEq)]
pub enum InitialState {
    None,
    String(String),
    Bool(Vec<bool>),
}

/// Single seed
fn single_seed(size: usize) -> Vec<bool> {
    let mut state = vec![false; size];
    state[size / 2] = true;
    state
}

/// Single seed inverse
fn single_seed_inverse(size: usize) -> Vec<bool> {
    let mut state = vec![true; size];
    state[size / 2] = false;
    state
}

/// Random initial state
fn random(size: usize) -> Vec<bool> {
    let mut rng = rand::rng();
    (0..size).map(|_| rng.random::<f64>() < 0.5).collect()
}

/// Custom pattern
fn custom_pattern(pattern_str: &String, size: usize) -> Vec<bool> {
    let pattern: Vec<bool> = pattern_str.chars().map(|c| c == '1').collect();
    let mut state = vec![false; size];
    for i in 0..size {
        state[i] = pattern[i % pattern.len()];
    }
    state
}

impl InitialState {
    /// Parse the initial state from a string
    pub fn parse_initial_state(&self, size: usize) -> Vec<bool> {
        match self {
            InitialState::None => random(size),
            InitialState::String(pattern_str) => {
                // Parse pattern from string
                let pattern_str = pattern_str.trim().to_uppercase();
                if pattern_str == "S" {
                    // Single seed
                    single_seed(size)
                } else if pattern_str == "SI" {
                    // Single seed inverse
                    single_seed_inverse(size)
                } else if pattern_str == "RANDOM" {
                    // Random initial state
                    random(size)
                } else if !pattern_str.is_empty() {
                    // Custom pattern
                    custom_pattern(&pattern_str, size)
                } else {
                    vec![false; size]
                }
            }
            // Set pattern from bool array
            InitialState::Bool(p) => p.iter().cloned().collect(),
        }
    }

    pub fn prepare_initial_state(&self, size: usize, rule: Rule, alpha: f64) -> Vec<Cell> {
        let state = self.parse_initial_state(size);
        state.iter().map(|s| Cell::new(rule, alpha, *s)).collect()
    }
}

impl fmt::Display for InitialState {
    /// Implement the Display trait for InitialState to print the initial state
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InitialState::None => write!(f, "None"),
            InitialState::String(s) => write!(f, "{}", s),
            InitialState::Bool(p) => {
                for &b in p {
                    write!(f, "{}", if b { "1" } else { "0" })?;
                }
                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_initial_state_none() {
        let state = InitialState::None.parse_initial_state(10);
        assert!(state.len() == 10);
    }

    #[test]
    fn test_parse_initial_state_single_seed() {
        let state = InitialState::String("S".to_string()).parse_initial_state(9);
        assert!(state.len() == 9);
        assert!(state[4]);
        assert!(state.iter().filter(|&&x| x).count() == 1);
    }

    #[test]
    fn test_parse_initial_state_single_seed_inverse() {
        let state = InitialState::String("SI".to_string()).parse_initial_state(9);
        assert!(state.len() == 9);
        assert!(!state[4]);
        assert!(state.iter().filter(|&&x| !x).count() == 1);
    }

    #[test]
    fn test_parse_initial_state_custom_pattern() {
        let state = InitialState::String("101".to_string()).parse_initial_state(6);
        assert!(state.len() == 6);
        assert!(state == vec![true, false, true, true, false, true]);
    }

    #[test]
    fn test_parse_initial_state_bool_vec() {
        let input = vec![true, false, true, false, true];
        let state = InitialState::Bool(input.clone()).parse_initial_state(5);
        assert!(state == input);
    }

    #[test]
    fn test_display() {
        assert!(InitialState::None.to_string() == "None");
        assert!(InitialState::String("S".to_string()).to_string() == "S");
        assert!(InitialState::Bool(vec![true, false, true]).to_string() == "101");
    }
}
