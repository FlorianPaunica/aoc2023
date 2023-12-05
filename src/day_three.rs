use crate::util;

pub fn main() {
    let data = util::read_file("three");
    let one_result = part_one(&data);
    let two_result = part_two(&data);
    println!("Input one result: {}", one_result);
    println!("Input two result: {}", two_result);
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Cell {
    Number(usize),
    Symbol(bool),
    Gear,
}

struct Number {
    number: usize,
    adjecent: Vec<(usize, usize)>,
}

fn part_one(data: &Vec<String>) -> usize {
    let schematic = build_schematic(data);
    let numbers = extract_numbers(&schematic);
    let mut sum = 0;
    for n in numbers {
        sum += n.number
    }
    sum
}

fn part_two(data: &Vec<String>) -> usize {
    let mut sum = 0;
    let mut gears: Vec<(usize, usize)> = vec![];
    let schematic = build_schematic(data);
    let numbers = extract_numbers(&schematic);
    for (i, row) in schematic.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if cell == &Cell::Gear {
                gears.push((i, j));
            }
        }
    }

    gears.iter().for_each(|g| {
        let mut adj_nums = vec![];
        for num in numbers.iter() {
            if adj_nums.len() > 2 {
                break;
            }
            for adj in num.adjecent.iter() {
                if adj.0 == g.0 && adj.1 == g.1 {
                    adj_nums.push(num.number);
                }
            }
        }
        if adj_nums.len() == 2 {
            let res = adj_nums[0] * adj_nums[1];
            sum += res;
        }
    });
    sum
}

fn extract_numbers(schematic: &Vec<Vec<Cell>>) -> Vec<Number> {
    let mut numbers: Vec<Number> = vec![];
    for (i, row) in schematic.iter().enumerate() {
        let mut number = 0;
        let mut col_start = 0;
        let mut col_end = 0;
        for (j, cell) in row.iter().enumerate() {
            match cell {
                Cell::Number(n) => {
                    if number == 0 {
                        col_start = j;
                        col_end = j;
                    } else {
                        col_end = j;
                    }
                    number = number * 10 + n;
                    if j == row.len() - 1 {
                        let (valid, gears) = validate_number(&schematic, i, col_start, col_end);
                        if valid {
                            numbers.push(Number {
                                number,
                                adjecent: gears,
                            });
                        }
                    }
                }
                _ => {
                    if number != 0 {
                        let (valid, gears) = validate_number(&schematic, i, col_start, col_end);
                        if valid {
                            numbers.push(Number {
                                number,
                                adjecent: gears,
                            });
                        }
                    };
                    number = 0;
                    col_start = 0;
                    col_end = 0;
                }
            }
        }
    }
    return numbers;
}

fn validate_number(
    schematic: &Vec<Vec<Cell>>,
    row: usize,
    col_start: usize,
    col_end: usize,
) -> (bool, Vec<(usize, usize)>) {
    let max_row = schematic.len() - 1;
    let max_col = schematic[0].len() - 1;
    let mut start_col = col_start;
    let mut end_col = col_end;
    let mut rows = vec![row];
    let mut gears: Vec<(usize, usize)> = vec![];

    if row == 0 {
        rows.push(1);
    } else if row == max_row {
        rows.push(row - 1);
    } else {
        rows = [row - 1, row, row + 1].to_vec()
    }

    if col_start > 0 {
        start_col = col_start - 1;
    };

    if col_end < max_col {
        end_col = col_end + 1;
    };

    let mut cells_to_check: Vec<(usize, usize)> = vec![];
    for r in rows {
        for c in start_col..=end_col {
            match schematic[r][c] {
                Cell::Number(_) => (),
                _ => cells_to_check.push((r, c)),
            }
        }
    }
    let valid = cells_to_check
        .iter()
        .map(|c| {
            if schematic[c.0][c.1] == Cell::Gear {
                gears.push(*c);
            };
            c
        })
        .any(|c| schematic[c.0][c.1] == Cell::Symbol(true) || schematic[c.0][c.1] == Cell::Gear);
    return (valid, gears);
}
fn build_schematic(data: &Vec<String>) -> Vec<Vec<Cell>> {
    let width = data[0].len();
    let height = data.len();

    let mut schematic = vec![vec![Cell::Symbol(false); width]; height];
    for (row, line) in data.iter().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            schematic[row][col] = match ch {
                '0' => Cell::Number(0),
                '1' => Cell::Number(1),
                '2' => Cell::Number(2),
                '3' => Cell::Number(3),
                '4' => Cell::Number(4),
                '5' => Cell::Number(5),
                '6' => Cell::Number(6),
                '7' => Cell::Number(7),
                '8' => Cell::Number(8),
                '9' => Cell::Number(9),
                '.' => Cell::Symbol(false),
                '*' => Cell::Gear,
                _ => Cell::Symbol(true),
            };
        }
    }
    return schematic;
}

#[cfg(test)]
mod tests {

    use super::{part_one, part_two};

    #[test]
    fn part_one_test() {
        let input: Vec<String> = vec![
            "467..114..",
            "...*......",
            "..35..633.",
            "......#...",
            "617*......",
            ".....+.58.",
            "..592.....",
            "......755.",
            "...$.*....",
            ".664.598..",
        ]
        .iter()
        .map(|x| String::from(*x))
        .collect();
        let result = part_one(&input);
        assert_eq!(result, 4361);
    }
    #[test]
    fn part_two_test() {
        let input: Vec<String> = vec![
            "467..114..",
            "...*......",
            "..35..633.",
            "......#...",
            "617*......",
            ".....+.58.",
            "..592.....",
            "......755.",
            "...$.*....",
            ".664.598..",
        ]
        .iter()
        .map(|x| String::from(*x))
        .collect();
        let result = part_two(&input);
        assert_eq!(result, 467835);
    }
}
