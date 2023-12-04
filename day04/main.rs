fn main() {
    let contents = include_str!("./input.txt");
    let lines = contents.split('\n');

    let mut result = 0;
    for line in lines {
        if line.is_empty() {
            continue;
        }

        let mut split = line.split(':');
        let game = split.nth(1).unwrap();

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

        if matches > 0 {
            result += 2_u32.pow(matches - 1);
        }
    }

    dbg!(result);
}
