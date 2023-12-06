use std::time::Instant;

use crate::util;

pub fn main() {
    let data = util::read_file("four");
    let start = Instant::now();
    let one_result = part_one(&data);
    let duration = start.elapsed();
    println!("input one result: {}", one_result);
    println!("execution time: {:?}", duration);
    let start = Instant::now();
    let two_result = part_two(&data);
    let duration = start.elapsed();
    println!("input two result: {}", two_result);
    println!("execution time: {:?}", duration);
}

#[derive(Debug, Default)]
struct Card {
    winners: Vec<usize>,
    have: Vec<usize>,
}

fn part_one(data: &Vec<String>) -> usize {
    let mut sum = 0;
    let cards = input_to_cards(data);
    cards.iter().for_each(|c| {
        let winners: Vec<&usize> = c.winners.iter().filter(|w| c.have.contains(*w)).collect();
        if winners.len() > 0 {
            let power: u32 = winners.len().try_into().unwrap();
            let points = 2usize.pow(power - 1);
            sum += points
        }
    });
    sum
}

fn part_two(data: &Vec<String>) -> usize {
    let mut sum = 0;
    let cards = input_to_cards(data);
    let mut card_count = vec![1; cards.len()];

    for (idx, c) in cards.iter().enumerate() {
        let winners: Vec<&usize> = c.winners.iter().filter(|w| c.have.contains(*w)).collect();
        let num_of_cards = card_count[idx];
        for i in idx + 1..=idx + winners.len() {
            card_count[i] += num_of_cards;
        }
    }
    for v in card_count {
        sum += v;
    }
    sum
}

fn input_to_cards(data: &Vec<String>) -> Vec<Card> {
    data.iter()
        .map(|line| {
            let end = line.find(":").unwrap();
            let rest = line.split_at(end + 1).1;
            let (winners, have) = rest.trim().split_once('|').unwrap();
            let winners = winners
                .trim()
                .split(" ")
                .filter_map(|n| {
                    let n = n.trim();
                    if n.len() > 0 {
                        Some(n.parse::<usize>().unwrap())
                    } else {
                        None
                    }
                })
                .collect::<Vec<usize>>();
            let have = have
                .trim()
                .split(" ")
                .filter_map(|n| {
                    let n = n.trim();
                    if n.len() > 0 {
                        Some(n.parse::<usize>().unwrap())
                    } else {
                        None
                    }
                })
                .collect::<Vec<usize>>();

            Card {
                winners,
                have,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{part_one, part_two};

    #[test]
    fn part_one_test() {
        let input: Vec<String> = vec![
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        ]
        .iter()
        .map(|x| String::from(*x))
        .collect();
        let result = part_one(&input);
        assert_eq!(result, 13);
    }
    #[test]
    fn part_two_test() {
        let input: Vec<String> = vec![
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        ]
        .iter()
        .map(|x| String::from(*x))
        .collect();
        let result = part_two(&input);
        assert_eq!(result, 30);
    }
}
