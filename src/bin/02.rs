advent_of_code::solution!(2);

fn is_safe(levels: &Vec<u32>) -> bool {
    levels.is_sorted_by(|a, b| a < b && (b - a) <= 3) ||
    levels.is_sorted_by(|a, b| a > b && (a - b) <= 3)
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut safe_count: u32 = 0;
    for line in input.lines() {
        let levels: Vec<u32> = line.split_whitespace().map(|s| s.parse::<u32>().unwrap()).collect();
        if is_safe(&levels) { safe_count += 1; }
    }
    Some(safe_count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut safe_count: u32 = 0;
    for line in input.lines() {
        let levels: Vec<u32> = line.split_whitespace().map(|s| s.parse::<u32>().unwrap()).collect();
        if is_safe(&levels) {
            safe_count += 1;
        } else {
            for i in 0..levels.len() {
                let mut new_levels = levels.clone();
                new_levels.remove(i);
                if is_safe(&new_levels) {
                    safe_count += 1;
                    break;
                }
            }
        }
    }
    Some(safe_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
