use std::{cmp::Ordering, collections::HashMap, u8};

fn main() {
    let contents = include_str!("./input.txt");
    dbg!(part_1(contents));
    dbg!(part_2(contents));
}

fn part_1(contents: &str) -> u32 {
    let lines = contents.split('\n');
    let mut hands = lines.filter_map(parse_hand_part_1).collect::<Vec<Hand>>();

    hands.sort_by(|a, b| a.compare_against(b));

    let mut acc = 0;

    for (idx, hand) in hands.iter().enumerate() {
        let bid = (idx + 1) as u32 * hand.bet;
        acc += bid;
    }

    acc
}

fn part_2(contents: &str) -> u32 {
    let lines = contents.split('\n');
    let mut hands = lines.filter_map(parse_hand_part_2).collect::<Vec<Hand>>();

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
    is_joker: bool,
    char: char,
}

#[derive(Debug, Clone)]
struct Hand {
    cards: Vec<Card>,
    bet: u32,
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug)]
enum HandValues {
    FiveOfAKind = 0,
    FourOfAKind = 1,
    FullHouse = 2,
    ThreeOfAKind = 3,
    TwoPair = 4,
    OnePair = 5,
    HighestCard = 6,
}

impl Hand {
    fn compare_against(&self, other_hand: &Hand) -> Ordering {
        match other_hand
            .determine_hand_value()
            .cmp(&self.determine_hand_value())
        {
            Ordering::Greater => return Ordering::Greater,
            Ordering::Less => return Ordering::Less,
            _ => (),
        }

        // Hands have same value, iterate over cards
        let zipped = other_hand.cards.iter().zip(self.cards.iter());

        for (other, card) in zipped {
            match other.value.cmp(&card.value) {
                Ordering::Greater => return Ordering::Greater,
                Ordering::Less => return Ordering::Less,
                _ => continue,
            }
        }

        Ordering::Equal
    }

    fn cards_as_string(&self) -> String {
        self.cards.iter().map(|c| c.char.to_string()).collect()
    }

    fn determine_hand_value(&self) -> HandValues {
        let mut grouped: HashMap<u8, (u8, &Card)> = HashMap::new();

        for card in self.cards.iter() {
            if let Some((prev_value, prev_card)) = grouped.get(&card.value) {
                grouped.insert(card.value, (*prev_value + 1, prev_card));
            } else {
                grouped.insert(card.value, (1, card));
            }
        }

        let joker_count = self.joker_counter();
        let max_count = *grouped
            .iter()
            .filter(|(_, (_, c))| !c.is_joker)
            .map(|(_, (v, _))| v)
            .max()
            .unwrap_or(&0);

        if joker_count == 0 && max_count == 1 {
            return HandValues::HighestCard;
        }

        if joker_count + max_count == 5 {
            return HandValues::FiveOfAKind;
        }

        if joker_count + max_count == 4 {
            return HandValues::FourOfAKind;
        }

        if joker_count == 0 && max_count == 3 && grouped.values().any(|(v, _)| *v == 2) {
            return HandValues::FullHouse;
        }

        if joker_count == 1
            && max_count == 2
            && grouped.values().filter(|(v, _)| *v == 2).count() == 2
        {
            return HandValues::FullHouse;
        }

        if joker_count + max_count == 3 {
            return HandValues::ThreeOfAKind;
        }

        if joker_count == 1
            && max_count == 2
            && grouped.values().filter(|(v, _)| *v == 2).count() == 1
        {
            return HandValues::ThreeOfAKind;
        }

        if joker_count == 0 && grouped.values().filter(|(v, _)| *v == 2).count() == 2 {
            return HandValues::TwoPair;
        }

        if joker_count == 1 && max_count == 2 && grouped.values().any(|(v, _)| *v == 2) {
            return HandValues::TwoPair;
        }

        HandValues::OnePair
    }

    fn joker_counter(&self) -> u8 {
        self.cards.iter().filter(|c| c.is_joker).count() as u8
    }
}

fn parse_hand_part_1(line: &str) -> Option<Hand> {
    if line.is_empty() {
        return None;
    }

    let mut split = line.split(' ');
    let cards = split
        .next()?
        .chars()
        .filter_map(|char| {
            let value = match char {
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
                    return None;
                }
            };

            Some(Card {
                value,
                char,
                is_joker: false,
            })
        })
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

fn parse_hand_part_2(line: &str) -> Option<Hand> {
    if line.is_empty() {
        return None;
    }

    let mut split = line.split(' ');
    let cards = split
        .next()?
        .chars()
        .filter_map(|char| {
            let value = match char {
                'J' => 12,
                '2' => 11,
                '3' => 10,
                '4' => 9,
                '5' => 8,
                '6' => 7,
                '7' => 6,
                '8' => 5,
                '9' => 4,
                'T' => 3,
                'Q' => 2,
                'K' => 1,
                'A' => 0,
                _ => {
                    return None;
                }
            };

            Some(Card {
                value,
                char,
                is_joker: char == 'J',
            })
        })
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
