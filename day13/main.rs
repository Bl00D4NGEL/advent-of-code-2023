fn main() {
    let contents = include_str!("./input.txt");
    // 28237 too low
    // 33337 too high
    dbg!(part_1(contents));
    dbg!(part_2(contents));
}

fn part_1(contents: &str) -> usize {
    let lines = contents.split("\n\n");

    let grids = lines
        .map(|line| {
            line.split('\n')
                .filter(|l| !l.is_empty())
                .map(|l| l.chars().collect())
                .collect::<Vec<Vec<char>>>()
        })
        .collect::<Vec<Vec<Vec<char>>>>();

    grids
        .iter()
        .enumerate()
        .map(|(grid_idx, grid)| {
            let first_row = grid.get(0).unwrap();
            let mut first_row_mirror_points = vec![];

            for (char_idx, char) in first_row.iter().enumerate() {
                if char_idx == 0 {
                    continue;
                }

                let prev_char = match first_row.get(char_idx - 1) {
                    None => continue,
                    Some(v) => v,
                };

                if !prev_char.eq(char) {
                    continue;
                }

                let mut walk_success = true;
                // try to walk back / forth the grid grid and check if the pairs still match
                for step in 0..char_idx {
                    let up_idx = char_idx as i64 - step as i64 - 1;
                    let down_idx = step + char_idx;

                    if up_idx < 0 {
                        break;
                    }
                    match (first_row.get(up_idx as usize), first_row.get(down_idx)) {
                        (Some(up), Some(down)) => {
                            if !up.eq(down) {
                                walk_success = false;
                                break;
                            }
                        }
                        _ => {
                            break;
                        }
                    }
                }

                if walk_success {
                    first_row_mirror_points.push(char_idx);
                }
            }

            for mirror_point in first_row_mirror_points.iter() {
                let all_rows_mirror = grid.iter().all(|row| {
                    for step in 0..*mirror_point {
                        let up_idx = *mirror_point as i64 - step as i64 - 1;
                        let down_idx = step + mirror_point;

                        if up_idx < 0 {
                            break;
                        }
                        match (row.get(up_idx as usize), row.get(down_idx)) {
                            (Some(up), Some(down)) => {
                                if !up.eq(down) {
                                    return false;
                                }
                            }
                            _ => {
                                break;
                            }
                        }
                    }

                    true
                });

                if all_rows_mirror {
                    return *mirror_point;
                }
            }

            for (i, row) in grid.iter().enumerate() {
                if i == 0 {
                    continue;
                }

                let prev_row = match grid.get(i - 1) {
                    None => continue,
                    Some(v) => v,
                };

                if !prev_row.eq(row) {
                    continue;
                }

                let mut walk_success = true;
                // try to walk back / forth the grid and check if the pairs still match
                for step in 0..i {
                    let up_idx = i as i64 - step as i64 - 1;
                    let down_idx = step + i;

                    if up_idx < 0 {
                        break;
                    }
                    match (grid.get(up_idx as usize), grid.get(down_idx)) {
                        (Some(up), Some(down)) => {
                            if !up.eq(down) {
                                walk_success = false;
                                break;
                            }
                        }
                        _ => {
                            break;
                        }
                    }
                }

                if walk_success {
                    return i * 100;
                }
            }

            panic!("Not vertical or horizontal {grid_idx}?");
        })
        .sum()
}

fn part_2(contents: &str) -> usize {
    1
}
