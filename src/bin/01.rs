fn get_list(input: &str) -> Option<Vec<u32>> {
    let mut totals: Vec<u32> = Vec::new();

    let mut current = 0;

    for line in input.lines() {
        if line.is_empty() {
            totals.push(current);
            current = 0;
            continue;
        }

        let num = line.parse::<u32>().ok()?;
        current += num;
    }

    Some(totals)
}

pub fn part_one(input: &str) -> Option<u32> {
    let totals = get_list(input).unwrap();
    let max_value = totals.iter().max().unwrap();
    Some(*max_value)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut totals = get_list(input).unwrap();
    totals.sort_by(|a, b| b.cmp(a));
    let max_values = totals[..3].iter().sum();
    Some(max_values)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), None);
    }
}
