use crate::{day16::Day16, solver::Solvable};
use clap::Parser;

mod day16;
mod solver;
mod utils;

/// Solve an advent of code day. If desired, specify a part to solve
#[derive(Parser, Debug)]
struct Cli {
    #[arg(short, long)]
    /// The day to solve
    day: usize,
    #[arg(long)]
    /// The path to the input
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();

    let solver = match args.day {
        16 => Solvable::new(
            args.path,
            Some(Box::new(Day16 {})),
            Some(Box::new(Day16 {})),
        ),
        day => {
            println!("Could not find solver for day {day} make sure it is implemented");

            return;
        }
    };

    solver.solve();
}
