use crate::solver::{Solver, SolvingError};

pub struct Day11 {}

impl Solver<usize, SolvingError> for Day11 {
    fn part_1(&self, contents: &str) -> Result<usize, SolvingError> {
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

        Ok(distances.iter().sum())
    }

    fn part_2(&self, contents: &str) -> Result<usize, SolvingError> {
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

        Ok(distances.iter().sum())
    }
}
