fn main() {
    let contents = include_str!("./input.txt");
    dbg!(part_1(contents));
    dbg!(part_2(contents));
}

fn part_1(contents: &str) -> usize {
    let mut lines = contents.split('\n');

    let times = lines
        .next()
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .split(' ')
        .filter_map(|c| c.parse::<u32>().ok())
        .collect::<Vec<u32>>();

    let distances = lines
        .next()
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .split(' ')
        .filter_map(|c| c.parse::<u32>().ok())
        .collect::<Vec<u32>>();

    distances
        .iter()
        .zip(times.iter())
        .map(|(distance, time)| {
            (1..*time)
                .filter(|passed_time| {
                    let total_distance = passed_time * (time - passed_time);
                    total_distance > *distance
                })
                .count()
        })
        .product::<usize>()
}

fn part_2(contents: &str) -> usize {
    let mut lines = contents.split('\n');

    let time = lines
        .next()
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .replace(' ', "")
        .parse::<u64>()
        .unwrap();

    let distance = lines
        .next()
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .replace(' ', "")
        .parse::<u64>()
        .unwrap();

    (1..time)
        .filter(|passed_time| {
            let total_distance = passed_time * (time - passed_time);
            total_distance > distance
        })
        .count()
}
