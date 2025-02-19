use clap::{Arg, Command};
use meca::boundaries::Boundaries;
use meca::color_scheme::ColorScheme;
use meca::initial_state::InitialState;
use meca::output::OutputType;
use meca::rule::Rule;
use meca::update_pattern::UpdatePattern;
use std::error::Error;
use std::fmt;

pub struct Args {
    pub rule: Rule,
    pub size: usize,
    pub alpha: f64,
    pub boundaries: Boundaries,
    pub update_pattern: UpdatePattern,
    pub initial_state: InitialState,
    pub steps: usize,
    pub output_type: OutputType,
    pub color_scheme: ColorScheme,
    pub filename: String,
}

impl Args {
    pub fn parse() -> Result<Self, Box<dyn Error>> {
        let matches = Command::new("meca")
            .version("2.0")
            .author("Andrea Bonomi")
            .about("One Neighbor Binary Cellular Automata")
            .arg(
                Arg::new("rule")
                    .short('r')
                    .long("rule")
                    .value_parser(clap::value_parser!(u8))
                    .required(true)
                    .help("Rule number (0-15)"),
            )
            .arg(
                Arg::new("size")
                    .short('w')
                    .long("width")
                    .value_parser(clap::value_parser!(usize))
                    .default_value("256")
                    .help("Grid width"),
            )
            .arg(
                Arg::new("steps")
                    .short('s')
                    .long("steps")
                    .value_parser(clap::value_parser!(usize))
                    .default_value("500")
                    .help("Number of steps"),
            )
            .arg(
                Arg::new("alpha")
                    .short('a')
                    .long("alpha")
                    .value_parser(clap::value_parser!(f64))
                    .default_value("0.0")
                    .help("Memory factor (0.0-1.0)"),
            )
            .arg(
                Arg::new("initial_state")
                    .short('i')
                    .long("initial_state")
                    .value_parser(clap::value_parser!(String))
                    .default_value("S")
                    .help("Initial state (S, SI, RANDOM, or a custom pattern, e.g. 1010)"),
            )
            .arg(
                Arg::new("boundaries")
                    .short('b')
                    .long("boundaries")
                    .value_parser(clap::value_parser!(String))
                    .default_value("periodic")
                    .help(
                        "Boundary conditions (Periodic, FixedOn, FixedOff, Adiabatic, Reflective)",
                    ),
            )
            .arg(
                Arg::new("update")
                    .short('u')
                    .long("update")
                    .value_parser(clap::value_parser!(String))
                    .default_value("Synchronous")
                    .help(format!(
                        "Update pattern ({})",
                        UpdatePattern::valid_values()
                    )),
            )
            .arg(
                Arg::new("color")
                    .short('c')
                    .long("color")
                    .value_parser(clap::value_parser!(String))
                    .default_value("BlackWhite")
                    .help("Color scheme (BlackWhite, Omega, Activation)"),
            )
            .arg(
                Arg::new("output_type")
                    .short('o')
                    .long("output")
                    .value_parser(clap::value_parser!(String))
                    .default_value("TimeSpaceGraph")
                    .help("Output (TimeSpaceGraph, TimeSpace, Console, CSV, None)"),
            )
            .get_matches();
        // Parse the arguments
        let rule = Rule::new(*matches.get_one::<u8>("rule").unwrap())?;
        let size = *matches.get_one::<usize>("size").unwrap();
        let steps = *matches.get_one::<usize>("steps").unwrap();
        let alpha = *matches.get_one::<f64>("alpha").unwrap();
        if alpha < 0.0 || alpha > 1.0 {
            return Err("Alpha must be in the range 0.0-1.0".into());
        }
        let initial_state =
            InitialState::String(matches.get_one::<String>("initial_state").unwrap().clone());
        let update_pattern = matches
            .get_one::<String>("update")
            .unwrap()
            .parse::<UpdatePattern>()?;
        let color_scheme = matches
            .get_one::<String>("color")
            .unwrap()
            .parse::<ColorScheme>()?;
        let boundaries = matches
            .get_one::<String>("boundaries")
            .unwrap()
            .parse::<Boundaries>()?;
        let output_type = matches
            .get_one::<String>("output_type")
            .unwrap()
            .parse::<OutputType>()?;
        // Generate the filename
        let filename = format!(
            "rule_{}_w{}_s{}{}{}{}.png",
            rule.number,
            size,
            steps,
            // Add the initial state to the filename
            if initial_state == InitialState::String("S".to_string()) {
                "".to_string()
            } else {
                format!("_{}", initial_state)
            },
            // Add the alpha value to the filename
            if alpha != 0.0 {
                format!("_a{}", alpha)
            } else {
                "".to_string()
            },
            // Add the update pattern to the filename
            if update_pattern == UpdatePattern::Synchronous {
                "".to_string()
            } else {
                format!("_{}", update_pattern)
            }
        );

        Ok(Args {
            rule,
            size,
            alpha,
            boundaries,
            update_pattern,
            initial_state,
            steps,
            output_type,
            color_scheme,
            filename,
        })
    }
}

impl fmt::Display for Args {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Implement the Display trait for Args to print the arguments
        write!(
            f,
            "\
Rule: {rule}
Alpha: {alpha}
Size: {size}
Boundaries: {boundaries}
Initial_state: {initial_state}
Steps: {steps}
Update pattern: {update_pattern}
Color scheme: {color_scheme}
Output type: {output_type}
",
            rule = self.rule,
            alpha = self.alpha,
            size = self.size,
            boundaries = self.boundaries,
            initial_state = self.initial_state,
            steps = self.steps,
            update_pattern = self.update_pattern,
            color_scheme = self.color_scheme,
            output_type = self.output_type
        )
    }
}
