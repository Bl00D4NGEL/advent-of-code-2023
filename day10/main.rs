use std::fmt::Debug;

fn main() {
    let contents = include_str!("./input.txt");
    dbg!(part_1(contents));
    dbg!(part_2(contents));
}

fn part_1(contents: &str) -> usize {
    let lines = contents.split('\n');

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

    let grid = Grid {
        rows: rows
            .iter()
            .enumerate()
            .map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .map(|(x, pipe)| Field { x, y, value: *pipe })
                    .collect::<Vec<Field<PipeType>>>()
            })
            .collect::<Vec<Vec<Field<PipeType>>>>(),
    };

    let mut start_position: Option<(usize, usize)> = None;

    for (y, row) in grid.rows.iter().enumerate() {
        for (x, pipe) in row.iter().enumerate() {
            if pipe.value.eq(&PipeType::StartingPosition) {
                start_position = Some((x, y));
                break;
            }
        }
    }

    dbg!(&start_position);

    if start_position.is_none() {
        panic!("Could not determine starting position");
    }

    let (mut current_x, mut current_y) = start_position.unwrap();
    let mut loop_pipes: Vec<(usize, usize)> = vec![(current_x, current_y)];

    loop {
        let connections = find_adjacent_connections(&grid, current_x, current_y);

        if connections.len() == 2 {
            let mut added_loop_pipes = false;

            for connection in connections.iter() {
                let has_connection = loop_pipes
                    .iter()
                    .any(|(x, y)| *x == connection.x && *y == connection.y);

                if !has_connection {
                    added_loop_pipes = true;
                    loop_pipes.push((connection.x, connection.y));
                    current_x = connection.x;
                    current_y = connection.y;
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

    dbg!(loop_pipes.len());

    (loop_pipes.len() + loop_pipes.len() % 2) / 2
}

fn find_adjacent_connections(grid: &Grid<PipeType>, x: usize, y: usize) -> Vec<&Field<PipeType>> {
    let current_pipe = grid.get_field(x, y).unwrap().clone();

    let adjacent_pipes = vec![
        (grid.north_of(x, y), Direction::North),
        (grid.south_of(x, y), Direction::South),
        (grid.east_of(x, y), Direction::East),
        (grid.west_of(x, y), Direction::West),
    ];
    adjacent_pipes
        .into_iter()
        .filter_map(|(p, d)| p.filter(|&pipe| current_pipe.value.connects_to(pipe.value, d)))
        .collect()
}

#[derive(Debug, Clone)]
struct Grid<T> {
    rows: Vec<Vec<Field<T>>>,
}

#[derive(Debug, Clone)]
struct Field<T> {
    x: usize,
    y: usize,
    value: T,
}

impl Grid<PipeType> {
    fn get_field(&self, x: usize, y: usize) -> Option<&Field<PipeType>> {
        match self.rows.get(y) {
            Some(row) => row.get(x),
            None => None,
        }
    }

    fn west_of(&self, x: usize, y: usize) -> Option<&Field<PipeType>> {
        if x == 0 {
            None
        } else {
            self.get_field(x - 1, y)
        }
    }

    fn east_of(&self, x: usize, y: usize) -> Option<&Field<PipeType>> {
        self.get_field(x + 1, y)
    }

    fn south_of(&self, x: usize, y: usize) -> Option<&Field<PipeType>> {
        if y == 0 {
            None
        } else {
            self.get_field(x, y - 1)
        }
    }

    fn north_of(&self, x: usize, y: usize) -> Option<&Field<PipeType>> {
        self.get_field(x, y + 1)
    }
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    South,
    West,
    East,
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

fn part_2(contents: &str) -> usize {
    1
}
