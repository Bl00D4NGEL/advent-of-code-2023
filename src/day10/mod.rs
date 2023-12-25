use std::fmt::Debug;

use crate::{
    solver::{Solver, SolvingError},
    utils::{Direction, Field, Grid, Position},
};

pub struct Day10 {}

impl Solver<usize, SolvingError> for Day10 {
    fn part_1(&self, contents: &str) -> Result<usize, SolvingError> {
        let lines = contents.split('\n');

        let grid = parse_input(lines.collect());

        let mut start_position: Option<Position> = None;

        for (y, row) in grid.get_rows().enumerate() {
            for (x, pipe) in row.iter().enumerate() {
                if pipe.value.eq(&PipeType::StartingPosition) {
                    start_position = Some(pipe.position);
                    break;
                }
            }
        }

        if start_position.is_none() {
            panic!("Could not determine starting position");
        }

        let mut current_position = start_position.unwrap();
        let mut loop_pipes: Vec<Position> = vec![current_position];

        loop {
            let connections = find_adjacent_connections(&grid, current_position);

            if connections.len() == 2 {
                let mut added_loop_pipes = false;

                for connection in connections.iter() {
                    let has_connection = loop_pipes.iter().any(|position| {
                        position.x == connection.position.x && position.y == connection.position.y
                    });

                    if !has_connection {
                        added_loop_pipes = true;
                        loop_pipes.push(connection.position);
                        current_position = connection.position;
                        break;
                    }
                }

                if !added_loop_pipes {
                    println!("Loop already has first AND last.. stopping");
                    break;
                }
            } else {
                println!("End? {:?}", connections);
                break;
            }
        }

        Ok((loop_pipes.len() + loop_pipes.len() % 2) / 2)
    }

    fn part_2(&self, contents: &str) -> Result<usize, SolvingError> {
        let lines = contents.split('\n');

        let grid = parse_input(lines.collect());

        let mut start_position: Option<(usize, usize)> = None;

        for (y, row) in grid.get_rows().enumerate() {
            for (x, pipe) in row.iter().enumerate() {
                if pipe.value.eq(&PipeType::StartingPosition) {
                    start_position = Some((x, y));
                    break;
                }
            }
        }

        if start_position.is_none() {
            panic!("Could not determine starting position");
        }

        let ground_pipes = grid
            .get_rows()
            .flat_map(|row| {
                row.iter()
                    .filter(|field| field.value.eq(&PipeType::Ground))
                    .collect::<Vec<&Field<PipeType>>>()
            })
            .collect::<Vec<&Field<PipeType>>>();

        // dbg!(&ground_pipes);
        // Find pipes that must be outside
        // Pipes must be outside if their x or y coordinate is 0 and their are next to a pipe
        let mut definetely_outside = vec![];
        for field in ground_pipes.iter() {
            if field.position.x == 0 {
                if let Some(east) = grid.east_of(field.position) {
                    // If north field is a pipe then the pipe is definetely outside
                    match east.value {
                        PipeType::Ground => (),
                        _ => {
                            definetely_outside.push(field);
                        }
                    }
                }
                if let Some(west) = grid.west_of(field.position) {
                    // If north field is a pipe then the pipe is definetely outside
                    match west.value {
                        PipeType::Ground => (),
                        _ => {
                            definetely_outside.push(field);
                        }
                    }
                }
            }
            if field.position.y == 0 {
                if let Some(south) = grid.south_of(field.position) {
                    // If north field is a pipe then the pipe is definetely outside
                    match south.value {
                        PipeType::Ground => (),
                        _ => {
                            definetely_outside.push(field);
                        }
                    }
                }
                if let Some(north) = grid.north_of(field.position) {
                    // If north field is a pipe then the pipe is definetely outside
                    match north.value {
                        PipeType::Ground => (),
                        _ => {
                            definetely_outside.push(field);
                        }
                    }
                }
            }
        }

        let pipes_left = ground_pipes
            .iter()
            .filter(|pipe| {
                dbg!(pipe);
                for outside_pipe in definetely_outside.iter() {
                    if outside_pipe.position.x == pipe.position.x
                        && outside_pipe.position.y == pipe.position.y
                    {
                        return false;
                    }
                }

                true
            })
            .collect::<Vec<&&Field<PipeType>>>();

        dbg!(pipes_left);

        // parse input
        // find all available "ground pipes"
        // group pipes together that are directly adjecant to each other
        // Pick any random group/ground pipe and start looking up, down, left and right
        // IF all directions are "enclosing" pipes we are probably trapped inside
        // Going back is not possible therefore the direction we came from is considered "enclosed" when scanning surrounding
        // And enclosing pipe would be F7 or LJ. This would be a "dead end":
        // |.|
        // L-J
        // as well as
        // ||
        // LJ

        // Trying to go into any direction means we need to either walk on ground (easy as it's just up, down, left or right) or squeeze through the pipes
        // || or JF can be passed through (since there's a gap inbetween)
        // a movement attempt therefore not only becomes going "straight" but also being able to traverse diagonally

        // |S-7
        // ||.|
        // |L-J
        // |X.|
        // ----

        // X can go through the "top left" but no other direction, if we were to write this in coordinates it would be something like
        // FROM (1, 1) THROUGH / TO (0, 2), (1, 2)

        //  any direction has no pipe => save current pipe in temporary group, go into available direction
        // We need to keep following through pipes until we either:
        // - Reach a dead end
        // - Reach a ground tile that is already considered "inside" or "outside"
        // - Reach the end of the grid -> All pipes are outside

        // If we reach a dead end and no other path lead to any other escape possibility all visited pipes are considered "inside"

        Err(SolvingError::NotImplemented)
    }
}

