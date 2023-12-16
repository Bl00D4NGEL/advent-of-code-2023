use std::fmt::Debug;

fn main() {
    let contents = include_str!("./input.txt");
    dbg!(part_1(contents));
    dbg!(part_2(contents));
}

fn part_1(contents: &str) -> usize {
    let lines = contents.split('\n');

    let mut rows = vec![];

    for line in lines {
        let row = line
            .chars()
            .map(|c| matches!(c, '#'))
            .collect::<Vec<bool>>();

        rows.push(row);
    }

    let mut galaxies = vec![];
    let mut empty_rows = vec![];

    for (row_idx, row) in rows.iter().enumerate() {
        if row.iter().all(|f| !*f) {
            empty_rows.push(row_idx);
        }
        for (col_idx, field) in row.iter().enumerate() {
            if *field {
                galaxies.push((row_idx, col_idx));
            }
        }
    }

    let mut empty_cols: Vec<usize> = vec![];
    let col_len = rows.first().unwrap().len();

    for i in 0..col_len {
        let is_empty = rows.iter().filter_map(|row| row.get(i)).all(|f| !*f);
        if is_empty {
            empty_cols.push(i);
        }
    }

    for (i, empty_row_idx) in empty_rows.iter().enumerate() {
        for galaxy in galaxies.iter_mut() {
            if galaxy.0 > (*empty_row_idx + i) {
                galaxy.0 += 1;
            }
        }
    }
    for (i, empty_col_idx) in empty_cols.iter().enumerate() {
        for galaxy in galaxies.iter_mut() {
            if galaxy.1 > (*empty_col_idx + i) {
                galaxy.1 += 1;
            }
        }
    }

    let galaxy_count = galaxies.len();

    let mut distances = vec![];
    for i in 0..galaxy_count {
        for j in (i + 1)..galaxy_count {
            if let Some(left) = galaxies.get(i) {
                if let Some(right) = galaxies.get(j) {
                    let x_diff = right.0.abs_diff(left.0);
                    let y_diff = right.1.abs_diff(left.1);
                    let distance = x_diff + y_diff;
                    distances.push(distance);
                }
            }
        }
    }

    distances.iter().sum()
}

fn part_2(contents: &str) -> usize {
    let lines = contents.split('\n');

    let mut rows = vec![];

    for line in lines {
        let row = line
            .chars()
            .map(|c| matches!(c, '#'))
            .collect::<Vec<bool>>();

        rows.push(row);
    }

    let mut galaxies = vec![];
    let mut empty_rows = vec![];

    for (row_idx, row) in rows.iter().enumerate() {
        if row.iter().all(|f| !*f) {
            empty_rows.push(row_idx);
        }
        for (col_idx, field) in row.iter().enumerate() {
            if *field {
                galaxies.push((row_idx, col_idx));
            }
        }
    }

    let mut empty_cols: Vec<usize> = vec![];
    let col_len = rows.first().unwrap().len();

    for i in 0..col_len {
        let is_empty = rows.iter().filter_map(|row| row.get(i)).all(|f| !*f);
        if is_empty {
            empty_cols.push(i);
        }
    }

    let multi = 999999;
    for (i, empty_row_idx) in empty_rows.iter().enumerate() {
        for galaxy in galaxies.iter_mut() {
            if galaxy.0 > (*empty_row_idx + (i * multi)) {
                galaxy.0 += multi;
            }
        }
    }
    for (i, empty_col_idx) in empty_cols.iter().enumerate() {
        for galaxy in galaxies.iter_mut() {
            if galaxy.1 > (*empty_col_idx + (i * multi)) {
                galaxy.1 += multi;
            }
        }
    }

    let galaxy_count = galaxies.len();

    let mut distances = vec![];
    for i in 0..galaxy_count {
        for j in (i + 1)..galaxy_count {
            if let Some(left) = galaxies.get(i) {
                if let Some(right) = galaxies.get(j) {
                    let x_diff = right.0.abs_diff(left.0);
                    let y_diff = right.1.abs_diff(left.1);
                    let distance = x_diff + y_diff;
                    distances.push(distance);
                }
            }
        }
    }

    distances.iter().sum()
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

impl<T> Grid<T> {
    fn from_vec(input: Vec<Vec<T>>) -> Grid<T> {
        Grid {
            rows: input
                .into_iter()
                .enumerate()
                .map(|(row_idx, row)| {
                    row.into_iter()
                        .enumerate()
                        .map(|(col_idx, col)| Field {
                            x: col_idx,
                            y: row_idx,
                            value: col,
                        })
                        .collect()
                })
                .collect(),
        }
    }
}

impl<T> Grid<T> {
    fn get_field(&self, x: usize, y: usize) -> Option<&Field<T>> {
        match self.rows.get(y) {
            Some(row) => row.get(x),
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

    fn west_of(&self, x: usize, y: usize) -> Option<&Field<T>> {
        if x == 0 {
            None
        } else {
            self.get_field(x - 1, y)
        }
    }

    fn east_of(&self, x: usize, y: usize) -> Option<&Field<T>> {
        self.get_field(x + 1, y)
    }

    fn south_of(&self, x: usize, y: usize) -> Option<&Field<T>> {
        if y == 0 {
            None
        } else {
            self.get_field(x, y - 1)
        }
    }

    fn south_west_of(&self, x: usize, y: usize) -> Option<&Field<T>> {
        if y == 0 || x == 0 {
            None
        } else {
            self.get_field(x - 1, y - 1)
        }
    }

    fn south_east_of(&self, x: usize, y: usize) -> Option<&Field<T>> {
        if y == 0 {
            None
        } else {
            self.get_field(x + 1, y - 1)
        }
    }

    fn north_of(&self, x: usize, y: usize) -> Option<&Field<T>> {
        self.get_field(x, y + 1)
    }

    fn north_west_of(&self, x: usize, y: usize) -> Option<&Field<T>> {
        if x == 0 {
            None
        } else {
            self.get_field(x - 1, y + 1)
        }
    }

    fn north_east_of(&self, x: usize, y: usize) -> Option<&Field<T>> {
        self.get_field(x + 1, y + 1)
    }
}

impl<T> Field<T> {
    fn is_adjacent_to(&self, x: usize, y: usize) -> bool {
        if self.x == x {
            if self.y == y + 1 {
                true
            } else {
                self.y == y - 1
            }
        } else if self.y == y {
            if self.x == x + 1 {
                true
            } else {
                self.x == x - 1
            }
        } else {
            false
        }
    }
}
