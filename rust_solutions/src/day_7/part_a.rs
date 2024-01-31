use crate::{generate_puzzle_input_test, generate_test_input_test, utils};

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
enum CardType {
    Two,
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

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
enum HandType {
    Unknown(Vec<CardType>),
    HighCard(Vec<CardType>),
    OnePair(Vec<CardType>),
    TwoPair(Vec<CardType>),
    ThreeOfAKind(Vec<CardType>),
    FullHouse(Vec<CardType>),
    FourOfAKind(Vec<CardType>),
    FiveOfAKind(Vec<CardType>),
}

#[derive(Debug)]
struct Hand {
    hand_type: HandType,
    bid: u64,
}

impl Hand {
    fn new(cards: &str, bid: u64) -> Self {
        // TODO: Determine hand type
        let mut card_counts: Vec<u8> = vec![0; 13]; // Maps from 2 - A
        let mut card_types: Vec<CardType> = Vec::with_capacity(5);
        for card in cards.split("") {
            match card {
                "2" => {
                    card_counts[0] += 1;
                    card_types.push(CardType::Two)
                }
                "3" => {
                    card_counts[1] += 1;
                    card_types.push(CardType::Three)
                }
                "4" => {
                    card_counts[2] += 1;
                    card_types.push(CardType::Four)
                }
                "5" => {
                    card_counts[3] += 1;
                    card_types.push(CardType::Five)
                }
                "6" => {
                    card_counts[4] += 1;
                    card_types.push(CardType::Six)
                }
                "7" => {
                    card_counts[5] += 1;
                    card_types.push(CardType::Seven)
                }
                "8" => {
                    card_counts[6] += 1;
                    card_types.push(CardType::Eight)
                }
                "9" => {
                    card_counts[7] += 1;
                    card_types.push(CardType::Nine)
                }
                "T" => {
                    card_counts[8] += 1;
                    card_types.push(CardType::Ten)
                }
                "J" => {
                    card_counts[9] += 1;
                    card_types.push(CardType::Jack)
                }
                "Q" => {
                    card_counts[10] += 1;
                    card_types.push(CardType::Queen)
                }
                "K" => {
                    card_counts[11] += 1;
                    card_types.push(CardType::King);
                }
                "A" => {
                    card_counts[12] += 1;
                    card_types.push(CardType::Ace);
                }
                _ => continue,
            }
        }
        // Convert cards to a vec of card types
        card_counts = card_counts.clone().into_iter().filter(|&x| x > 0).collect();
        card_counts.sort();

        let hand_type = match card_counts {
            _ if card_counts == [2, 3] => HandType::FullHouse(card_types),
            _ if card_counts == [5] => HandType::FiveOfAKind(card_types),
            _ if card_counts == [1, 4] => HandType::FourOfAKind(card_types),
            _ if card_counts == [1, 1, 3] => HandType::ThreeOfAKind(card_types),
            _ if card_counts == [1, 2, 2] => HandType::TwoPair(card_types),
            _ if card_counts == [1, 1, 1, 2] => HandType::OnePair(card_types),
            _ if card_counts == [1, 1, 1, 1, 1] => HandType::HighCard(card_types),
            _ => HandType::Unknown(card_types),
        };

        Self { hand_type, bid }
    }

    fn cards(&self) -> &Vec<CardType> {
        match self.hand_type {
            HandType::Unknown(ref cards) => cards,
            HandType::HighCard(ref cards) => cards,
            HandType::OnePair(ref cards) => cards,
            HandType::TwoPair(ref cards) => cards,
            HandType::ThreeOfAKind(ref cards) => cards,
            HandType::FourOfAKind(ref cards) => cards,
            HandType::FiveOfAKind(ref cards) => cards,
            HandType::FullHouse(ref cards) => cards,
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand_type == other.hand_type
    }
}

impl Eq for Hand {}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let hand_type_cmp = self.hand_type.partial_cmp(&other.hand_type);
        if hand_type_cmp.unwrap() == std::cmp::Ordering::Equal {
            for (card, other) in self.cards().iter().zip(other.cards()) {
                if card > other {
                    return Some(std::cmp::Ordering::Greater);
                } else if card < other {
                    return Some(std::cmp::Ordering::Less);
                }
            }
        }

        hand_type_cmp
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let hand_type_cmp = self.hand_type.cmp(&other.hand_type);
        if hand_type_cmp == std::cmp::Ordering::Equal {
            for (card, other) in self.cards().iter().zip(other.cards()) {
                if card > other {
                    return std::cmp::Ordering::Greater;
                } else if card < other {
                    return std::cmp::Ordering::Less;
                }
            }
        }

        hand_type_cmp
    }
}

pub fn solve(file: &str) -> u64 {
    let lines = utils::read_file(file);
    let mut hands: Vec<Hand> = vec![];
    for line in lines {
        let l: Vec<&str> = line.split(' ').collect();
        hands.push(Hand::new(l[0], l[1].parse::<u64>().unwrap()));
    }

    hands.sort();

    (1..=hands.len())
        .map(|rank| rank as u64 * hands[rank - 1].bid)
        .sum()
}

generate_test_input_test!(7, 6440);
generate_puzzle_input_test!(7, 248179786);
