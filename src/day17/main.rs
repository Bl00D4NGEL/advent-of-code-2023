use std::fmt::{Display, Pointer};

fn main() {
    let contents = include_str!("./example.txt");
    dbg!(part_1(contents));
    dbg!(part_2(contents));
}

fn part_1(contents: &str) -> usize {
    let rows = contents
        .lines()
        .map(|line| line.chars().filter_map(|c| c.to_digit(10)).collect())
        .collect::<Vec<Vec<u32>>>();

    let grid = Grid::from(rows);

    println!("{}", grid);
    1
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

impl<T> Display for Grid<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.rows.iter() {
            for field in row {
                write!(f, "{}", field.value)?;
            }
            writeln!(f, "")?;
        }
        Ok(())
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
