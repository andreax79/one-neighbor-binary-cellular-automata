use crate::boundaries::Boundaries;
use crate::cell::Cell;
use crate::initial_state::InitialState;
use crate::rng::generate_eq_clocked_times;
use crate::rng::generate_random_order_indexes;
use crate::rule::Rule;
use crate::update_pattern::UpdatePattern;
use rand_core::RngCore;

pub trait Configuration {
    /// Get the rule
    fn get_rule(&self, i: usize) -> &dyn Rule;
    /// Get the number of cells
    fn get_size(&self) -> usize;
    /// Get the alpha
    fn get_alpha(&self, i: usize) -> f64;
    /// Get the big omega
    fn get_big_omega(&self, i: usize) -> f64;
    /// Get boundaries
    fn get_boundaries(&self) -> &Boundaries;
    /// Get the update pattern
    fn get_update_pattern(&self) -> &UpdatePattern;
    /// Get the initial state
    fn get_initial_state(&self) -> &InitialState;
    /// Get the seed
    fn get_seed(&self) -> u64;
    /// Get the number of steps
    fn get_steps(&self) -> usize;
    /// Get the update order
    fn get_update_order(&self) -> Option<Vec<usize>>;
    /// Prepare the initial state
    fn prepare_initial_state(&self, rng: &mut dyn RngCore) -> Vec<Cell>;
}

pub struct StdConfiguration {
    rule: Box<dyn Rule>,
    size: usize,
    alpha: f64,
    boundaries: Boundaries,
    update_pattern: UpdatePattern,
    initial_state: InitialState,
    seed: u64,
    steps: usize,
    update_order: Option<Vec<usize>>,
}

impl StdConfiguration {
    pub fn new(
        rule: Box<dyn Rule>,
        size: usize,
        alpha: f64,
        boundaries: Boundaries,
        update_pattern: UpdatePattern,
        initial_state: InitialState,
        seed: u64,
        steps: usize,
        rng: &mut dyn RngCore,
    ) -> StdConfiguration {
        let update_order = match update_pattern {
            UpdatePattern::OasCyclic => Some(generate_random_order_indexes(size, rng)),
            UpdatePattern::OasEqClocked(t) => Some(generate_eq_clocked_times(size, t, rng)),
            _ => None,
        };
        StdConfiguration {
            rule,
            alpha,
            boundaries,
            update_pattern,
            initial_state,
            seed,
            steps,
            size,
            update_order,
        }
    }
}

impl Configuration for StdConfiguration {
    /// Get the rule
    fn get_rule(&self, _: usize) -> &dyn Rule {
        self.rule.as_ref()
    }
    /// Get the number of cells
    fn get_size(&self) -> usize {
        self.size
    }
    /// Get the alpha
    fn get_alpha(&self, _: usize) -> f64 {
        self.alpha
    }
    /// Get the big omega
    fn get_big_omega(&self, i: usize) -> f64 {
        1.0 / (1.0 - self.get_alpha(i))
    }
    /// Get boundaries
    fn get_boundaries(&self) -> &Boundaries {
        &self.boundaries
    }
    /// Get the update pattern
    fn get_update_pattern(&self) -> &UpdatePattern {
        &self.update_pattern
    }
    /// Get the initial state
    fn get_initial_state(&self) -> &InitialState {
        &self.initial_state
    }
    /// Get the seed
    fn get_seed(&self) -> u64 {
        self.seed
    }
    /// Get the number of steps
    fn get_steps(&self) -> usize {
        self.steps
    }
    /// Get the update order
    fn get_update_order(&self) -> Option<Vec<usize>> {
        self.update_order.clone()
    }
    /// Prepare the initial state
    fn prepare_initial_state(&self, rng: &mut dyn RngCore) -> Vec<Cell> {
        self.initial_state
            .prepare_initial_state(self.size, self.alpha, rng)
    }
}
