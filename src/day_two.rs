use crate::util;
pub fn day_two() {
    let one_data = util::read_file("two");
    let one_result = part_one(&one_data);
    let two_result = part_two(&one_data);
    println!("Input one result: {}", one_result);
    println!("Input two result: {}", two_result);
}
#[derive(Default, Debug)]
struct GameSet {
    red: usize,
    blue: usize,
    green: usize,
}

#[derive(Debug)]
struct Game {
    number: usize,
    sets: Vec<GameSet>,
}

fn part_one(data: &Vec<String>) -> usize {
    let max_blue = 14;
    let max_red = 12;
    let max_green = 13;

    let mut result = 0;

    let games = input_to_games(data);
    for game in games.iter() {
        let mut sets_valid = true;
        for set in game.sets.iter() {
            if set.red > max_red || set.blue > max_blue || set.green > max_green {
                sets_valid = false;
                break;
            }
        }
        if sets_valid {
            result += game.number
        };
    }
    result
}

fn part_two(data: &Vec<String>) -> usize {
    let mut result = 0;
    let games = input_to_games(data);
    for game in games.iter() {
        let mut blue = 0;
        let mut green = 0;
        let mut red = 0;
        for set in game.sets.iter() {
            if set.green > green {
                green = set.green;
            }
            if set.blue > blue {
                blue = set.blue;
            }
            if set.red > red {
                red = set.red;
            }
        }
        result = result + (blue * green * red);
    }
    result
}

fn input_to_games(data: &Vec<String>) -> Vec<Game> {
    data.iter()
        .map(|line| {
            let (game_number, rest) = parse_game_number(&line);
            let sets = parse_sets(rest);
            Game {
                number: game_number,
                sets,
            }
        })
        .collect()
}

fn parse_game_number(line: &String) -> (usize, &str) {
    let idx = 5;
    let end = line.find(":").unwrap();
    let game_no: usize = line[idx..end].parse().unwrap();
    let rest = line.split_at(end + 1);
    (game_no, rest.1)
}

fn parse_sets(line: &str) -> Vec<GameSet> {
    let set_str: Vec<&str> = line.trim().split(";").map(|x| x.trim()).collect();
    let mut sets: Vec<GameSet> = vec![];
    for ss in set_str {
        sets.push(parse_color_sring(ss));
    }
    sets
}

fn parse_color_sring(from: &str) -> GameSet {
    let mut blue: usize = 0;
    let mut green: usize = 0;
    let mut red: usize = 0;

    let colors: Vec<&str> = from.split(",").collect();
    for color in colors.iter() {
        if let Some((v, n)) = color.trim().split_once(" ") {
            let v = v.parse::<usize>().unwrap();

            match n {
                "blue" => blue = v,
                "green" => green = v,
                "red" => red = v,
                _ => (),
            }
        }
    }

    GameSet { red, green, blue }
}

#[cfg(test)]
mod tests {

    use super::{part_one, part_two};

    #[test]
    fn part_one_test() {
        let input: Vec<String> = vec![
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        ]
        .iter()
        .map(|x| String::from(*x))
        .collect();
        let result = part_one(&input);
        assert_eq!(result, 8);
    }
    #[test]
    fn part_two_test() {
        let input: Vec<String> = vec![
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        ]
        .iter()
        .map(|x| String::from(*x))
        .collect();
        let result = part_two(&input);
        assert_eq!(result, 2286);
    }
}
