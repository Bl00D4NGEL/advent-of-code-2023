pub mod day01;

fn main() {
    let contents = include_str!("./input.txt");
    let lines = contents.split('\n');
    let mut sum = 0;

    let digits = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];

    for line in lines {
        if line.is_empty() {
            continue;
        }

        let first_digit = find_first_digit(line, &digits).unwrap_or(0);
        let last_digit = find_last_digit(line, &digits).unwrap_or(0);

        sum += first_digit * 10;
        sum += last_digit;
    }

    dbg!(sum);
}

fn find_first_digit(line: &str, digits: &[(&str, u32)]) -> Option<u32> {
    let mut cloned = line.to_string();
    while !cloned.is_empty() {
        for (digit_as_string, digit) in digits {
            if cloned.starts_with(digit_as_string) {
                return Some(*digit);
            }
            if cloned.starts_with(digit.to_string().as_str()) {
                return Some(*digit);
            }
        }

        cloned.remove(0);
    }

    None
}

fn find_last_digit(line: &str, digits: &[(&str, u32)]) -> Option<u32> {
    let mut cloned = line.to_string();
    while !cloned.is_empty() {
        for (digit_as_string, digit) in digits {
            if cloned.ends_with(digit_as_string) {
                return Some(*digit);
            }
            if cloned.ends_with(digit.to_string().as_str()) {
                return Some(*digit);
            }
        }

        cloned.remove(cloned.len() - 1);
    }

    None
}
