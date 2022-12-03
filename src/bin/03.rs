use std::collections::HashSet;

fn get_chars_vec() -> Vec<char> {
    ('a'..='z')
        .into_iter()
        .chain(('A'..='Z').into_iter())
        .collect::<Vec<char>>()
}

fn index_of(c: char) -> usize {
    let chars = get_chars_vec();
    let index = chars.iter().position(|&x| x == c).unwrap() as usize;
    index
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum: u32 = 0;
    input.lines().for_each(|line| {
        let left = line.chars().take(line.len() / 2).collect::<String>();
        let left_set: HashSet<char> = HashSet::from_iter(left.chars());
        let right = line.chars().skip(line.len() / 2).collect::<String>();
        let right_set = HashSet::from_iter(right.chars());

        for c in left_set.intersection(&right_set) {
            let index = index_of(*c) + 1;
            sum += index as u32;
        }
    });

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sum: u32 = 0;
    let mut chunk: Vec<&str> = Vec::new();

    for line in input.lines() {
        chunk.push(line);
        if chunk.len() == 3 {
            let a: HashSet<char> = HashSet::from_iter(chunk[0].chars());
            let b: HashSet<char> = HashSet::from_iter(chunk[1].chars());
            let c: HashSet<char> = HashSet::from_iter(chunk[2].chars());

            let ab = a
                .intersection(&b)
                .collect::<Vec<&char>>()
                .iter()
                .map(|&x| *x)
                .collect::<HashSet<char>>();
            for c in ab.intersection(&c) {
                let index = index_of(*c) + 1;
                sum += index as u32;
            }

            chunk.clear();
        }
    }

    Some(sum)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), None);
    }
}
