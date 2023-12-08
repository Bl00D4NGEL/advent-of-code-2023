use std::collections::HashMap;

fn main() {
    let contents = include_str!("./input.txt");
    dbg!(part_1(contents));
    dbg!(part_2(contents));
}

fn part_1(contents: &str) -> u32 {
    let mut lines = contents.split('\n');

    let instructions = lines.next().unwrap();

    let mut networks: HashMap<String, (String, String)> = HashMap::new();

    for line in lines {
        let mut split = line.split(" = ");

        if let Some(root) = split.next() {
            if let Some(targets) = split.next() {
                let chars = targets
                    .chars()
                    .filter(|c| c.is_ascii_alphabetic())
                    .collect::<Vec<char>>();

                if chars.len() != 6 {
                    println!(
                        "Chars lenth is not 6, instead it is {}, {:?}",
                        chars.len(),
                        chars
                    );
                    dbg!(chars);
                    continue;
                }

                let (left, right) = chars.split_at(3);
                networks.insert(
                    root.to_owned(),
                    (
                        left.iter().collect::<String>(),
                        right.iter().collect::<String>(),
                    ),
                );
            }
        }
    }

    let mut steps = 0;
    let mut current_map = &String::from("AAA");

    for direction in instructions.chars().cycle() {
        if let Some((left, right)) = networks.get(current_map) {
            match direction {
                'R' => {
                    current_map = right;
                }
                'L' => {
                    current_map = left;
                }
                _ => {
                    println!("Unknown direction given: {direction}")
                }
            }
        }

        steps += 1;
        if current_map == "ZZZ" {
            break;
        }
    }

    steps
}

fn part_2(contents: &str) -> u32 {
    1
}
