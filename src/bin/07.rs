advent_of_code::solution!(7);
use std::iter::repeat;

use iter_tools::Itertools;

struct Equation {
    test_value: u64,
    numbers: Vec<u64>
}

const OPERATORS: [char; 2] = ['*', '+'];

fn parse_into_equations(input: &str) -> Vec<Equation> {
    input.lines().map(|line| {
        let (test_value, numbers) = line.split_once(": ").unwrap();
        Equation {
            test_value: test_value.parse().unwrap(),
            numbers: numbers.split_whitespace().map(|num| num.parse().unwrap()).collect(),
        }
    }).collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let equations: Vec<Equation> = parse_into_equations(input);
    let mut total: u64 = 0;

    for equation in equations {
        let operator_combinations: Vec<Vec<&char>> = repeat(OPERATORS.iter()).take(equation.numbers.len() - 1).multi_cartesian_product().collect();
        for operators in operator_combinations {
            let mut value = equation.numbers[0];
            for i in 0..operators.len() {
                match *operators[i] {
                    '*' => value *= equation.numbers[i+1],
                    '+' => value += equation.numbers[i+1],
                    _ => println!("Invalid operator!")
                }
            }
            if value == equation.test_value {
                total += equation.test_value;
                break;
            }
        }
    }
    Some(total)
}

const NEW_OPERATORS: [char; 3] = ['*', '+', '|'];

pub fn part_two(input: &str) -> Option<u64> {
    let equations: Vec<Equation> = parse_into_equations(input);
    let mut total: u64 = 0;

    for equation in equations {
        let operator_combinations: Vec<Vec<&char>> = repeat(NEW_OPERATORS.iter()).take(equation.numbers.len() - 1).multi_cartesian_product().collect();
        for operators in operator_combinations {
            let mut value = equation.numbers[0];
            for i in 0..operators.len() {
                match *operators[i] {
                    '*' => value *= equation.numbers[i+1],
                    '+' => value += equation.numbers[i+1],
                    '|' => value = format!("{}{}", value, equation.numbers[i+1]).parse().unwrap(),
                    _ => println!("Invalid operator!")
                }
            }
            if value == equation.test_value {
                total += equation.test_value;
                break;
            }
        }
    }
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
