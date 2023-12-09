fn main() {
    let contents = include_str!("./input.txt");
    dbg!(part_1(contents));
    // 20762 too high
    dbg!(part_2(contents));
}

fn part_1(contents: &str) -> i32 {
    contents
        .split('\n')
        .filter_map(|line| {
            if line.is_empty() {
                return None;
            }

            let initial_sequence = line
                .split(' ')
                .filter_map(|s| s.parse::<i32>().ok())
                .collect::<Vec<i32>>();

            extrapolate_next_number(initial_sequence)
        })
        .sum()
}

fn extrapolate_next_number(sequence: Vec<i32>) -> Option<i32> {
    let initial_last = *sequence.last()?;

    let mut new_sequence = sequence;

    let mut all_differences = vec![];

    loop {
        let mut differences = vec![];
        if new_sequence.len() == 0 {
            break;
        }

        let mut iter = new_sequence.iter();
        let mut prev = iter.next().unwrap();
        for number in iter {
            differences.push(number - prev);

            prev = number;
        }

        if differences.iter().all(|d| *d == 0) {
            break;
        }
        all_differences.push(differences.clone());
        new_sequence = differences;
    }

    let mut prev = 0;
    while let Some(differences) = all_differences.pop() {
        if let Some(last) = differences.last() {
            prev = last + prev;
        }
    }

    Some(prev + initial_last)
}

fn part_2(contents: &str) -> i32 {
    contents
        .split('\n')
        .filter_map(|line| {
            if line.is_empty() {
                return None;
            }

            let initial_sequence = line
                .split(' ')
                .filter_map(|s| s.parse::<i32>().ok())
                .collect::<Vec<i32>>();

            extrapolate_previous_number(initial_sequence)
        })
        .sum()
}

fn extrapolate_previous_number(sequence: Vec<i32>) -> Option<i32> {
    let initial_first = *sequence.first()?;

    let mut new_sequence = sequence;

    let mut all_differences = vec![];

    loop {
        let mut differences = vec![];
        if new_sequence.len() == 0 {
            break;
        }

        let mut iter = new_sequence.iter();
        let mut prev = iter.next().unwrap();
        for number in iter {
            differences.push(number - prev);

            prev = number;
        }

        if differences.iter().all(|d| *d == 0) {
            break;
        }
        all_differences.push(differences.clone());
        new_sequence = differences;
    }

    let mut prev = 0;
    while let Some(differences) = all_differences.pop() {
        if let Some(first) = differences.first() {
            prev = first - prev;
        }
    }

    Some(initial_first - prev)
}
