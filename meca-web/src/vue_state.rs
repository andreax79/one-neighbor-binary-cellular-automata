use meca::boundaries::Boundaries;
use meca::color_scheme::ColorScheme;
use meca::initial_state::InitialState;
use meca::rule::Rule;
use meca::update_pattern::UpdatePattern;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct VueState {
    pub rule: u8,
    pub steps: usize,
    pub size: usize,
    pub boundaries: String,
    pub update_pattern: String,
    pub update_pattern_number: usize,
    pub initial_state: String,
    pub custom_initial_state: String,
    pub alpha: f64,
    pub seed: u64,
    pub color_scheme: String,
    pub cell_size: usize,
    pub boundaries_values: Vec<String>,
    pub initial_state_values: Vec<String>,
    pub update_pattern_values: Vec<String>,
    pub color_scheme_values: Vec<String>,
}

impl VueState {
    /// Create a new VueState with default values
    pub fn new() -> Self {
        VueState {
            rule: 6,
            steps: 600,
            size: 800,
            boundaries: "Periodic".to_string(),
            update_pattern_number: 10,
            initial_state: "S".to_string(),
            custom_initial_state: "10".to_string(),
            update_pattern: "Synchronous".to_string(),
            alpha: 0.0,
            seed: 0,
            color_scheme: "BlackWhite".to_string(),
            cell_size: 1,
            boundaries_values: Boundaries::VALID_VALUES
                .iter()
                .map(|s| s.to_string())
                .collect(),
            initial_state_values: InitialState::VALID_VALUES
                .iter()
                .map(|s| s.to_string())
                .collect(),
            update_pattern_values: UpdatePattern::VALID_VALUES
                .iter()
                .map(|s| s.to_string())
                .collect(),
            color_scheme_values: ColorScheme::VALID_VALUES
                .iter()
                .map(|s| s.to_string())
                .collect(),
        }
    }

    /// Get the rule
    pub fn get_rule(&self) -> Result<Rule, &'static str> {
        Rule::new(self.rule)
    }

    /// Get the initial state
    pub fn get_initial_state(&self) -> Result<InitialState, String> {
        if self.initial_state == "Custom" {
            Ok(InitialState::String(self.custom_initial_state.to_string()))
        } else {
            Ok(InitialState::String(self.initial_state.to_string()))
        }
    }

    /// Get the color scheme
    pub fn get_color_scheme(&self) -> Result<ColorScheme, &'static str> {
        self.color_scheme.parse::<ColorScheme>()
    }

    /// Get the boundaries
    pub fn get_boundaries(&self) -> Result<Boundaries, &'static str> {
        self.boundaries.parse::<Boundaries>()
    }

    /// Get the update pattern
    pub fn get_update_pattern(&self) -> Result<UpdatePattern, String> {
        let update_pattern = if self.update_pattern.ends_with("<n>") {
            format!(
                "{}{}",
                &self.update_pattern[..self.update_pattern.len() - 3],
                self.update_pattern_number
            )
        } else {
            self.update_pattern.clone()
        };
        update_pattern.parse::<UpdatePattern>()
    }
}
