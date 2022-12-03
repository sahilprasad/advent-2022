pub fn part_one(input: &str) -> Option<u32> {
    let mut score = 0;
    let mine = ["x", "y", "z"];
    let theirs = ["a", "b", "c"];

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        let round = line.split(' ').collect::<Vec<&str>>();
        let my_index = mine
            .iter()
            .position(|&x| x == round[1].to_lowercase())
            .unwrap() as usize;
        let their_index = theirs
            .iter()
            .position(|&x| x == round[0].to_lowercase())
            .unwrap() as usize;
        score += my_index;
        score += 1;
        if my_index == their_index {
            score += 3;
        } else if (my_index > their_index && (my_index - their_index) == 1) || (my_index < their_index && (their_index - my_index) == 2) {
            score += 6;
        }
    }

    Some(score as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut score: u32 = 0;
    let mine = ["x", "y", "z"];
    let theirs = ["a", "b", "c"];
    for line in input.lines() {
        let round = line.split(' ').collect::<Vec<&str>>();
        let my_index = mine
            .iter()
            .position(|&x| x == round[1].to_lowercase())
            .unwrap() as u32;
        let their_index = theirs
            .iter()
            .position(|&x| x == round[0].to_lowercase())
            .unwrap() as u32;

        let move_score: u32 = match my_index {
            0 => {
                if their_index == 0 {
                    3
                } else {
                    their_index
                }
            }
            1 => their_index + 1,
            2 => ((their_index + 1) % 3) + 1,
            _ => 0,
        };

        let round_score = 3 * my_index;

        score += move_score;
        score += round_score;
    }

    Some(score as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), None);
    }
}
