use crate::activation::Activation;
use crate::cell::Cell;
use crate::config::Configuration;
use anyhow::Result;
use image::Rgb;
use std::fmt;
use std::str;

pub const BLACK: Rgb<u8> = Rgb([0, 0, 0]);
pub const WHITE: Rgb<u8> = Rgb([255, 255, 255]);
pub const RED: Rgb<u8> = Rgb([255, 0, 0]);
pub const GREEN: Rgb<u8> = Rgb([0, 255, 0]);
pub const BLUE: Rgb<u8> = Rgb([0, 0, 255]);
pub const ORANGE: Rgb<u8> = Rgb([255, 200, 0]);
pub const CYAN: Rgb<u8> = Rgb([0, 255, 255]);

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ColorScheme {
    BlackWhite,
    OmegaColor,
    ActivationColor,
}

/// Compute the color of the cell based on the black and white color scheme
fn no_color(cell: &Cell) -> Rgb<u8> {
    if cell.state {
        BLACK
    } else {
        WHITE
    }
}

/// Compute the color of the cell based on the omega value
fn omega_color(cell: &Cell, big_omega: f64) -> Rgb<u8> {
    let c = (cell.omega / big_omega * 255.0) as u8;
    if cell.state {
        Rgb([255 - c, 255 - c, 255 - c])
    } else {
        Rgb([255, 255, 255 - c])
    }
}

/// Compute the color of the cell based on the activation
fn activation_color(cell: &Cell) -> Rgb<u8> {
    match cell.activation {
        Activation::NewStateOff => WHITE,
        //     Activation::SelfRightOn | Activation::SelfLeftOn | Activation::AllOn => BLACK,
        //     Activation::LeftOn | Activation::RightOn | Activation::LeftRightOn => GREEN,
        //     Activation::SelfOn => RED,
        //     Activation::AllOff => BLUE,
        // AllOff,      // 000
        Activation::AllOff => Rgb([127, 127, 127]), // 000
        Activation::RightOn => Rgb([127, 127, 0]),  // 001
        Activation::SelfOn => Rgb([127, 0, 127]),   // 010
        Activation::SelfRightOn => Rgb([127, 0, 0]), // 011
        Activation::LeftOn => Rgb([0, 127, 127]),   // 100
        Activation::LeftRightOn => Rgb([0, 127, 0]), // 101
        Activation::SelfLeftOn => Rgb([0, 0, 127]), // 110
        Activation::AllOn => Rgb([0, 0, 0]),        // 111
    }
}

impl ColorScheme {
    pub const VALID_VALUES: [&'static str; 3] = ["BlackWhite", "Omega", "Activation"];

    /// Returns a comma-separated list of valid update pattern names
    pub fn valid_values() -> String {
        Self::VALID_VALUES.join(", ")
    }

    /// Get the color of the cell based on the color scheme
    pub fn get_color(&self, cell: &Cell, i: usize, config: &dyn Configuration) -> Rgb<u8> {
        match self {
            ColorScheme::BlackWhite => no_color(&cell),
            ColorScheme::OmegaColor => omega_color(&cell, config.get_big_omega(i)),
            ColorScheme::ActivationColor => activation_color(&cell),
        }
    }
}

impl str::FromStr for ColorScheme {
    type Err = anyhow::Error;

    /// Implement the FromStr trait for ColorScheme to parse the color scheme
    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "blackwhite" => Ok(ColorScheme::BlackWhite),
            "omega" => Ok(ColorScheme::OmegaColor),
            "activation" => Ok(ColorScheme::ActivationColor),
            _ => Err(anyhow::anyhow!(
                "Invalid color scheme: must be 'BlackWhite', 'Omega', or 'Activation'"
            )),
        }
    }
}

impl fmt::Display for ColorScheme {
    /// Implement the Display trait for ColorScheme to print the color scheme
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ColorScheme::BlackWhite => write!(f, "BlackWhite"),
            ColorScheme::OmegaColor => write!(f, "Omega"),
            ColorScheme::ActivationColor => write!(f, "Activation"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid() {
        assert!(matches!(
            "blackwhite".parse::<ColorScheme>(),
            Ok(ColorScheme::BlackWhite)
        ));
        assert!(matches!(
            "omega".parse::<ColorScheme>(),
            Ok(ColorScheme::OmegaColor)
        ));
        assert!(matches!(
            "activation".parse::<ColorScheme>(),
            Ok(ColorScheme::ActivationColor)
        ));
    }

    #[test]
    fn test_parse_invalid() {
        assert!("invalid".parse::<ColorScheme>().is_err());
        assert!("blackwhite123".parse::<ColorScheme>().is_err());
        assert!("omega_color".parse::<ColorScheme>().is_err());
    }

    #[test]
    fn test_display() {
        assert!(ColorScheme::BlackWhite.to_string() == "BlackWhite");
        assert!(ColorScheme::OmegaColor.to_string() == "Omega");
        assert!(ColorScheme::ActivationColor.to_string() == "Activation");
    }
    #[test]
    fn test_blackwhite_color_scheme() {
        let black_cell = Cell::new(0.0, true);
        let white_cell = Cell::new(0.0, false);

        assert!(no_color(&black_cell) == BLACK);
        assert!(no_color(&white_cell) == WHITE);
    }

    #[test]
    fn test_activation_color_scheme() {
        let new_state_off = Cell {
            state: true,
            activation: Activation::NewStateOff,
            omega: 0.0,
        };
        let all_on = Cell {
            state: true,
            activation: Activation::AllOn,
            omega: 0.0,
        };
        let left_on = Cell {
            state: true,
            activation: Activation::LeftOn,
            omega: 0.0,
        };
        let self_on = Cell {
            state: true,
            activation: Activation::SelfOn,
            omega: 0.0,
        };
        let all_off = Cell {
            state: true,
            activation: Activation::AllOff,
            omega: 0.0,
        };

        assert!(activation_color(&new_state_off) == WHITE);
        assert!(activation_color(&all_on) == BLACK);
        assert!(activation_color(&left_on) == GREEN);
        assert!(activation_color(&self_on) == RED);
        assert!(activation_color(&all_off) == BLUE);
    }
}
