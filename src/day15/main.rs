use std::collections::HashMap;

fn main() {
    let contents = include_str!("./input.txt");
    dbg!(part_1(contents));
    dbg!(part_2(contents));
}

fn part_1(contents: &str) -> u32 {
    contents
        .split(',')
        .map(|s| {
            let mut value = 0;
            for c in s.chars() {
                value += c as u32;
                value *= 17;
                value %= 256;
            }

            value
        })
        .sum()
}

fn part_2(contents: &str) -> usize {
    let mut lens_boxes: HashMap<u32, Vec<(Vec<char>, u32)>> = HashMap::new();
    (0..256).for_each(|i| {
        lens_boxes.insert(i, vec![]);
    });

    contents.split(',').for_each(|s| {
        let focus_power: u32 = match s.split('=').nth(1) {
            None => 0,
            Some(v) => v.parse().unwrap(),
        };

        let is_insert = s.contains('=');

        let lens_name = s
            .chars()
            .take_while(|c| c.ne(&'=') && c.ne(&'-'))
            .collect::<Vec<char>>();

        let mut box_id = 0;
        for c in lens_name.iter() {
            box_id += *c as u32;
            box_id *= 17;
            box_id %= 256;
        }

        if let Some(lens_box) = lens_boxes.get(&box_id) {
            let mut replaced = false;
            let mut new_lens_box = lens_box
                .iter()
                .filter_map(|(lens, focus)| {
                    if is_insert {
                        if lens.eq(&lens_name) {
                            replaced = true;
                            return Some((lens_name.to_owned(), focus_power));
                        }
                    } else if lens.eq(&lens_name) {
                        return None;
                    }

                    Some((lens.to_owned(), focus.to_owned()))
                })
                .collect::<Vec<(Vec<char>, u32)>>();

            if is_insert && !replaced {
                new_lens_box.push((lens_name, focus_power));
            }

            lens_boxes.insert(box_id, new_lens_box);
        } else {
            panic!("No box found {box_id}");
        }
    });

    let mut sum = 0;
    for (box_id, lens_box) in lens_boxes {
        if lens_box.is_empty() {
            continue;
        }
        for (idx, (_, power)) in lens_box.iter().enumerate() {
            sum += (box_id as usize + 1) * (idx + 1) * *power as usize;
        }
    }

    sum
}
