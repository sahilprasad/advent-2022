use std::collections::HashMap;

use advent_of_code::helpers::isnumeric;

#[derive(Debug)]
struct FileValue {
    name: String,
    bytes: u64,
}

#[derive(Debug)]
enum Value {
    Dir(String),
    File(FileValue),
}

fn build_map(input: &str) -> (HashMap<String, Vec<Value>>, HashMap<String, u64>) {
    let mut dirs: HashMap<String, Vec<Value>> = HashMap::new();
    let mut sizes: HashMap<String, u64> = HashMap::new();

    let mut dir: Vec<String> = Vec::new();
    input.lines().for_each(|line| {
        if line.starts_with("$ cd") {
            let new_dir = line.trim_start_matches("$ cd").trim();
            if !dir.is_empty() {
                if new_dir == ".." {
                    dir.pop();
                } else {
                    dir.push(new_dir.to_string());
                    sizes.entry(dir.join("/")).or_insert(0);
                }
            } else {
                sizes.insert("/".to_string(), 0);
                dir.push("/".to_string());
            }
        } else if !line.starts_with("$ ls") {
            let split = line.split_whitespace().collect::<Vec<&str>>();
            if isnumeric(split[0]) {
                let size = split[0].parse::<u64>().unwrap();
                let filename = split[1];

                let current_dir = dir.join("/");
                if !dirs.contains_key(&current_dir) {
                    dirs.insert(current_dir.clone(), vec![]);
                }

                let values = dirs.get_mut(&current_dir).unwrap();
                values.push(Value::File(FileValue {
                    name: filename.to_string(),
                    bytes: size,
                }));

                for i in 0..dir.len() + 1 {
                    let parent = dir[..i].join("/");
                    if parent.is_empty() {
                        continue;
                    }
                    let parent_size = sizes.get_mut(&parent).unwrap();
                    *parent_size += size;
                }
            } else if line.starts_with("dir") {
                let child_dirname = split[1];
                let current_dir = dir.join("/");

                if !dirs.contains_key(&current_dir) {
                    dirs.insert(current_dir.clone(), vec![]);
                }

                if !sizes.contains_key(&current_dir) {
                    sizes.insert(current_dir.clone(), 0);
                }

                let values = dirs.get_mut(&current_dir).unwrap();
                values.push(Value::Dir(child_dirname.to_string()));
            }
        }
    });

    (dirs, sizes)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (_, sizes) = build_map(input);

    let mut total = 0;
    for (_, size) in sizes {
        if size < 100000 {
            total += size;
        }
    }

    Some(total as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (_, sizes) = build_map(input);

    let used_space = sizes.get("/").unwrap();
    let unused_space = 70000000 - used_space;
    let required_space = 30000000 - unused_space;

    let mut values = sizes.values().collect::<Vec<&u64>>();
    values.retain(|&x| *x >= required_space);
    values.sort();

    Some(*values[0] as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }
}
