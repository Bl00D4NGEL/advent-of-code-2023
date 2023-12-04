fn main() {
    let contents = include_str!("./input.txt");
    let lines = contents.split('\n');

    let mut symbols = vec![];
    let mut digits = vec![];

    for (line_idx, line) in lines.clone().enumerate() {
        if line.is_empty() {
            continue;
        }

        for (char_idx, character) in line.chars().enumerate() {
            if character.is_ascii_digit() {
                digits.push((line_idx, char_idx));
            } else if character != '.' {
                symbols.push((line_idx, char_idx));
            }
        }
    }

    let mut counting_zones = vec![];

    for (line_idx, char_idx) in symbols.iter() {
        counting_zones.push((*line_idx + 1, *char_idx + 1));
        counting_zones.push((*line_idx + 1, *char_idx));
        counting_zones.push((*line_idx, *char_idx + 1));
        counting_zones.push((*line_idx, *char_idx));
        if *char_idx > 0 {
            counting_zones.push((*line_idx + 1, *char_idx - 1));
            counting_zones.push((*line_idx, *char_idx - 1));
        }
        if *line_idx > 0 {
            counting_zones.push((*line_idx - 1, *char_idx + 1));
            counting_zones.push((*line_idx - 1, *char_idx));
            if *char_idx > 0 {
                counting_zones.push((*line_idx - 1, *char_idx - 1));
            }
        }
    }

    counting_zones.sort();
    counting_zones.dedup();

    let mut counted_numbers = vec![];
    for (line_idx, line) in lines.enumerate() {
        let mut current_number = String::new();
        let mut is_counting_number = false;
        for (char_idx, character) in line.chars().enumerate() {
            if character.is_ascii_digit() {
                if current_number.is_empty() {
                    // Starting new number here
                    current_number = character.to_string();
                } else {
                    // Already counding number
                    current_number.extend([character.to_string()]);
                }

                if counting_zones.contains(&(line_idx, char_idx)) {
                    is_counting_number = true;
                }
            } else {
                if !current_number.is_empty() && is_counting_number {
                    counted_numbers.push(current_number.clone());
                }
                current_number = String::new();
                is_counting_number = false;
            }
        }

        if !current_number.is_empty() && is_counting_number {
            counted_numbers.push(current_number.clone());
        }
    }

    let result: u32 = counted_numbers
        .iter()
        .map(|n| n.parse::<u32>().unwrap())
        .sum();

    dbg!(result);
}
