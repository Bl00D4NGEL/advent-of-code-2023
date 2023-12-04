use std::{collections::HashMap, time::SystemTime};

fn main() {
    let contents = include_str!("./input.txt");
    let lines = contents.split('\n');

    let mut rounds: HashMap<u32, u32> = HashMap::new();
    for line in lines {
        if line.is_empty() {
            continue;
        }

        let mut split = line.split(':');
        let card_number = split
            .next()
            .unwrap()
            .split(' ')
            .last()
            .unwrap()
            .parse::<u32>()
            .unwrap();
        let game = split.next().unwrap();

        let mut draw = game.split('|');
        let winning_numbers = draw
            .next()
            .unwrap()
            .split(' ')
            .filter_map(|d| d.parse::<u8>().ok())
            .collect::<Vec<u8>>();
        let my_numbers = draw
            .next()
            .unwrap()
            .split(' ')
            .filter_map(|d| d.parse::<u8>().ok())
            .collect::<Vec<u8>>();

        let matches = my_numbers
            .iter()
            .filter(|n| winning_numbers.contains(n))
            .collect::<Vec<&u8>>()
            .len() as u32;

        rounds.insert(card_number, matches);
    }

    let result: u32 = rounds
        .values()
        .map(|v| {
            if *v == 0 {
                return 0;
            }

            2_u32.pow(*v - 1)
        })
        .sum();

    let mut existing_copies: HashMap<u32, i32> = HashMap::new();
    for i in 1..=rounds.values().len() as u32 {
        existing_copies.insert(i, 1);
    }

    let now = SystemTime::now();

    for card_number in 1..=(rounds.values().len() as u32) {
        for _ in 1..=*existing_copies.get(&card_number).unwrap_or(&1) {
            if let Some(copies_won) = rounds.get(&card_number) {
                if *copies_won == 0 {
                    continue;
                }
                for x in (card_number + 1)..=(card_number + *copies_won) {
                    if let Some(value) = existing_copies.get(&x) {
                        existing_copies.insert(x, *value + 1);
                    }
                }
            }
        }
    }
    let sum = existing_copies.values().sum::<i32>();

    match now.elapsed() {
        Ok(elapsed) => {
            // it prints '2'
            println!("{}", elapsed.as_millis());
        }
        Err(e) => {
            // an error occurred!
            println!("Error: {e:?}");
        }
    }
    dbg!(sum);
    dbg!(result);
}
