pub struct Day16;

use crate::solver::{PartOneSolver, PartTwoSolver};
use crate::utils::{Direction, Grid, Position};

impl PartOneSolver<usize> for Day16 {
    fn solve(&self, contents: &str) -> Result<usize, ()> {
        let rows = contents
            .lines()
            .rev()
            .map(|line| line.chars().map(FieldType::from).collect())
            .collect::<Vec<Vec<FieldType>>>();

        let grid = Grid::from(rows);

        let current_position = Position::new(0, grid.row_length() - 1);
        let direction = Direction::East;
        let mut energized = vec![];

        move_in_grid(&grid, direction, current_position, &mut energized);

        let mut positions = energized
            .iter()
            .map(|(_, pos)| pos)
            .collect::<Vec<&Position>>();
        positions.sort();
        positions.dedup();

        Ok(positions.len())
    }
}

impl PartTwoSolver<usize> for Day16 {
    fn solve(&self, contents: &str) -> Result<usize, ()> {
        let rows = contents
            .lines()
            .rev()
            .map(|line| line.chars().map(FieldType::from).collect())
            .collect::<Vec<Vec<FieldType>>>();

        let grid = Grid::from(rows);

        let mut configurations = vec![];
        let row_len = grid.row_length();
        let col_len = grid.col_length().unwrap_or(0);
        for y in 0..row_len {
            for x in 0..col_len {
                if x == 0 {
                    configurations.push((Direction::East, Position { x, y }));
                }
                if x == col_len - 1 {
                    configurations.push((Direction::West, Position { x, y }));
                }
                if y == 0 {
                    configurations.push((Direction::North, Position { x, y }));
                }
                if y == col_len - 1 {
                    configurations.push((Direction::South, Position { x, y }));
                }
            }
        }

        let configurations_count = configurations.len();
        Ok(configurations
            .into_iter()
            .enumerate()
            .map(|(idx, (direction, position))| {
                if idx % 10 == 0 {
                    println!("{idx} / {configurations_count}");
                }
                let mut energized = vec![];

                move_in_grid(&grid, direction, position, &mut energized);

                let mut positions = energized
                    .iter()
                    .map(|(_, pos)| pos)
                    .collect::<Vec<&Position>>();
                positions.sort();
                positions.dedup();

                positions.len()
            })
            .max()
            .unwrap())
    }
}

fn move_in_grid(
    grid: &Grid<FieldType>,
    direction: Direction,
    current_position: Position,
    energized: &mut Vec<(Direction, Position)>,
) {
    let current_field = grid.get_field(current_position);
    if current_field.is_none() {
        return;
    }

    if energized.contains(&(direction, current_position)) {
        return;
    }

    energized.push((direction, current_position));

    let current_field_value = current_field.unwrap().value;

    match (direction, current_field_value) {
        (Direction::East, FieldType::VerticalSplitter)
        | (Direction::West, FieldType::VerticalSplitter) => {
            if let Some(north) = current_position.move_to(Direction::North) {
                move_in_grid(grid, Direction::North, north, energized);
            }

            if let Some(x) = current_position.move_to(Direction::South) {
                move_in_grid(grid, Direction::South, x, energized);
            }
        }
        (Direction::South, FieldType::HorizontalSplitter)
        | (Direction::North, FieldType::HorizontalSplitter) => {
            if let Some(x) = current_position.move_to(Direction::East) {
                move_in_grid(grid, Direction::East, x, energized);
            }
            if let Some(x) = current_position.move_to(Direction::West) {
                move_in_grid(grid, Direction::West, x, energized);
            }
        }
        (Direction::East, FieldType::MirrorTopLeftDownRight)
        | (Direction::West, FieldType::MirrorTopRightDownLeft) => {
            if let Some(x) = current_position.move_to(Direction::South) {
                move_in_grid(grid, Direction::South, x, energized);
            }
        }
        (Direction::North, FieldType::MirrorTopLeftDownRight)
        | (Direction::South, FieldType::MirrorTopRightDownLeft) => {
            if let Some(x) = current_position.move_to(Direction::West) {
                move_in_grid(grid, Direction::West, x, energized);
            }
        }
        (Direction::East, FieldType::MirrorTopRightDownLeft)
        | (Direction::West, FieldType::MirrorTopLeftDownRight) => {
            if let Some(x) = current_position.move_to(Direction::North) {
                move_in_grid(grid, Direction::North, x, energized);
            }
        }
        (Direction::North, FieldType::MirrorTopRightDownLeft)
        | (Direction::South, FieldType::MirrorTopLeftDownRight) => {
            if let Some(x) = current_position.move_to(Direction::East) {
                move_in_grid(grid, Direction::East, x, energized);
            }
        }
        _ => {
            if let Some(x) = current_position.move_to(direction) {
                move_in_grid(grid, direction, x, energized);
            }
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
enum FieldType {
    MirrorTopRightDownLeft,
    MirrorTopLeftDownRight,
    HorizontalSplitter,
    VerticalSplitter,
    Empty,
}

impl From<char> for FieldType {
    fn from(value: char) -> Self {
        match value {
            '/' => FieldType::MirrorTopRightDownLeft,
            '\\' => FieldType::MirrorTopLeftDownRight,
            '|' => FieldType::VerticalSplitter,
            '-' => FieldType::HorizontalSplitter,
            '.' => FieldType::Empty,
            _ => panic!("Could not convert {value} to FieldType"),
        }
    }
}
