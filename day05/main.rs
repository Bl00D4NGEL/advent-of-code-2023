use rayon::prelude::*;
use std::collections::HashMap;

type SourceNumberStart = u64;
type DestinationNumberStart = u64;
type RangeLength = u64;

fn main() {
    let contents = include_str!("./input.txt");
    let mut lines = contents.split('\n');

    let seed_line = lines.next().unwrap();
    let seeds = seed_line
        .split(':')
        .nth(1)
        .unwrap()
        .split(' ')
        .filter_map(|c| c.parse::<u64>().ok())
        .collect::<Vec<u64>>();

    let mut maps: HashMap<&str, Vec<(SourceNumberStart, DestinationNumberStart, RangeLength)>> =
        HashMap::new();
    let mut current_map_name = "";
    let mut map_order = vec![];
    for line in lines {
        if line.is_empty() {
            continue;
        }

        if line.ends_with("map:") {
            current_map_name = line;
            maps.insert(line, vec![]);
            map_order.push(current_map_name);
            continue;
        }

        let current_map = match maps.get_mut(current_map_name) {
            None => {
                println!("Could not find map {current_map_name}");
                continue;
            }
            Some(v) => v,
        };

        let split = line
            .split(' ')
            .filter_map(|c| c.parse::<u64>().ok())
            .collect::<Vec<u64>>();

        if split.len() != 3 {
            println!("Split length is not 3");
            dbg!(split);
            continue;
        }

        let destination_start = *split.first().unwrap();
        let source_start = *split.get(1).unwrap();
        let range_length = *split.get(2).unwrap();

        current_map.push((source_start, destination_start, range_length));
    }

    let locations = seeds
        .iter()
        .map(|seed| determine_final_seed_number(*seed, &map_order, &maps));

    let result = locations.min();

    dbg!(result);

    let mut seed_chunks = seeds.chunks(2);
    let mut mins = vec![];
    while let Some([start_seed, length]) = seed_chunks.next() {
        let minimum_for_range = (*start_seed..(*start_seed + *length))
            .into_par_iter()
            .map(|seed| determine_final_seed_number(seed, &map_order, &maps))
            .min();

        mins.push(minimum_for_range);
    }

    dbg!(mins.iter().min());
}

fn determine_final_seed_number(
    initial_seed: u64,
    map_order: &Vec<&str>,
    maps: &HashMap<&str, Vec<(SourceNumberStart, DestinationNumberStart, RangeLength)>>,
) -> u64 {
    let mut seed_number = initial_seed;

    for map_name in map_order {
        let ranges = maps.get(map_name).unwrap();

        for (source_start, destination_start, range_length) in ranges {
            let source_end = source_start + range_length;
            if seed_number < source_end && seed_number >= *source_start {
                let new_seed = destination_start + seed_number - source_start;
                seed_number = new_seed;
                break;
            }
        }
    }

    seed_number
}