fn parse_input(lines: Vec<&str>) -> Grid<PipeType> {
    let mut rows = vec![];
    for (y, line) in lines.into_iter().rev().enumerate() {
        let row = line
            .chars()
            .filter_map(|c| match c {
                '|' => Some(PipeType::Vertical),
                '-' => Some(PipeType::Horizontal),
                'L' => Some(PipeType::NorthEast),
                'J' => Some(PipeType::NorthWest),
                '7' => Some(PipeType::SouthWest),
                'F' => Some(PipeType::SouthEast),
                '.' => Some(PipeType::Ground),
                'S' => Some(PipeType::StartingPosition),
                _ => None,
            })
            .collect::<Vec<PipeType>>();
        rows.push(row);
    }

    Grid::from(rows)
}

fn find_adjacent_connections(grid: &Grid<PipeType>, position: Position) -> Vec<&Field<PipeType>> {
    let current_pipe = grid.get_field(position).unwrap().clone();

    let adjacent_pipes = vec![
        (grid.north_of(position), Direction::North),
        (grid.south_of(position), Direction::South),
        (grid.east_of(position), Direction::East),
        (grid.west_of(position), Direction::West),
    ];
    adjacent_pipes
        .into_iter()
        .filter_map(|(p, d)| p.filter(|&pipe| current_pipe.value.connects_to(pipe.value, d)))
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PipeType {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    StartingPosition,
}

impl PipeType {
    fn connects_to(&self, other_pipe: PipeType, direction: Direction) -> bool {
        match self {
            PipeType::Ground => false,
            PipeType::StartingPosition => matches!(
                (direction, other_pipe),
                (Direction::North, PipeType::Vertical)
                    | (Direction::North, PipeType::SouthEast)
                    | (Direction::North, PipeType::SouthWest)
                    | (Direction::South, PipeType::Vertical)
                    | (Direction::South, PipeType::NorthEast)
                    | (Direction::South, PipeType::NorthWest)
                    | (Direction::East, PipeType::Horizontal)
                    | (Direction::East, PipeType::NorthWest)
                    | (Direction::East, PipeType::SouthWest)
                    | (Direction::West, PipeType::Horizontal)
                    | (Direction::West, PipeType::NorthEast)
                    | (Direction::West, PipeType::SouthEast)
            ),
            PipeType::Vertical => matches!(
                (direction, other_pipe),
                (Direction::North, PipeType::Vertical)
                    | (Direction::North, PipeType::SouthEast)
                    | (Direction::North, PipeType::SouthWest)
                    | (Direction::North, PipeType::StartingPosition)
                    | (Direction::South, PipeType::Vertical)
                    | (Direction::South, PipeType::NorthEast)
                    | (Direction::South, PipeType::NorthWest)
                    | (Direction::South, PipeType::StartingPosition)
            ),
            PipeType::Horizontal => matches!(
                (direction, other_pipe),
                (Direction::West, PipeType::Horizontal)
                    | (Direction::West, PipeType::SouthEast)
                    | (Direction::West, PipeType::NorthEast)
                    | (Direction::West, PipeType::StartingPosition)
                    | (Direction::East, PipeType::Horizontal)
                    | (Direction::East, PipeType::SouthWest)
                    | (Direction::East, PipeType::NorthWest)
                    | (Direction::East, PipeType::StartingPosition)
            ),
            PipeType::NorthEast => matches!(
                (direction, other_pipe),
                (Direction::North, PipeType::Vertical)
                    | (Direction::North, PipeType::SouthEast)
                    | (Direction::North, PipeType::SouthWest)
                    | (Direction::North, PipeType::StartingPosition)
                    | (Direction::East, PipeType::Horizontal)
                    | (Direction::East, PipeType::NorthWest)
                    | (Direction::East, PipeType::SouthWest)
                    | (Direction::East, PipeType::StartingPosition)
            ),
            PipeType::NorthWest => matches!(
                (direction, other_pipe),
                (Direction::North, PipeType::Vertical)
                    | (Direction::North, PipeType::SouthEast)
                    | (Direction::North, PipeType::SouthWest)
                    | (Direction::North, PipeType::StartingPosition)
                    | (Direction::West, PipeType::Horizontal)
                    | (Direction::West, PipeType::NorthEast)
                    | (Direction::West, PipeType::SouthEast)
                    | (Direction::West, PipeType::StartingPosition)
            ),
            PipeType::SouthWest => matches!(
                (direction, other_pipe),
                (Direction::South, PipeType::Vertical)
                    | (Direction::South, PipeType::NorthEast)
                    | (Direction::South, PipeType::NorthWest)
                    | (Direction::South, PipeType::StartingPosition)
                    | (Direction::West, PipeType::Horizontal)
                    | (Direction::West, PipeType::NorthEast)
                    | (Direction::West, PipeType::SouthEast)
                    | (Direction::West, PipeType::StartingPosition)
            ),
            PipeType::SouthEast => matches!(
                (direction, other_pipe),
                (Direction::South, PipeType::Vertical)
                    | (Direction::South, PipeType::NorthEast)
                    | (Direction::South, PipeType::NorthWest)
                    | (Direction::South, PipeType::StartingPosition)
                    | (Direction::East, PipeType::Horizontal)
                    | (Direction::East, PipeType::SouthWest)
                    | (Direction::East, PipeType::NorthWest)
                    | (Direction::East, PipeType::StartingPosition)
            ),
            _ => false,
        }
    }
}
