use std::{cmp::Ordering, str::Lines};

fn main() {
    let contents = include_str!("./example.txt");
    dbg!(part_1(contents));
    dbg!(part_2(contents));
}

fn part_1(contents: &str) -> usize {
    let lines = contents.lines();

    let rows = parse_lines(lines);

    let columns = rows_to_columns(rows);

    let col_len = columns.len();
    columns
        .iter()
        .map(|column| {
            let mut chunks = vec![];
            let mut chunk: Vec<&Field> = vec![];
            for field in column.iter() {
                chunk.push(field);
                if field.to_owned().eq(&Field::CubeRock) {
                    chunks.push(tilt_north(&mut chunk));
                    chunk = vec![];
                }
            }
            chunks.push(tilt_north(&mut chunk));

            chunks.into_iter().flatten().collect::<Vec<&Field>>()
        })
        .map(|column| {
            column
                .iter()
                .enumerate()
                .map(|(idx, field)| {
                    if field.eq(&&Field::RoundRock) {
                        col_len - idx
                    } else {
                        0
                    }
                })
                .sum::<usize>()
        })
        .sum()
}

fn part_2(contents: &str) -> usize {
    let lines = contents.lines();

    let initial_rows = parse_lines(lines);

    let mut rows = initial_rows;

    for i in 0..1_000_000_000 {
        let columns = rows;
        rows = cycle_grid(columns);
    }
    1
}

fn cycle_grid(grid: Vec<Vec<&Field>>) -> Vec<Vec<&Field>> {
    let cloned_grid = grid.clone();
    let tilted_north = rows_to_columns(cloned_grid)
        .iter_mut()
        .map(tilt_north)
        .collect::<Vec<Vec<&Field>>>();

    let tilted_west = columns_to_rows(tilted_north)
        .iter_mut()
        .map(tilt_north)
        .collect();

    let tilted_south = rows_to_columns(tilted_west)
        .iter_mut()
        .map(tilt_south)
        .collect();

    let tilted_east = columns_to_rows(tilted_south)
        .iter_mut()
        .map(tilt_south)
        .collect();

    tilted_east
}

fn parse_lines(lines: Lines<'_>) -> Vec<Vec<&Field>> {
    let mut rows = vec![];

    for line in lines {
        let row = line
            .chars()
            .map(|c| match c {
                'O' => &Field::RoundRock,
                '#' => &Field::CubeRock,
                '.' => &Field::Empty,
                _ => panic!("Could not match character {c}"),
            })
            .collect();

        rows.push(row)
    }

    rows
}

fn rows_to_columns(rows: Vec<Vec<&Field>>) -> Vec<Vec<&Field>> {
    let mut columns = vec![];
    let col_len = rows.first().unwrap().len();
    for column_idx in 0..col_len {
        let column = rows
            .iter()
            .filter_map(|row| row.get(column_idx))
            .copied()
            .collect();
        columns.push(column);
    }

    columns
}

fn columns_to_rows(columns: Vec<Vec<&Field>>) -> Vec<Vec<&Field>> {
    let mut rows = vec![];
    let row_len = columns.first().unwrap().len();
    for row_idx in 0..row_len {
        let row = columns
            .iter()
            .filter_map(|column| column.get(row_idx))
            .copied()
            .collect::<Vec<&Field>>();

        rows.push(row);
    }

    rows
}

fn tilt_north<'a>(fields: &mut Vec<&'a Field>) -> Vec<&'a Field> {
    fields.sort_by(|a, b| match (a, b) {
        (Field::RoundRock, Field::Empty) => Ordering::Less,
        (Field::Empty, Field::RoundRock) => Ordering::Greater,
        _ => Ordering::Equal,
    });

    fields.to_owned()
}

fn tilt_south<'a>(fields: &mut Vec<&'a Field>) -> Vec<&'a Field> {
    fields.sort_by(|a, b| match (a, b) {
        (Field::RoundRock, Field::Empty) => Ordering::Greater,
        (Field::Empty, Field::RoundRock) => Ordering::Less,
        _ => Ordering::Equal,
    });

    fields.to_owned()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Field {
    RoundRock,
    CubeRock,
    Empty,
}
