advent_of_code::solution!(4);

const XMAS: [char; 4] = ['X', 'M', 'A', 'S'];

fn is_xmas_in_direction(grid: &Vec<Vec<char>>, i: i32, j: i32, direction: &(i32, i32)) -> bool {
    let height = grid.len() as i32;
    let width = grid[0].len() as i32;
    let i_dir = direction.0;
    let j_dir = direction.1;

    if (i + j_dir * 3 < 0) || (i + j_dir * 3 > height - 1) || (j + i_dir * 3 < 0) || (j + i_dir * 3 > width - 1) {
        return false;
    }

    for n in 1..=3 {
        let next_i = (i + j_dir * n) as usize;
        let next_j = (j + i_dir * n) as usize;
        let next_char = grid[next_i][next_j];
        if next_char != XMAS[n as usize] {
            return false;
        }
    }
    true
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let directions= vec![(1,0), (1,1), (0,1), (-1,1), (-1,0), (-1,-1), (0,-1), (1,-1)];
    let mut xmas_count: u32 = 0;

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == XMAS[0] {
                for direction in &directions {
                    if is_xmas_in_direction(&grid, i as i32, j as i32, direction) {
                        xmas_count += 1;
                    }
                }
            }
        }
    }
    Some(xmas_count)
}

fn is_cross_mas(grid: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    let height = grid.len();
    let width = grid[0].len();

    if (i < 1) || (i > height - 2) || (j < 1) || (j > width - 2) {
        return false;
    }

    let top_left = grid[i-1][j-1];
    let top_right = grid[i-1][j+1];
    let bottom_left = grid[i+1][j-1];
    let bottom_right = grid[i+1][j+1];

    let string: String = [top_left, top_right, bottom_right, bottom_left, top_left, top_right, bottom_right].iter().collect();
    string.contains("MMSS")
}


pub fn part_two(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut count: u32 = 0;

    for i in 1..grid.len()-1 {
        for j in 1..grid[0].len()-1 {
            if grid[i][j] == 'A' {
                if is_cross_mas(&grid, i, j) {
                    count += 1;
                }
            }
        }
    }
    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
