use clap::{Arg, ArgAction, Command};
use meca::boundaries::Boundaries;
use meca::color_scheme::ColorScheme;
use meca::initial_state::InitialState;
use meca::output::OutputType;
use meca::rule::RuleType;
use meca::update_pattern::UpdatePattern;
use std::error::Error;
use std::fmt;
use std::time::SystemTime;

pub struct Args {
    pub rule_number: u8,               // Rule number
    pub rule_type: RuleType,           // Rule family (1nCA or ECA)
    pub size: usize,                   // Grid width
    pub alpha: f64,                    // Memory factor
    pub boundaries: Boundaries,        // Boundary conditions
    pub update_pattern: UpdatePattern, // Update pattern
    pub initial_state: InitialState,   // Initial state
    pub seed: u64,                     // Random seed
    pub steps: usize,                  // Number of steps
    pub output_type: OutputType,       // Output type
    pub color_scheme: ColorScheme,     // Output color scheme
    pub filename: String,              // Output filename
    pub quiet: bool,                   // Quiet mode
    pub is_random: bool,               // Has random elements
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
                    .help("Rule number"),
            )
            .arg(
                Arg::new("family")
                    .short('f')
                    .long("family")
                    .value_parser(clap::value_parser!(String))
                    .default_value("1nCA")
                    .help("Rule family (1nCA or ECA)"),
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
                Arg::new("seed")
                    .short('n')
                    .long("seed")
                    .value_parser(clap::value_parser!(u64))
                    .help("Random seed"),
            )
            .arg(
                Arg::new("boundaries")
                    .short('b')
                    .long("boundaries")
                    .value_parser(clap::value_parser!(String))
                    .default_value("periodic")
                    .help(format!(
                        "Boundary conditions ({})",
                        Boundaries::valid_values()
                    )),
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
                    .help(format!("Color scheme ({})", ColorScheme::valid_values())),
            )
            .arg(
                Arg::new("output_type")
                    .short('o')
                    .long("output")
                    .value_parser(clap::value_parser!(String))
                    .default_value("TimeSpaceGraph")
                    .help(format!("Output type ({})", OutputType::valid_values())),
            )
            .arg(
                Arg::new("quiet")
                    .short('q')
                    .long("quiet")
                    .action(ArgAction::SetTrue)
                    .help("Quiet mode"),
            )
            .get_matches();
        // Parse the arguments
        let rule_number = *matches.get_one::<u8>("rule").unwrap();
        let rule_type = matches
            .get_one::<String>("family")
            .unwrap()
            .parse::<RuleType>()?;
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
        let seed = matches.get_one::<u64>("seed").cloned().unwrap_or_else(|| {
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs()
        });
        let quiet = *matches.get_one::<bool>("quiet").unwrap_or(&false);
        let is_random = initial_state.is_random() || update_pattern.is_random();

        // Generate the filename
        let filename = format!(
            "rule_{}_w{}_s{}{}{}{}{}.png",
            rule_number,
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
            },
            // Add the random seed to the filename
            if is_random {
                format!("_r{}", seed)
            } else {
                "".to_string()
            }
        );

        Ok(Args {
            rule_number,
            rule_type,
            size,
            alpha,
            boundaries,
            update_pattern,
            initial_state,
            seed,
            steps,
            output_type,
            color_scheme,
            filename,
            quiet,
            is_random,
        })
    }
}

impl fmt::Display for Args {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Implement the Display trait for Args to print the arguments
        write!(
            f,
            "\
Rule: {rule_type} {rule_number}
Alpha: {alpha}
Size: {size}
Boundaries: {boundaries}
Initial state: {initial_state}
Seed: {seed}
Steps: {steps}
Update pattern: {update_pattern}
Color scheme: {color_scheme}
Output type: {output_type}
",
            rule_number = self.rule_number,
            rule_type = self.rule_type,
            alpha = self.alpha,
            size = self.size,
            boundaries = self.boundaries,
            initial_state = self.initial_state,
            seed = if self.is_random {
                format!("{}", self.seed)
            } else {
                "-".to_string()
            },
            steps = self.steps,
            update_pattern = self.update_pattern,
            color_scheme = self.color_scheme,
            output_type = self.output_type
        )
    }
}
