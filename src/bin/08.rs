use std::collections::HashSet;

// Parse the input into a 2D array of integers representing the heights of the trees
fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

// Check if a tree is visible from a given direction
fn is_visible(grid: &Vec<Vec<u32>>, row: usize, col: usize, direction: (i32, i32)) -> (bool, u32) {
    let (dr, dc) = direction;
    let (mut r, mut c) = (row as isize + dr as isize, col as isize + dc as isize);
    let height = grid[row][col];

    let mut num_trees_seen = 0;
    while r >= 0 && r < grid.len() as isize && c >= 0 && c < grid[0].len() as isize {
        num_trees_seen += 1;
        if grid[r as usize][c as usize] >= height {
            return (false, num_trees_seen);
        }
        r += dr as isize;
        c += dc as isize;
    }

    (true, num_trees_seen)
}

fn get_scenic_score(grid: &Vec<Vec<u32>>, row: usize, col: usize) -> u32 {
    let mut scores: Vec<u32> = Vec::new();

    for (dr, dc) in &[(-1, 0), (1, 0), (0, -1), (0, 1)] {
        let (_, num_trees_seen) = is_visible(grid, row, col, (*dr, *dc));
        if num_trees_seen > 0 {
            scores.push(num_trees_seen);
        }
    }

    scores.iter().product()
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse_input(input);
    let mut count = 0;
    let mut counted: HashSet<(usize, usize)> = HashSet::new();
    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            if counted.contains(&(r, c)) {
                continue;
            }

            for (dr, dc) in &[(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let (visible, _) = is_visible(&grid, r, c, (*dr, *dc));
                if visible {
                    count += 1;
                    counted.insert((r, c));
                    break;
                }
            }
        }
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = parse_input(input);
    let mut max_score = 0;
    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            let score = get_scenic_score(&grid, r, c);
            if score > max_score {
                max_score = score;
            }
        }
    }
    Some(max_score as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}
