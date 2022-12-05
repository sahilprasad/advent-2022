use std::collections::HashMap;

fn get_stacks() -> HashMap<i32, Vec<char>> {
    let mut map = HashMap::new();
    map.insert(1, "mjcbfrlh".chars().collect::<Vec<char>>());
    map.insert(2, "zcd".chars().collect::<Vec<char>>());
    map.insert(3, "hjfcngw".chars().collect::<Vec<char>>());
    map.insert(4, "pjdmtsb".chars().collect::<Vec<char>>());
    map.insert(5, "ncdrj".chars().collect::<Vec<char>>());
    map.insert(6, "wldqpjgz".chars().collect::<Vec<char>>());
    map.insert(7, "pztfrh".chars().collect::<Vec<char>>());
    map.insert(8, "lvmg".chars().collect::<Vec<char>>());
    map.insert(9, "cbgpfqrj".chars().collect::<Vec<char>>());
    map
}

fn make_moves(input: &str, reverse: bool) {
    let mut stacks = get_stacks();
    let split = input.split("\n\n").collect::<Vec<&str>>();
    let moves = split[1];
    moves.lines().for_each(|line| {
        let line_split = line.split_whitespace().collect::<Vec<&str>>();
        let count = line_split[1].parse::<i32>().unwrap();
        let from = line_split[3].parse::<i32>().unwrap();
        let to = line_split[5].parse::<i32>().unwrap();

        let mut from_stack = stacks.get(&from).unwrap().clone();
        let mut to_stack = stacks.get(&to).unwrap().clone();

        let mut to_add: Vec<char> = from_stack
            .drain((from_stack.len() - count as usize)..)
            .collect();
        if reverse {
            to_add.reverse();
        }
        to_stack.extend(to_add);

        stacks.insert(from, from_stack.clone());
        stacks.insert(to, to_stack.clone());
    });

    let mut res: Vec<char> = Vec::new();

    for i in 1..10 {
        let stack = stacks.get(&i).unwrap();
        res.push(*stack.last().unwrap());
    }

    println!(
        "Result: {}",
        res.iter()
            .map(|x| x.to_uppercase().collect::<String>())
            .collect::<String>()
    );
}

pub fn part_one(input: &str) -> Option<u32> {
    make_moves(input, true);
    Some(0)
}

pub fn part_two(input: &str) -> Option<u32> {
    make_moves(input, false);
    Some(0)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some(0));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), None);
    }
}
