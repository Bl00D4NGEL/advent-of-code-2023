use std::{error::Error, fmt::Display, fs, path::PathBuf};

#[derive(Debug)]
pub enum SolvingError {
    NotImplemented,
    Other(Box<dyn Error>),
}

impl Display for SolvingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SolvingError::NotImplemented => {
                write!(f, "Not implemented")
            }
            SolvingError::Other(err) => err.fmt(f),
        }
    }
}

impl Error for SolvingError {}

pub struct Solvable<T, E>
where
    T: Display,
    E: Error,
{
    path_to_file: PathBuf,
    solver: Box<dyn Solver<T, E>>,
}

pub trait Solver<T, E>
where
    T: Display,
    E: Error,
{
    fn part_1(&self, contents: &str) -> Result<T, E>;
    fn part_2(&self, contents: &str) -> Result<T, E>;
}

impl<T, E> Solvable<T, E>
where
    T: Display,
    E: Error,
{
    pub fn new(path_to_file: PathBuf, solver: Box<dyn Solver<T, E>>) -> Solvable<T, E> {
        Solvable {
            path_to_file,
            solver,
        }
    }

    pub fn solve(&self) {
        let contents = match fs::read_to_string(&self.path_to_file) {
            Err(error) => {
                println!(
                    "Could not read file {} because: {error}",
                    self.path_to_file.display()
                );
                return;
            }
            Ok(c) => c,
        };

        println!("Solving part 1...");
        match self.solver.part_1(contents.as_str()) {
            Ok(result) => {
                println!("The result for part 1 is: {result}")
            }
            Err(error) => {
                println!("An error occured when solving part 1: {:?}", error)
            }
        }

        println!("Solving part 2...");
        match self.solver.part_2(contents.as_str()) {
            Ok(result) => {
                println!("The result for part 2 is: {result}")
            }
            Err(error) => {
                println!("An error occured when solving part 2: {:?}", error)
            }
        }
    }
}
