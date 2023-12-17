fn main() {
    let contents = include_str!("./input.txt");
    dbg!(part_1(contents));
    dbg!(part_2(contents));
}

fn part_1(contents: &str) -> usize {
    let rows = contents
        .lines()
        .rev()
        .map(|line| line.chars().map(FieldType::from).collect())
        .collect::<Vec<Vec<FieldType>>>();

    let grid = Grid::from(rows);

    let current_position = Position {
        x: 0,
        y: grid.rows.len() - 1,
    };
    let direction = Direction::East;
    let mut energized = vec![];

    move_in_grid(&grid, direction, current_position, &mut energized);

    let mut positions = energized
        .iter()
        .map(|(_, pos)| pos)
        .collect::<Vec<&Position>>();
    positions.sort();
    positions.dedup();

    positions.len()
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

fn part_2(contents: &str) -> usize {
    1
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
enum Direction {
    West,
    East,
    North,
    South,
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

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone)]
struct Grid<T> {
    rows: Vec<Vec<Field<T>>>,
}

#[derive(Debug, Clone)]
struct Field<T> {
    position: Position,
    value: T,
}

impl<T> From<Vec<Vec<T>>> for Grid<T> {
    fn from(value: Vec<Vec<T>>) -> Self {
        Grid {
            rows: value
                .into_iter()
                .enumerate()
                .map(|(row_idx, row)| {
                    row.into_iter()
                        .enumerate()
                        .map(|(col_idx, col)| Field {
                            position: Position {
                                x: col_idx,
                                y: row_idx,
                            },
                            value: col,
                        })
                        .collect()
                })
                .collect(),
        }
    }
}

impl<T> Grid<T> {
    fn get_field(&self, position: Position) -> Option<&Field<T>> {
        match self.rows.get(position.y) {
            Some(row) => row.get(position.x),
            None => None,
        }
    }

    fn get_fields(&self) -> Vec<&Field<T>> {
        self.rows.iter().flatten().collect()
    }

    fn get_row(&self, idx: usize) -> Option<&Vec<Field<T>>> {
        self.rows.get(idx)
    }

    fn get_column(&self, idx: usize) -> Vec<&Field<T>> {
        self.rows.iter().filter_map(|row| row.get(idx)).collect()
    }

    fn west_of(&self, position: Position) -> Option<&Field<T>> {
        let new_position = position.move_to(Direction::West)?;

        self.get_field(new_position)
    }

    fn east_of(&self, position: Position) -> Option<&Field<T>> {
        let new_position = position.move_to(Direction::East)?;

        self.get_field(new_position)
    }

    fn south_of(&self, position: Position) -> Option<&Field<T>> {
        let new_position = position.move_to(Direction::South)?;

        self.get_field(new_position)
    }

    fn north_of(&self, position: Position) -> Option<&Field<T>> {
        let new_position = position.move_to(Direction::North)?;

        self.get_field(new_position)
    }
}

impl<T> Field<T> {
    fn is_adjacent_to(&self, position: Position) -> bool {
        if self.position.x == position.x {
            if self.position.y == position.y + 1 {
                true
            } else {
                self.position.y == position.y - 1
            }
        } else if self.position.y == position.y {
            if self.position.x == position.x + 1 {
                true
            } else {
                self.position.x == position.x - 1
            }
        } else {
            false
        }
    }
}

impl Position {
    fn move_to(&self, direction: Direction) -> Option<Position> {
        match direction {
            Direction::West => {
                if self.x == 0 {
                    None
                } else {
                    Some(Position {
                        x: self.x - 1,
                        y: self.y,
                    })
                }
            }
            Direction::East => Some(Position {
                x: self.x + 1,
                y: self.y,
            }),
            Direction::North => Some(Position {
                x: self.x,
                y: self.y + 1,
            }),
            Direction::South => {
                if self.y == 0 {
                    None
                } else {
                    Some(Position {
                        x: self.x,
                        y: self.y - 1,
                    })
                }
            }
        }
    }
}
