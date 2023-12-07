use std::time::Instant;

use crate::util;

pub fn main() {
    let data = util::read_file("six");
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

#[derive(Debug, Copy, Clone)]
struct Race {
    time: usize,
    distance: usize,
    speed: usize
}

impl Race {
    fn hold_button(&mut self) {
        self.speed += 1;
        self.time -= 1;
    }

    fn estimate_distance(&self) -> usize {
        self.time * self.speed
    }
}

fn part_one(data: &Vec<String>) -> usize {
    let mut sum = 0;
    let races = parse_input(data);
    for mut race in races {
        let mut winners: Vec<usize> = vec![];
        while race.time > 0 {
            let distance = race.estimate_distance(); 
            if distance > race.distance {
                winners.push(race.speed);
            }
            race.hold_button();
        }
        if sum == 0 {
            sum = winners.len();
        } else {
            sum = sum * winners.len();
        }
    }
    sum
}

fn part_two(data: &Vec<String>) -> usize {
    let mut sum = 0;
    let mut found_winners = false;
    let mut loosers = 0;
    let mut race = parse_input_two(data);
        while race.time > 0 {
            if loosers > 1 {
                break;
            }
            let distance = race.estimate_distance(); 
            if distance > race.distance {
                found_winners = true;
                sum+=1;
            } else if found_winners {
                loosers += 1;
            }
            race.hold_button();
        }
    sum
}

fn parse_input(data: &Vec<String>) -> Vec<Race> {
    let times: Vec<usize> = data.first().unwrap().split_once(":").map(|(_, rest)| {
        rest.trim().split(" ").filter_map(|i| i.parse().ok())
    }).unwrap().collect();
    let distances: Vec<usize> = data.last().unwrap().split_once(":").map(|(_, rest)| {
        rest.trim().split(" ").filter_map(|i| i.parse().ok())
    }).unwrap().collect();
    distances.iter().enumerate().map(|(i, d)| Race {distance: *d, time: times[i], speed: 0}).collect()
}

fn parse_input_two(data: &Vec<String>) -> Race {
    let time: usize = data.first().unwrap().split_once(":").map(|(_, rest)| {
        rest.trim().replace(" ", "").parse::<usize>().unwrap()
    }).unwrap();
    let distance: usize = data.last().unwrap().split_once(":").map(|(_, rest)| {
        rest.trim().replace(" ", "").parse::<usize>().unwrap()
    }).unwrap();
    Race {
        time,
        distance,
        speed: 0
    }
}

#[cfg(test)]
mod tests {
    use super::{part_one, part_two};

    #[test]
    fn part_one_test() {
        let input: Vec<String> = vec!["Time:      7  15   30", "Distance:  9  40  200"]
            .iter()
            .map(|x| String::from(*x))
            .collect();
        let result = part_one(&input);
        assert_eq!(result, 288);
    }

        #[test]
        fn part_two_test() {
        let input: Vec<String> = vec!["Time:      7  15   30", "Distance:  9  40  200"]
            .iter()
            .map(|x| String::from(*x))
            .collect();
            let result = part_two(&input);
            assert_eq!(result, 71503);
        }
}
