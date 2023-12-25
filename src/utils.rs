use std::fmt::Display;

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
pub enum Direction {
    West,
    East,
    North,
    South,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Clone)]
pub struct Grid<T> {
    rows: Vec<Vec<Field<T>>>,
}

#[derive(Debug, Clone)]
pub struct Field<T> {
    pub position: Position,
    pub value: T,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Position {
        Position { x, y }
    }
}

impl<T> Field<T> {
    pub fn new(position: Position, value: T) -> Field<T> {
        Field { position, value }
    }
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
    pub fn get_field(&self, position: Position) -> Option<&Field<T>> {
        match self.rows.get(position.y) {
            Some(row) => row.get(position.x),
            None => None,
        }
    }

    pub fn get_fields(&self) -> Vec<&Field<T>> {
        self.rows.iter().flatten().collect()
    }

    pub fn get_rows(&self) -> std::slice::Iter<'_, Vec<Field<T>>> {
        self.rows.iter()
    }

    pub fn row_length(&self) -> usize {
        self.rows.len()
    }

    pub fn col_length(&self) -> Option<usize> {
        Some(self.rows.first()?.len())
    }

    pub fn get_row(&self, idx: usize) -> Option<&Vec<Field<T>>> {
        self.rows.get(idx)
    }

    pub fn get_column(&self, idx: usize) -> Option<Vec<&Field<T>>> {
        Some(self.rows.iter().filter_map(|row| row.get(idx)).collect())
    }

    pub fn west_of(&self, position: Position) -> Option<&Field<T>> {
        let new_position = position.move_to(Direction::West)?;

        self.get_field(new_position)
    }

    pub fn east_of(&self, position: Position) -> Option<&Field<T>> {
        let new_position = position.move_to(Direction::East)?;

        self.get_field(new_position)
    }

    pub fn south_of(&self, position: Position) -> Option<&Field<T>> {
        let new_position = position.move_to(Direction::South)?;

        self.get_field(new_position)
    }

    pub fn north_of(&self, position: Position) -> Option<&Field<T>> {
        let new_position = position.move_to(Direction::North)?;

        self.get_field(new_position)
    }

    pub fn south_west_of(&self, position: Position) -> Option<&Field<T>> {
        let new_position = position.move_to(Direction::South)?;

        self.get_field(new_position.move_to(Direction::West)?)
    }

    pub fn south_east_of(&self, position: Position) -> Option<&Field<T>> {
        let new_position = position.move_to(Direction::South)?;

        self.get_field(new_position.move_to(Direction::East)?)
    }

    pub fn north_west_of(&self, position: Position) -> Option<&Field<T>> {
        let new_position = position.move_to(Direction::North)?;

        self.get_field(new_position.move_to(Direction::West)?)
    }

    pub fn north_east_of(&self, position: Position) -> Option<&Field<T>> {
        let new_position = position.move_to(Direction::North)?;

        self.get_field(new_position.move_to(Direction::East)?)
    }
}

impl<T> Field<T> {
    pub fn is_adjacent_to(&self, position: Position) -> bool {
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
    pub fn move_to(&self, direction: Direction) -> Option<Position> {
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
