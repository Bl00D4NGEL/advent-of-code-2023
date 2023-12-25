use std::{fmt::Display, fs, path::PathBuf};

pub struct Solvable<T>
where
    T: Display,
{
    path_to_file: PathBuf,
    part_1_solver: Option<Box<dyn PartOneSolver<T>>>,
    part_2_solver: Option<Box<dyn PartTwoSolver<T>>>,
}

pub trait PartOneSolver<T>
where
    T: Display,
{
    fn solve(&self, contents: &str) -> Result<T, ()>;
}

pub trait PartTwoSolver<T>
where
    T: Display,
{
    fn solve(&self, contents: &str) -> Result<T, ()>;
}

impl<'a, T> Solvable<T>
where
    T: Display,
{
    pub fn new(
        path_to_file: PathBuf,
        part_1_solver: Option<Box<dyn PartOneSolver<T>>>,
        part_2_solver: Option<Box<dyn PartTwoSolver<T>>>,
    ) -> Solvable<T> {
        Solvable {
            path_to_file,
            part_1_solver,
            part_2_solver,
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
        match &self.part_1_solver {
            None => println!("Part 1 not implemented yet"),
            Some(solver) => match solver.solve(contents.as_str()) {
                Ok(result) => {
                    println!("The result for part 1 is: {result}")
                }
                Err(error) => {
                    println!("An error occured when solving part 1: {:?}", error)
                }
            },
        }

        println!("Solving part 2...");
        match &self.part_2_solver {
            None => println!("Part 2 not implemented yet"),
            Some(solver) => match solver.solve(contents.as_str()) {
                Ok(result) => {
                    println!("The result for part 2 is: {result}")
                }
                Err(error) => {
                    println!("An error occured when solving part 2: {:?}", error)
                }
            },
        }
    }
}
