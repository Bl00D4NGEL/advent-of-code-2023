use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Symbol {
    pub id: usize,
    position: Position,
    value: char,
}

#[derive(Debug, Clone)]
struct Position {
    x: usize,
    y: usize,
}

impl Symbol {
    fn is_gear(&self) -> bool {
        self.value == '*'
    }

    fn is_adjecant_to(&self, pos: Position) -> bool {
        pos.x.abs_diff(self.position.x) < 2 && pos.y.abs_diff(self.position.y) < 2
    }
}

fn main() {
    let contents = include_str!("./input.txt");
    let lines = contents.split('\n');

    let mut symbols = vec![];

    let mut symbol_to_counted_numbers: HashMap<usize, Vec<(String, (usize, usize))>> =
        HashMap::new();

    for (line_idx, line) in lines.clone().enumerate() {
        if line.is_empty() {
            continue;
        }

        for (char_idx, character) in line.chars().enumerate() {
            if !character.is_ascii_digit() && character != '.' {
                symbols.push(Symbol {
                    id: symbols.len(),
                    position: Position {
                        x: line_idx,
                        y: char_idx,
                    },
                    value: character,
                });
            }
        }
    }

    let mut counted_numbers = vec![];
    for (line_idx, line) in lines.enumerate() {
        let mut current_number = String::new();
        let mut adjacent_symbols = vec![];
        for (char_idx, character) in line.chars().enumerate() {
            if character.is_ascii_digit() {
                if current_number.is_empty() {
                    // Starting new number here
                    current_number = character.to_string();
                } else {
                    // Already counding number
                    current_number.extend([character.to_string()]);
                }

                let filtered_symbols = symbols.iter().filter(|symbol| {
                    symbol.is_adjecant_to(Position {
                        x: line_idx,
                        y: char_idx,
                    })
                });

                for symbol in filtered_symbols {
                    adjacent_symbols.push(symbol);
                }
            } else {
                if !current_number.is_empty() && !adjacent_symbols.is_empty() {
                    counted_numbers.push(current_number.clone());

                    for symbol in adjacent_symbols {
                        if !symbol.is_gear() {
                            continue;
                        }
                        if let Some(counter) = symbol_to_counted_numbers.get_mut(&symbol.id) {
                            counter.push((current_number.clone(), (line_idx, char_idx)));
                        } else {
                            symbol_to_counted_numbers.insert(
                                symbol.id,
                                vec![(current_number.clone(), (line_idx, char_idx))],
                            );
                        }
                    }
                }
                current_number = String::new();
                adjacent_symbols = vec![];
            }
        }

        if !current_number.is_empty() && !adjacent_symbols.is_empty() {
            counted_numbers.push(current_number.clone());

            for symbol in adjacent_symbols {
                if !symbol.is_gear() {
                    continue;
                }
                if let Some(counter) = symbol_to_counted_numbers.get_mut(&symbol.id) {
                    counter.push((current_number.clone(), (line_idx, line.len())));
                } else {
                    symbol_to_counted_numbers.insert(
                        symbol.id,
                        vec![(current_number.clone(), (line_idx, line.len()))],
                    );
                }
            }
        }
    }

    let result_2: u32 = symbol_to_counted_numbers
        .values()
        .map(|values| {
            let mut c = values.clone();
            c.sort();
            c.dedup();
            if c.len() != 2 {
                return 0;
            }
            c.iter()
                .map(|(n, _)| n.parse::<u32>().unwrap())
                .product::<u32>()
        })
        .sum();

    let result: u32 = counted_numbers
        .iter()
        .map(|n| n.parse::<u32>().unwrap())
        .sum();

    dbg!(result);
    dbg!(result_2);
}
