use std::{cmp::Ordering, collections::HashMap, u8};

fn main() {
    let contents = include_str!("./input.txt");
    dbg!(part_1(contents));
    dbg!(part_2(contents));
}

fn part_1(contents: &str) -> u32 {
    let lines = contents.split('\n');
    let mut hands = lines.filter_map(parse_hand).collect::<Vec<Hand>>();

    hands.sort_by(|a, b| a.compare_against(b));

    let mut acc = 0;

    for (idx, hand) in hands.iter().enumerate() {
        let bid = (idx + 1) as u32 * hand.bet;
        acc += bid;
    }

    acc
}

#[derive(Debug, Clone)]
struct Card {
    value: u8,
    char: char,
}

impl Card {
    fn from_char(input: char) -> Result<Card, ()> {
        let value = match input {
            '2' => 12,
            '3' => 11,
            '4' => 10,
            '5' => 9,
            '6' => 8,
            '7' => 7,
            '8' => 6,
            '9' => 5,
            'T' => 4,
            'J' => 3,
            'Q' => 2,
            'K' => 1,
            'A' => 0,
            _ => {
                return Err(());
            }
        };

        Ok(Card { value, char: input })
    }
}

#[derive(Debug, Clone)]
struct Hand {
    cards: Vec<Card>,
    bet: u32,
}

impl Hand {
    fn compare_against(&self, other_hand: &Hand) -> Ordering {
        if other_hand.determine_hand_value() > self.determine_hand_value() {
            return Ordering::Greater;
        }

        if other_hand.determine_hand_value() < self.determine_hand_value() {
            return Ordering::Less;
        }

        // Hands have same value, iterate over cards
        let zipped = other_hand.cards.iter().zip(self.cards.iter());

        for (other, card) in zipped {
            if other.value > card.value {
                return Ordering::Greater;
            }

            if card.value > other.value {
                return Ordering::Less;
            }
        }

        Ordering::Equal
    }

    fn print_hand(&self) {
        let value_string: String = self.cards.iter().map(|c| c.value.to_string()).collect();

        println!("{} has value {}", self.cards_as_string(), value_string);
    }

    fn cards_as_string(&self) -> String {
        self.cards.iter().map(|c| c.char.to_string()).collect()
    }

    fn determine_hand_value(&self) -> u8 {
        if self.is_five_of_a_kind() {
            0
        } else if self.is_four_of_a_kind() {
            1
        } else if self.is_full_house() {
            2
        } else if self.is_three_of_a_kind() {
            3
        } else if self.is_two_pair() {
            4
        } else if self.is_pair() {
            5
        } else {
            6
        }
    }

    fn is_five_of_a_kind(&self) -> bool {
        let grouped = self.group_cards();

        grouped.keys().len() == 1
    }

    fn group_cards(&self) -> HashMap<u8, u8> {
        let mut cards = HashMap::new();
        for card in self.cards.iter() {
            if let Some(prev_value) = cards.get(&card.value) {
                cards.insert(card.value, *prev_value + 1);
            } else {
                cards.insert(card.value, 1);
            }
        }

        cards
    }

    fn is_four_of_a_kind(&self) -> bool {
        let grouped = self.group_cards();

        for value in grouped.values() {
            if *value == 4 {
                return true;
            }
        }

        false
    }

    fn is_full_house(&self) -> bool {
        let mut has_three = false;
        let mut has_pair = false;

        let grouped = self.group_cards();
        for value in grouped.values() {
            if *value == 3 {
                has_three = true;
            }

            if *value == 2 {
                has_pair = true;
            }
        }

        has_pair && has_three
    }

    fn is_three_of_a_kind(&self) -> bool {
        let grouped = self.group_cards();
        for value in grouped.values() {
            if *value == 3 {
                return true;
            }
        }
        false
    }

    fn is_two_pair(&self) -> bool {
        let mut found_pairs = 0;
        let grouped = self.group_cards();
        for value in grouped.values() {
            if *value == 2 {
                found_pairs += 1;
            }
        }

        found_pairs == 2
    }

    fn is_pair(&self) -> bool {
        let mut found_pairs = 0;
        let grouped = self.group_cards();
        for value in grouped.values() {
            if *value == 2 {
                found_pairs += 1;
            }
        }

        found_pairs == 1
    }

    fn best_card(&self) -> u8 {
        self.cards.iter().map(|c| c.value).min().unwrap()
    }
}

fn parse_hand(line: &str) -> Option<Hand> {
    if line.is_empty() {
        return None;
    }

    let mut split = line.split(' ');
    let cards = split
        .next()?
        .chars()
        .filter_map(|c| Card::from_char(c).ok())
        .collect::<Vec<Card>>();

    if cards.len() != 5 {
        println!(
            "Expected cards to have length 5. Instead they have length of {:?}. Skipping",
            cards.len()
        );
        return None;
    }

    let bet = split.next()?.parse::<u32>().ok()?;

    Some(Hand { cards, bet })
}

fn part_2(contents: &str) -> usize {
    0
}
