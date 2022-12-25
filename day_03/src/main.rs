use std::collections::HashSet;
use std::fs;

fn char_to_priority(c: char) -> u32 {
    let alphabet: Vec<char> = ('a'..='z').chain('A'..='Z').collect();
    let priorities: Vec<u32> = (1..=52).collect();
    let index = alphabet.iter().position(|&x| x == c).unwrap();
    *priorities.get(index).unwrap()
}

fn get_intersecting_char(sets: Vec<HashSet<char>>) -> char {
    let intersecting_char_set = sets.into_iter().reduce(|a, b| a.intersection(&b).cloned().collect()).unwrap();
    *intersecting_char_set.iter().next().unwrap()
}

fn part_1(data: String) -> u32 {
    let lines: Vec<Vec<char>> = data.lines().map(|line| line.chars().collect()).collect();
    let mut priorities = 0u32;
    for line in &lines {
        let split = line.split_at(line.len() / 2);
        let compartments: Vec<HashSet<char>> = vec![split.0.iter().cloned().collect(), split.1.iter().cloned().collect()];
        let intersecting_char = get_intersecting_char(compartments);
        let priority = char_to_priority(intersecting_char);
        priorities += priority;
    }
    return priorities;
}

fn part_2(data: String) -> u32 {
    let lines: Vec<HashSet<char>> = data
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    lines
        .chunks(3)
        .map(|x| -> u32 {
            let sets: Vec<HashSet<char>> = x.to_vec();
            let intersecting_char = get_intersecting_char(sets);
            char_to_priority(intersecting_char)
        })
        .sum()
}

fn main() {
    let data_1 = fs::read_to_string("input.txt").expect("Could not read file");
    let answer_1 = part_1(data_1);
    println!("Part 1: {}", answer_1);


    let data_2 = fs::read_to_string("input.txt").expect("Could not read file");
    let answer_2 = part_2(data_2);
    println!("Part 2: {}", answer_2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let data = fs::read_to_string("test_input.txt").expect("Could not read file");
        assert_eq!(part_1(data), 157);
    }

    #[test]
    fn test_part_2() {
        let data = fs::read_to_string("test_input.txt").expect("Could not read file");
        assert_eq!(part_2(data), 70);
    }

    #[test]
    fn test_char_to_priority() {
        assert_eq!(char_to_priority('a'), 1);
        assert_eq!(char_to_priority('z'), 26);
        assert_eq!(char_to_priority('A'), 27);
        assert_eq!(char_to_priority('Z'), 52);
    }

    #[test]
    fn test_get_intersecting_char() {
        let set_a = HashSet::from(['a', 'b', 'c']);
        let set_b = HashSet::from(['b', 'd', 'e']);
        let set_c = HashSet::from(['f', 'g', 'b']);
        let sets: Vec<HashSet<_>> = vec![set_a, set_b, set_c];
        let intersecting_char = get_intersecting_char(sets);
        assert_eq!(intersecting_char, 'b');
    }
}
