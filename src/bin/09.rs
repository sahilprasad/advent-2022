use std::collections::HashSet;

// Struct to represent a coordinate in 2D space
#[derive(Debug, Copy, Clone)]
struct Coordinate {
    x: i32,
    y: i32,
}

fn get_unique_tail_coords(input: &str, num_tails: i32) -> Option<u32> {
    let mut rope: Vec<Coordinate> = vec![Coordinate { x: 0, y: 0 }];
    for _ in 0..num_tails {
        rope.push(Coordinate { x: 0, y: 0 });
    }

    let mut tail_coords: HashSet<(i32, i32)> = HashSet::new();
    tail_coords.insert((0, 0));

    input.lines().for_each(|line| {
        let parts = line.split_whitespace().collect::<Vec<&str>>();
        let direction = parts[0];
        let steps: i32 = parts[1].parse().unwrap();

        for _ in 0..steps {
            let head = rope.first_mut().unwrap();
            match direction {
                "U" => {
                    head.y += 1;
                }
                "D" => {
                    head.y -= 1;
                }
                "L" => {
                    head.x -= 1;
                }
                "R" => {
                    head.x += 1;
                }
                _ => {}
            }

            for i in 1..rope.len() {
                let dx = (rope[i - 1].x - rope[i].x).abs();
                let dy = (rope[i - 1].y - rope[i].y).abs();

                if (dx > 1 || dy > 1) || (dx > 0 && dy > 0 && !(dx == 1 && dy == 1)) {
                    rope[i].x += (rope[i - 1].x - rope[i].x).signum();
                    rope[i].y += (rope[i - 1].y - rope[i].y).signum();
                }
            }

            let last = rope.last().unwrap();
            tail_coords.insert((last.x, last.y));

            // println!("Rope: {:?}", rope);
        }
    });

    // println!("Tail coordinates: {:?}", tail_coords);

    Some(tail_coords.len() as u32)
}

pub fn part_one(input: &str) -> Option<u32> {
    get_unique_tail_coords(input, 1)
}

pub fn part_two(input: &str) -> Option<u32> {
    get_unique_tail_coords(input, 9)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), Some(36));
    }
}
