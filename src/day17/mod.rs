use crate::{
    solver::{Solver, SolvingError},
    utils::Grid,
};

pub struct Day17;

impl Solver<usize, SolvingError> for Day17 {
    fn part_1(&self, contents: &str) -> Result<usize, SolvingError> {
        let rows = contents
            .lines()
            .map(|line| line.chars().filter_map(|c| c.to_digit(10)).collect())
            .collect::<Vec<Vec<u32>>>();

        let grid = Grid::from(rows);

        println!("{}", grid);

        Err(SolvingError::NotImplemented)
    }

    fn part_2(&self, contents: &str) -> Result<usize, SolvingError> {
        Err(SolvingError::NotImplemented)
    }
}
