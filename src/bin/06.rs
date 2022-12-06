use std::collections::{HashSet, VecDeque};

const START_OF_PACKET: usize = 4;
const START_OF_MESSAGE: usize = 14;

fn get_count(input: &str, max_size: usize) -> u32 {
    let mut q: VecDeque<char> = VecDeque::new();
    let mut set: HashSet<char> = HashSet::new();

    let mut count = 0;
    for c in input.chars() {
        if set.contains(&c) {
            let mut cont = true;
            while cont {
                let popped = q.pop_front().unwrap();
                set.remove(&popped);
                if popped == c {
                    cont = false;
                }
            }
        }

        q.push_back(c);
        set.insert(c);
        count += 1;

        if set.len() == max_size {
            break;
        }
        if q.len() > max_size {
            let popped = q.pop_front().unwrap();
            set.remove(&popped);
        }
    }

    count
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(get_count(input, START_OF_PACKET))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(get_count(input, START_OF_MESSAGE))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(5));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), None);
    }
}
