fn main() {
    let contents = include_str!("./input.txt");
    let lines = contents.split('\n');

    let mut result = 0;

    for line in lines {
        if line.is_empty() {
            continue;
        }

        let mut split = line.split(':');
        let rounds = split.nth(1).unwrap_or("").trim();

        let mut max_green = 1;
        let mut max_red = 1;
        let mut max_blue = 1;
        for round in rounds.split(';') {
            for color in round.split(',') {
                let mut split = color.trim().split(' ');
                let amount = split.next().unwrap_or("0").parse::<u32>().unwrap_or(0);
                let color_name = split.next().unwrap_or("");

                match color_name {
                    "red" => {
                        max_red = max_red.max(amount);
                    }
                    "blue" => {
                        max_blue = max_blue.max(amount);
                    }
                    "green" => {
                        max_green = max_green.max(amount);
                    }
                    _ => {}
                };
            }
        }

        let power = max_red * max_green * max_blue;
        result += power;
    }

    dbg!(result);
}
