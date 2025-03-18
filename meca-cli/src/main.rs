use crate::args::Args;
use meca::config::Configuration;
use meca::config::StdConfiguration;
use meca::row::Row;
use meca::stats::Stats;
use rand_core::SeedableRng;
use rand_pcg::Lcg128Xsl64;

pub mod args;

fn run_app() -> Result<(), Box<dyn std::error::Error>> {
    // Parse the command line arguments
    let args = Args::parse()?;
    if !args.quiet {
        eprint!("{}", &args);
    }
    let mut rng = Lcg128Xsl64::seed_from_u64(args.seed);
    let config = StdConfiguration::new(
        args.rule_type.get_rule(args.rule_number)?,
        args.size,
        args.alpha,
        args.boundaries,
        args.update_pattern,
        args.initial_state,
        args.seed,
        args.steps,
        &mut rng,
    );

    let mut output =
        args.output_type
            .open_output(&args.filename, args.size, args.steps, args.color_scheme);
    let mut stats = Stats::new();
    let mut row = Row::new(config.prepare_initial_state(&mut rng));

    for _ in 0..args.steps {
        // Calculate density
        stats.update(row.density());

        // Draw the row
        output.add_row(&row, &config);

        // Update the row
        let new_row = row.step(&mut rng, &config);
        row = new_row;
    }

    output.close()?;
    if !args.quiet {
        eprintln!("{}", stats);
    }

    Ok(())
}

fn main() {
    if let Err(e) = run_app() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
