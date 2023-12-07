use std::{collections::HashMap, time::Instant};

use crate::util;

pub fn main() {
    let data = util::read_file("seven");
    let start = Instant::now();
    let one_result = part_one(&data);
    let duration = start.elapsed();
    println!("input one result: {}", one_result);
    println!("Execution time: {:?}", duration);
    let start = Instant::now();
    let two_result = part_two(&data);
    let duration = start.elapsed();
    println!("input two result: {}", two_result);
    println!("Execution time: {:?}", duration);
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum HandType {
    FiveKind,
    FourKind,
    FullHouse,
    ThreeKind,
    TwoPair,
    OnePair,
    HighCard,
    Unknown,
}

#[derive(Debug, Clone)]
struct Hand {
    cards: String,
    bid: usize,
    hand_type: HandType,
}

impl Hand {
    fn parse_type(&mut self, joker: bool) {
        let mut cards: HashMap<char, u8> = HashMap::new();
        let mut j_count = 0;
        for char in self.cards.chars() {
            if joker && char == 'J' {
                j_count += 1;
            } else {
                match cards.get(&char) {
                    Some(c) => cards.insert(char, *c + 1),
                    None => cards.insert(char, 1),
                };
            }
        }

        let mut card_counts: Vec<u8> = cards.values().map(|v| *v).collect();
        card_counts.sort();
        if j_count > 0 {
            if card_counts.len() == 0 {
                card_counts.push(5);
            } else {
                let cc = card_counts.clone();
                let last = cc.last().unwrap();
                let idx = cc.len() - 1;
                card_counts[idx] = last + j_count;
            }
        }
        if card_counts == vec![5] {
            return self.hand_type = HandType::FiveKind;
        }
        if card_counts == vec![1, 4] {
            return self.hand_type = HandType::FourKind;
        }
        if card_counts == vec![2, 3] {
            return self.hand_type = HandType::FullHouse;
        }
        if card_counts == vec![1, 1, 3] {
            return self.hand_type = HandType::ThreeKind;
        }
        if card_counts == vec![1, 2, 2] {
            return self.hand_type = HandType::TwoPair;
        }
        if card_counts == vec![1, 1, 1, 2] {
            return self.hand_type = HandType::OnePair;
        }
        return self.hand_type = HandType::HighCard;
    }
}

fn part_one(data: &Vec<String>) -> usize {
    let mut sum = 0;
    let mut hands = parse_input(data);
    hands.iter_mut().for_each(|h| h.parse_type(false));
    let sorted = sort_hands(hands, false);
    sorted.iter().enumerate().for_each(|(idx, h)| {
        sum += h.bid * (sorted.len() - idx);
    });
    sum
}

fn part_two(data: &Vec<String>) -> usize {
    let mut sum = 0;
    let mut hands = parse_input(data);
    hands.iter_mut().for_each(|h| h.parse_type(true));
    let sorted = sort_hands(hands, true);
    sorted.iter().enumerate().for_each(|(idx, h)| {
        sum += h.bid * (sorted.len() - idx);
    });
    sum
}

fn sort_hands(hands: Vec<Hand>, joker: bool) -> Vec<Hand> {
    let mut five: Vec<Hand> = vec![];
    let mut four: Vec<Hand> = vec![];
    let mut full: Vec<Hand> = vec![];
    let mut three: Vec<Hand> = vec![];
    let mut two: Vec<Hand> = vec![];
    let mut one: Vec<Hand> = vec![];
    let mut high: Vec<Hand> = vec![];

    hands.iter().for_each(|h| match h.hand_type {
        HandType::FiveKind => five.push(h.clone()),
        HandType::FourKind => four.push(h.clone()),
        HandType::FullHouse => full.push(h.clone()),
        HandType::ThreeKind => three.push(h.clone()),
        HandType::TwoPair => two.push(h.clone()),
        HandType::OnePair => one.push(h.clone()),
        HandType::HighCard => high.push(h.clone()),
        HandType::Unknown => panic!("Should not have any unknown"),
    });

    let five = sort_same_type(five, joker);
    let four = sort_same_type(four, joker);
    let full = sort_same_type(full, joker);
    let three = sort_same_type(three, joker);
    let two = sort_same_type(two, joker);
    let one = sort_same_type(one, joker);
    let high = sort_same_type(high, joker);

    [five, four, full, three, two, one, high]
        .into_iter()
        .flatten()
        .collect::<Vec<Hand>>()
}

fn sort_same_type(mut hands: Vec<Hand>, joker: bool) -> Vec<Hand> {
    if hands.len() < 2 {
        return hands;
    }
    let mut ranks: HashMap<char, u8> = HashMap::from([
        ('2', 2),
        ('3', 3),
        ('4', 4),
        ('5', 5),
        ('6', 6),
        ('7', 7),
        ('8', 8),
        ('9', 9),
        ('T', 10),
        ('J', 12),
        ('Q', 13),
        ('K', 14),
        ('A', 15),
    ]);
    if joker {
        ranks.insert('J', 1);
    }

    hands.sort_by(|a, b| {
        let mut a_ch = a.cards.chars().nth(0).unwrap();
        let mut b_ch = b.cards.chars().nth(0).unwrap();
        for (idx, _) in a.cards.chars().enumerate() {
            if a_ch != b_ch {
                break;
            } else {
                a_ch = a.cards.chars().nth(idx).unwrap();
                b_ch = b.cards.chars().nth(idx).unwrap();
            }
        }
        ranks.get(&b_ch).unwrap().cmp(ranks.get(&a_ch).unwrap())
    });
    return hands.clone();
}

fn parse_input(data: &Vec<String>) -> Vec<Hand> {
    data.iter()
        .map(|line| {
            let (hand, bid) = line.split_once(" ").unwrap();
            Hand {
                cards: String::from(hand.trim()),
                bid: bid.trim().parse::<usize>().unwrap(),
                hand_type: HandType::Unknown,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::day_seven::{Hand, HandType};

    use super::{part_one, part_two};

    #[test]
    fn part_one_test() {
        let input: Vec<String> = vec![
            "32T3K 765",
            "T55J5 684",
            "KK677 28",
            "KTJJT 220",
            "QQQJA 483",
        ]
        .iter()
        .map(|x| String::from(*x))
        .collect();
        let result = part_one(&input);
        assert_eq!(result, 6440);
    }

    #[test]
    fn part_two_test() {
        let input: Vec<String> = vec![
            "32T3K 765",
            "T55J5 684",
            "KK677 28",
            "KTJJT 220",
            "QQQJA 483",
        ]
        .iter()
        .map(|x| String::from(*x))
        .collect();
        let result = part_two(&input);
        assert_eq!(result, 5905);
    }
    #[test]
    fn cards_paring() {
        let mut hand = Hand {
            cards: String::from("AAAAA"),
            bid: 1,
            hand_type: HandType::Unknown,
        };
        hand.parse_type(true);
        assert_eq!(hand.hand_type, HandType::FiveKind);

        let mut hand = Hand {
            cards: String::from("AAJAA"),
            bid: 1,
            hand_type: HandType::Unknown,
        };
        hand.parse_type(true);
        assert_eq!(hand.hand_type, HandType::FiveKind);

        let mut hand = Hand {
            cards: String::from("9JJ99"),
            bid: 1,
            hand_type: HandType::Unknown,
        };
        hand.parse_type(true);
        assert_eq!(hand.hand_type, HandType::FiveKind);

        let mut hand = Hand {
            cards: String::from("9JJJ9"),
            bid: 1,
            hand_type: HandType::Unknown,
        };
        hand.parse_type(true);
        assert_eq!(hand.hand_type, HandType::FiveKind);

        let mut hand = Hand {
            cards: String::from("JJJJA"),
            bid: 1,
            hand_type: HandType::Unknown,
        };
        hand.parse_type(true);
        assert_eq!(hand.hand_type, HandType::FiveKind);

        let mut hand = Hand {
            cards: String::from("AKAAA"),
            bid: 1,
            hand_type: HandType::Unknown,
        };
        hand.parse_type(true);
        assert_eq!(hand.hand_type, HandType::FourKind);

        let mut hand = Hand {
            cards: String::from("KAJAA"),
            bid: 1,
            hand_type: HandType::Unknown,
        };
        hand.parse_type(true);
        assert_eq!(hand.hand_type, HandType::FourKind);

        let mut hand = Hand {
            cards: String::from("KAJJA"),
            bid: 1,
            hand_type: HandType::Unknown,
        };
        hand.parse_type(true);
        assert_eq!(hand.hand_type, HandType::FourKind);

        let mut hand = Hand {
            cards: String::from("KAJJJ"),
            bid: 1,
            hand_type: HandType::Unknown,
        };
        hand.parse_type(true);
        assert_eq!(hand.hand_type, HandType::FourKind);

        let mut hand = Hand {
            cards: String::from("A88AA"),
            bid: 1,
            hand_type: HandType::Unknown,
        };
        hand.parse_type(true);
        assert_eq!(hand.hand_type, HandType::FullHouse);

        let mut hand = Hand {
            cards: String::from("A9JA9"),
            bid: 1,
            hand_type: HandType::Unknown,
        };
        hand.parse_type(true);
        assert_eq!(hand.hand_type, HandType::FullHouse);

        let mut hand = Hand {
            cards: String::from("AAAKQ"),
            bid: 1,
            hand_type: HandType::Unknown,
        };
        hand.parse_type(true);
        assert_eq!(hand.hand_type, HandType::ThreeKind);

        let mut hand = Hand {
            cards: String::from("J2QAA"),
            bid: 1,
            hand_type: HandType::Unknown,
        };
        hand.parse_type(true);
        assert_eq!(hand.hand_type, HandType::ThreeKind);
        let mut hand = Hand {
            cards: String::from("45JAJ"),
            bid: 1,
            hand_type: HandType::Unknown,
        };
        hand.parse_type(true);
        assert_eq!(hand.hand_type, HandType::ThreeKind);

        let mut hand = Hand {
            cards: String::from("AA8KK"),
            bid: 1,
            hand_type: HandType::Unknown,
        };
        hand.parse_type(true);
        assert_eq!(hand.hand_type, HandType::TwoPair);

        let mut hand = Hand {
            cards: String::from("AJ8QK"),
            bid: 1,
            hand_type: HandType::Unknown,
        };
        hand.parse_type(true);
        assert_eq!(hand.hand_type, HandType::OnePair);

        let mut hand = Hand {
            cards: String::from("JQ8AK"),
            bid: 1,
            hand_type: HandType::Unknown,
        };
        hand.parse_type(true);
        assert_eq!(hand.hand_type, HandType::OnePair);

        let mut hand = Hand {
            cards: String::from("2389Q"),
            bid: 1,
            hand_type: HandType::Unknown,
        };
        hand.parse_type(true);
        assert_eq!(hand.hand_type, HandType::HighCard);
    }
}
