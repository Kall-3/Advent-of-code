use crate::problem::Problem;

pub struct DaySeven;

use regex::Regex;
use std::{cmp::Ordering, collections::HashMap};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
enum Card {
    Two = 0,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Card {
    fn from_char(c: char) -> Option<Card> {
        match c {
            '2' => Some(Card::Two),
            '3' => Some(Card::Three),
            '4' => Some(Card::Four),
            '5' => Some(Card::Five),
            '6' => Some(Card::Six),
            '7' => Some(Card::Seven),
            '8' => Some(Card::Eight),
            '9' => Some(Card::Nine),
            'T' => Some(Card::Ten),
            'J' => Some(Card::Jack),
            'Q' => Some(Card::Queen),
            'K' => Some(Card::King),
            'A' => Some(Card::Ace),
            _ => None,
        }
    }
}

#[derive(Eq, Debug)]
struct Hand {
    cards: Vec<Card>,
    power: u32,
    bet: u32,
}

impl Hand {
    fn new(cards: Vec<Card>, bet: u32) -> Hand {
        let power = Self::calculate_power(&cards);

        Hand { cards, power, bet }
    }

    fn calculate_power(cards: &Vec<Card>) -> u32 {
        // Placeholder for actual hand power calculation
        let mut map: HashMap<Card, u8> = HashMap::new();

        for &card in cards {
            *map.entry(card).or_insert(0) += 1;
        }

        if map.len() == 1 {
            // Five of a kind
            6
        } else if map.len() == 2 && map.values().any(|&count| count == 4) {
            // Four of a kind
            5
        } else if map.len() == 2 && map.values().any(|&count| count == 3) {
            // Full house
            4
        } else if map.values().any(|&count| count == 3) {
            // Three of a kind
            3
        } else if map.len() == 3 && map.values().filter(|&&count| count == 2).count() == 2 {
            // Two pair
            2
        } else if map.len() == 4 && map.values().any(|&count| count == 2) {
            // One pair
            1
        } else {
            // High card
            0
        }

    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Hand) -> Ordering {
        self.power.cmp(&other.power)
            .then(self.cards.cmp(&other.cards))
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Hand) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Hand) -> bool {
        self.power == other.power && self.cards == other.cards
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
enum CardWithJoker {
    Joker = 0,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Ace,
}

impl CardWithJoker {
    fn from_char(c: char) -> Option<CardWithJoker> {
        match c {
            'J' => Some(CardWithJoker::Joker),
            '2' => Some(CardWithJoker::Two),
            '3' => Some(CardWithJoker::Three),
            '4' => Some(CardWithJoker::Four),
            '5' => Some(CardWithJoker::Five),
            '6' => Some(CardWithJoker::Six),
            '7' => Some(CardWithJoker::Seven),
            '8' => Some(CardWithJoker::Eight),
            '9' => Some(CardWithJoker::Nine),
            'T' => Some(CardWithJoker::Ten),
            'Q' => Some(CardWithJoker::Queen),
            'K' => Some(CardWithJoker::King),
            'A' => Some(CardWithJoker::Ace),
            _ => None,
        }
    }
}

#[derive(Eq, Debug)]
struct HandWithJoker {
    cards: Vec<CardWithJoker>,
    power: u32,
    bet: u32,
}

impl HandWithJoker {
    fn new(cards: Vec<CardWithJoker>, bet: u32) -> HandWithJoker {
        let power = Self::calculate_power(&cards);

        HandWithJoker { cards, power, bet }
    }

    fn calculate_power(cards: &Vec<CardWithJoker>) -> u32 {
        // Placeholder for actual hand power calculation
        let mut map: HashMap<CardWithJoker, u8> = HashMap::new();

        for &card in cards {
            *map.entry(card).or_insert(0) += 1;
        }

        let joker_count = map.remove(&CardWithJoker::Joker).unwrap_or(0);

        if joker_count == 5 {
            // Five jokers
            return 6;
        } else if map.values().any(|&count| count + joker_count >= 5) {
            // Five of a kind
            6
        } else if map.values().any(|&count| count + joker_count >= 4) {
            // Four of a kind
            5
        } else if map.len() == 2 {
            // Full house
            4
        } else if map.values().any(|&count| count + joker_count >= 3) {
            // Three of a kind
            3
        } else if map.len() == 3 {
            // Two pair
            2
        } else if map.values().any(|&count| count + joker_count >= 2) {
            // One pair
            1
        } else {
            // High card
            0
        }
    }
}

impl Ord for HandWithJoker {
    fn cmp(&self, other: &HandWithJoker) -> Ordering {
        self.power.cmp(&other.power)
            .then(self.cards.cmp(&other.cards))
    }
}

impl PartialOrd for HandWithJoker {
    fn partial_cmp(&self, other: &HandWithJoker) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for HandWithJoker {
    fn eq(&self, other: &HandWithJoker) -> bool {
        self.power == other.power && self.cards == other.cards
    }
}

impl Problem for DaySeven {
    fn part_one(&self, input: &str) -> String {
        let re = Regex::new(r"^(?P<cards>\w+)\s+(?P<bet>\d+)$").unwrap();

        let mut hands: Vec<Hand> = Vec::new();

        for line in input.lines() {
            let (cards, bet) = match re.captures(line) {
                Some(caps) => {
                    let cards_str = caps.name("cards").unwrap().as_str();
                    let bet_str = caps.name("bet").unwrap().as_str().parse::<u32>().expect("Invalid bet number");
                    (cards_str, bet_str)
                }
                None => {
                    println!("No match for line: {}", line);
                    continue;
                }
            };

            let mut cards_vec: Vec<Card> = Vec::new();

            for c in cards.chars() {
                let card: Card = Card::from_char(c).expect("Invalid card character");
                cards_vec.push(card);
            }

            let hand = Hand::new(cards_vec, bet);
            hands.push(hand);
        }

        hands.sort();

        let mut res: u32 = 0;
        for (i, hand) in hands.into_iter().enumerate() {
            res += (i as u32 + 1) * hand.bet;
        }

        format!("{}", res)
    }

    fn part_two(&self, input: &str) -> String {
        let re = Regex::new(r"^(?P<cards>\w+)\s+(?P<bet>\d+)$").unwrap();

        let mut hands: Vec<HandWithJoker> = Vec::new();

        for line in input.lines() {
            let (cards, bet) = match re.captures(line) {
                Some(caps) => {
                    let cards_str = caps.name("cards").unwrap().as_str();
                    let bet_str = caps.name("bet").unwrap().as_str().parse::<u32>().expect("Invalid bet number");
                    (cards_str, bet_str)
                }
                None => {
                    println!("No match for line: {}", line);
                    continue;
                }
            };

            let mut cards_vec: Vec<CardWithJoker> = Vec::new();

            for c in cards.chars() {
                let card: CardWithJoker = CardWithJoker::from_char(c).expect("Invalid card character");
                cards_vec.push(card);
            }

            let hand = HandWithJoker::new(cards_vec, bet);
            hands.push(hand);
        }

        hands.sort();

        let mut res: u32 = 0;

        for (i, hand) in hands.into_iter().enumerate() {
            res += (i as u32 + 1) * hand.bet;
        }

        format!("{}", res)
    }
}
