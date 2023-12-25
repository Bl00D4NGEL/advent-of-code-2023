use clap::Parser;
use day10::Day10;
use day11::Day11;
use day12::Day12;
use day13::Day13;
use day14::Day14;
use day15::Day15;
use day16::Day16;
use day17::Day17;
use day18::Day18;
use day19::Day19;
use day20::Day20;
use day21::Day21;
use day22::Day22;
use day23::Day23;
use day24::Day24;
use day25::Day25;
use solver::{Solvable, Solver};

mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;
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

    let solver: Box<dyn Solver<_, _>> = match args.day {
        10 => Box::new(Day10 {}),
        11 => Box::new(Day11 {}),
        12 => Box::new(Day12 {}),
        13 => Box::new(Day13 {}),
        14 => Box::new(Day14 {}),
        15 => Box::new(Day15 {}),
        16 => Box::new(Day16 {}),
        17 => Box::new(Day17 {}),
        18 => Box::new(Day18 {}),
        19 => Box::new(Day19 {}),
        20 => Box::new(Day20 {}),
        21 => Box::new(Day21 {}),
        22 => Box::new(Day22 {}),
        23 => Box::new(Day23 {}),
        24 => Box::new(Day24 {}),
        25 => Box::new(Day25 {}),
        day => {
            println!("Could not find solver for day {day} make sure it is implemented");

            return;
        }
    };

    let solvable = Solvable::new(args.path, solver);

    solvable.solve();
}
