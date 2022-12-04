pub fn part_one(input: &str) -> Option<u32> {
    let mut total = 0;
    input.lines().for_each(|line| {
        let split = line.split(',').collect::<Vec<&str>>();
        let first_range = split[0]
            .split('-')
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        let second_range = split[1]
            .split('-')
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        if (first_range[0] >= second_range[0] && first_range[1] <= second_range[1])
            || (second_range[0] >= first_range[0] && second_range[1] <= first_range[1])
        {
            total += 1;
        }
    });

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut total = 0;

    input.lines().for_each(|line| {
        let split = line.split(',').collect::<Vec<&str>>();
        let first_range = split[0]
            .split('-')
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        let second_range = split[1]
            .split('-')
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        if u32::max(first_range[0], second_range[0]) <= u32::min(first_range[1], second_range[1]) {
            total += 1;
        }
    });

    Some(total)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), None);
    }
}
