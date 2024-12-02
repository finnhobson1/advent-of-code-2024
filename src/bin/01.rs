use std::iter::zip;

advent_of_code::solution!(1);

fn parse_into_vectors(input: &str) -> (Vec<u32>, Vec<u32>) {
    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in input.lines() {
        let mut items = line.split_whitespace();
        left.push(items.next().unwrap().parse::<u32>().unwrap());
        right.push(items.next().unwrap().parse::<u32>().unwrap());
    }

    (left, right)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut left, mut right) = parse_into_vectors(input);

    left.sort();
    right.sort();

    let distance = zip(left, right)
        .map(|(l, r)| l.abs_diff(r))
        .sum();

    Some(distance)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (left, right) = parse_into_vectors(input);

    let mut similarity: u32 = 0;
    for l in left {
        let right_count = right.iter().filter(|&r| *r == l).count() as u32;
        similarity += l * right_count;
    }
    
    Some(similarity)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
