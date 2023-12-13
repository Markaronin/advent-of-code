use std::cmp::Ordering;

use advent_of_code_util::{base_aoc, parse::read_lines};
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
enum Card1 {
    Ace,
    King,
    Queen,
    Jack,
    T,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
}
impl From<char> for Card1 {
    fn from(value: char) -> Self {
        match value {
            'A' => Card1::Ace,
            'K' => Card1::King,
            'Q' => Card1::Queen,
            'J' => Card1::Jack,
            'T' => Card1::T,
            '9' => Card1::Nine,
            '8' => Card1::Eight,
            '7' => Card1::Seven,
            '6' => Card1::Six,
            '5' => Card1::Five,
            '4' => Card1::Four,
            '3' => Card1::Three,
            '2' => Card1::Two,
            _ => panic!("Invalid card type {value}"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Hash)]
enum Card2 {
    Ace,
    King,
    Queen,
    T,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    Joker,
}

impl From<char> for Card2 {
    fn from(value: char) -> Self {
        match value {
            'A' => Card2::Ace,
            'K' => Card2::King,
            'Q' => Card2::Queen,
            'T' => Card2::T,
            '9' => Card2::Nine,
            '8' => Card2::Eight,
            '7' => Card2::Seven,
            '6' => Card2::Six,
            '5' => Card2::Five,
            '4' => Card2::Four,
            '3' => Card2::Three,
            '2' => Card2::Two,
            'J' => Card2::Joker,
            _ => panic!("Invalid card type {value}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Hand<T> {
    pub cards: Vec<T>,
    pub bid: usize,
}
impl<T: Clone + Eq + std::hash::Hash + From<char>> Hand<T> {
    pub fn from_line(line: &str) -> Self {
        let mut line = line.split_ascii_whitespace();
        let cards = line
            .next()
            .unwrap()
            .chars()
            .map(|c| T::from(c))
            .collect_vec();
        let bid = line.next().unwrap().parse::<usize>().unwrap();
        Self { cards, bid }
    }

    pub fn hand_type(&self) -> HandType {
        let counts = self.cards.clone().into_iter().counts();
        let mut counts = counts.values().collect_vec();
        counts.sort_by(|a, b| b.cmp(a));
        if *counts[0] == 5 {
            HandType::FiveOfAKind
        } else if *counts[0] == 4 {
            HandType::FourOfAKind
        } else if *counts[0] == 3 && *counts[1] == 2 {
            HandType::FullHouse
        } else if *counts[0] == 3 {
            HandType::ThreeOfAKind
        } else if *counts[0] == 2 && *counts[1] == 2 {
            HandType::TwoPair
        } else if *counts[0] == 2 {
            HandType::OnePair
        } else {
            HandType::HighCard
        }
    }
}

impl PartialOrd for Hand<Card1> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let ord = self.hand_type().partial_cmp(&other.hand_type()).unwrap();
        match &ord {
            Ordering::Less => Some(ord),
            Ordering::Equal => self.cards.partial_cmp(&other.cards),
            Ordering::Greater => Some(ord),
        }
    }
}
impl Ord for Hand<Card1> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for Hand<Card2> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let ord = best_hand_with_joker(self)
            .hand_type()
            .partial_cmp(&best_hand_with_joker(other).hand_type())
            .unwrap();
        match &ord {
            Ordering::Less => Some(ord),
            Ordering::Equal => self.cards.partial_cmp(&other.cards),
            Ordering::Greater => Some(ord),
        }
    }
}
impl Ord for Hand<Card2> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn best_hand_with_joker(hand: &Hand<Card2>) -> Hand<Card2> {
    let non_joker_cards = vec![
        Card2::Ace,
        Card2::King,
        Card2::Queen,
        Card2::T,
        Card2::Nine,
        Card2::Eight,
        Card2::Seven,
        Card2::Six,
        Card2::Five,
        Card2::Four,
        Card2::Three,
        Card2::Two,
    ];
    let joker_indexes = hand
        .cards
        .iter()
        .enumerate()
        .filter(|(_, card)| **card == Card2::Joker)
        .map(|(i, _)| i)
        .collect_vec();

    let mut possible_hands = non_joker_cards
        .into_iter()
        .combinations_with_replacement(joker_indexes.len())
        .map(|joker_replacements| {
            let mut new_cards = hand.cards.clone();
            for (i, card) in joker_indexes.iter().zip_eq(joker_replacements) {
                new_cards[*i] = card;
            }
            Hand::<Card2> {
                cards: new_cards,
                bid: hand.bid,
            }
        })
        .collect_vec();
    possible_hands.sort();

    possible_hands.into_iter().next().unwrap()
}

fn get_program_output(input_file: &str) -> (usize, usize) {
    let input = read_lines(input_file);

    let mut result_1 = input
        .clone()
        .into_iter()
        .map(|line| Hand::<Card1>::from_line(&line))
        .collect_vec();
    result_1.sort_by(|a, b| b.cmp(a));
    let result_1 = result_1
        .into_iter()
        .enumerate()
        .map(|(i, hand)| hand.bid * (i + 1))
        .sum::<usize>();

    let mut result_2 = input
        .clone()
        .into_iter()
        .map(|line| Hand::<Card2>::from_line(&line))
        .collect_vec();
    result_2.sort_by(|a, b| b.cmp(a));
    println!("{result_2:#?}");
    let result_2 = result_2
        .into_iter()
        .enumerate()
        .map(|(i, hand)| hand.bid * (i + 1))
        .sum::<usize>();

    (result_1, result_2)
}

base_aoc!(6440, 5905);
