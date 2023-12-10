use std::fmt::Debug;

fn main() {
    let contents = include_str!("./example.txt");
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

    if let Some((x, y)) = start_position {
        let current_pipe = grid.get_field(x, y).unwrap().clone();

        let adjacent_pipes = vec![
            (grid.north_of(x, y), Direction::North),
            (grid.south_of(x, y), Direction::South),
            (grid.east_of(x, y), Direction::East),
            (grid.west_of(x, y), Direction::West),
        ];
        let connections = adjacent_pipes
            .iter()
            .filter_map(|(p, d)| match p {
                None => None,
                Some(pipe) => {
                    if current_pipe.value.connects_to(pipe.value, *d) {
                        Some((pipe, d))
                    } else {
                        None
                    }
                }
            })
            .collect::<Vec<(&&Field<PipeType>, &Direction)>>();

        dbg!(current_pipe, connections);
        1
    } else {
        panic!("Could not determine starting position");
    }
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
            PipeType::StartingPosition => match (direction, other_pipe) {
                (Direction::North, PipeType::Vertical) => true,
                (Direction::North, PipeType::SouthEast) => true,
                (Direction::North, PipeType::SouthWest) => true,
                (Direction::South, PipeType::Vertical) => true,
                (Direction::South, PipeType::NorthEast) => true,
                (Direction::South, PipeType::NorthWest) => true,
                (Direction::East, PipeType::Horizontal) => true,
                (Direction::East, PipeType::NorthWest) => true,
                (Direction::East, PipeType::SouthWest) => true,
                (Direction::West, PipeType::Horizontal) => true,
                (Direction::West, PipeType::NorthEast) => true,
                (Direction::West, PipeType::SouthEast) => true,
                _ => false,
            },
            PipeType::Vertical => match (direction, other_pipe) {
                (Direction::North, PipeType::SouthEast) => true,
                (Direction::North, PipeType::SouthWest) => true,
                (Direction::North, PipeType::StartingPosition) => true,
                (Direction::South, PipeType::NorthEast) => true,
                (Direction::South, PipeType::NorthWest) => true,
                (Direction::South, PipeType::StartingPosition) => true,
                _ => false,
            },
            PipeType::Horizontal => match (direction, other_pipe) {
                (Direction::West, PipeType::SouthEast) => true,
                (Direction::West, PipeType::NorthEast) => true,
                (Direction::West, PipeType::StartingPosition) => true,
                (Direction::East, PipeType::SouthWest) => true,
                (Direction::East, PipeType::NorthWest) => true,
                (Direction::East, PipeType::StartingPosition) => true,
                _ => false,
            },
            PipeType::NorthEast => match (direction, other_pipe) {
                (Direction::North, PipeType::Vertical) => true,
                (Direction::North, PipeType::SouthEast) => true,
                (Direction::North, PipeType::SouthWest) => true,
                (Direction::North, PipeType::StartingPosition) => true,
                (Direction::East, PipeType::Horizontal) => true,
                (Direction::East, PipeType::NorthWest) => true,
                (Direction::East, PipeType::SouthWest) => true,
                (Direction::East, PipeType::StartingPosition) => true,
                _ => false,
            },
            PipeType::NorthWest => match (direction, other_pipe) {
                (Direction::North, PipeType::Vertical) => true,
                (Direction::North, PipeType::SouthEast) => true,
                (Direction::North, PipeType::SouthWest) => true,
                (Direction::North, PipeType::StartingPosition) => true,
                (Direction::West, PipeType::Horizontal) => true,
                (Direction::West, PipeType::NorthEast) => true,
                (Direction::West, PipeType::SouthEast) => true,
                (Direction::West, PipeType::StartingPosition) => true,
                _ => false,
            },
            PipeType::SouthWest => match (direction, other_pipe) {
                (Direction::South, PipeType::Vertical) => true,
                (Direction::South, PipeType::NorthEast) => true,
                (Direction::South, PipeType::NorthWest) => true,
                (Direction::South, PipeType::StartingPosition) => true,
                (Direction::West, PipeType::Horizontal) => true,
                (Direction::West, PipeType::NorthEast) => true,
                (Direction::West, PipeType::SouthEast) => true,
                (Direction::West, PipeType::StartingPosition) => true,
                _ => false,
            },
            PipeType::SouthEast => match (direction, other_pipe) {
                (Direction::South, PipeType::Vertical) => true,
                (Direction::South, PipeType::NorthEast) => true,
                (Direction::South, PipeType::NorthWest) => true,
                (Direction::South, PipeType::StartingPosition) => true,
                (Direction::East, PipeType::Horizontal) => true,
                (Direction::East, PipeType::SouthWest) => true,
                (Direction::East, PipeType::NorthWest) => true,
                (Direction::East, PipeType::StartingPosition) => true,
                _ => false,
            },
            _ => false,
        }
    }
}

fn part_2(contents: &str) -> usize {
    1
}
