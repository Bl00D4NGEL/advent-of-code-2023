use std::cmp::Ordering;

fn main() {
    let contents = include_str!("./input.txt");
    dbg!(part_1(contents));
    dbg!(part_2(contents));
}

fn part_1(contents: &str) -> usize {
    let lines = contents.lines();

    let mut grid: Vec<Vec<Field>> = vec![];

    for line in lines {
        let row = line
            .chars()
            .map(|c| match c {
                'O' => Field::RoundRock,
                '#' => Field::CubeRock,
                '.' => Field::Empty,
                _ => panic!("Could not match character {c}"),
            })
            .collect();

        grid.push(row)
    }

    let mut columns: Vec<Vec<Field>> = vec![];
    let col_len = grid.first().unwrap().len();
    for column_idx in 0..col_len {
        let column = grid
            .iter()
            .filter_map(|row| row.get(column_idx))
            .copied()
            .collect();
        columns.push(column);
    }

    columns
        .iter_mut()
        .map(|column| {
            let mut chunks = vec![];
            let mut chunk = vec![];
            for field in column.iter() {
                chunk.push(field);
                if field.eq(&Field::CubeRock) {
                    chunk.sort_by(|a, b| match (a, b) {
                        (Field::RoundRock, Field::Empty) => Ordering::Less,
                        (Field::Empty, Field::RoundRock) => Ordering::Greater,
                        _ => Ordering::Equal,
                    });
                    chunks.push(chunk);
                    chunk = vec![];
                }
            }
            chunk.sort_by(|a, b| match (a, b) {
                (Field::RoundRock, Field::Empty) => Ordering::Less,
                (Field::Empty, Field::RoundRock) => Ordering::Greater,
                _ => Ordering::Equal,
            });
            chunks.push(chunk);

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Field {
    RoundRock,
    CubeRock,
    Empty,
}

fn part_2(contents: &str) -> usize {
    1
}
