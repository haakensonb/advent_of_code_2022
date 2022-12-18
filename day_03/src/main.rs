/* Thoughts:
- Each line of letters should be split in half into two compartments
- Want to identify letters that appear in both compartments for a line
    - This can be found easily using intersection, if both compartments are sets
- Should have a function to convert a letter to integer priority
- Want the sum of priorities of all these intersection letters
*/
use std::collections::{HashMap, HashSet};
use std::fs;

fn file_to_rucksacks(filepath: &str) -> Vec<Vec<char>> {
    let file_contents = fs::read_to_string(filepath).expect("Could not read file");
    let lines: Vec<&str> = file_contents.trim().split("\n").collect();
    let mut rucksacks: Vec<Vec<char>> = Vec::new();
    for line in lines.iter() {
        let chars: Vec<char> = line.chars().collect();
        rucksacks.push(chars);
    }
    rucksacks
}

fn char_to_priority(c: char) -> u32 {
    let lowercase: [char; 26] = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ];
    let uppercase: [char; 26] = lowercase.map(|c| c.to_ascii_uppercase());
    let chars = [lowercase, uppercase].concat();
    let mut priorities: HashMap<char, u32> = HashMap::new();
    let mut priority: u32 = 1;
    for c in chars.iter() {
        priorities.insert(*c, priority);
        priority += 1;
    }
    *priorities.get(&c).unwrap()
}

fn rucksack_to_compartments(rucksack: Vec<char>) -> (Vec<char>, Vec<char>) {
    let compartments = rucksack.split_at(rucksack.len() / 2);
    (compartments.0.to_vec(), compartments.1.to_vec())
}

fn get_compartments_intersection(compartments: (Vec<char>, Vec<char>)) -> Vec<char> {
    let first_compartment: HashSet<_> = HashSet::from_iter(compartments.0);
    let second_compartment: HashSet<_> = HashSet::from_iter(compartments.1);
    let intersection: HashSet<_> = first_compartment
        .intersection(&second_compartment)
        .collect();
    let intersecting_chars: Vec<char> = intersection.into_iter().cloned().collect();
    intersecting_chars
}

fn rucksacks_to_priorities(rucksacks: Vec<Vec<char>>) -> Vec<u32> {
    let mut priorities: Vec<u32> = Vec::new();
    for rucksack in rucksacks {
        let compartments = rucksack_to_compartments(rucksack);
        let intersecting_chars = get_compartments_intersection(compartments);
        let priority: u32 = intersecting_chars
            .iter()
            .map(|c| char_to_priority(*c))
            .sum();
        priorities.push(priority);
    }
    priorities
}

fn part_1(filepath: &str) -> u32 {
    let rucksacks = file_to_rucksacks(filepath);
    let priorities: Vec<u32> = rucksacks_to_priorities(rucksacks);
    priorities.iter().sum()
}

fn main() {
    let answer_1 = part_1("input.txt");
    println!("Part 1: {}", answer_1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1("test_input.txt"), 157);
    }

    #[test]
    fn test_char_to_priority() {
        assert_eq!(char_to_priority('a'), 1);
        assert_eq!(char_to_priority('z'), 26);
        assert_eq!(char_to_priority('A'), 27);
        assert_eq!(char_to_priority('Z'), 52);
    }

    #[test]
    fn test_rucksack_to_compartments() {
        let rucksack: Vec<char> = "vJrwpWtwJgWrhcsFMMfFFhFp".chars().collect();
        let expected_first_compartment: Vec<char> = "vJrwpWtwJgWr".chars().collect();
        let expected_second_compartment: Vec<char> = "hcsFMMfFFhFp".chars().collect();
        let compartments = rucksack_to_compartments(rucksack);
        assert_eq!(expected_first_compartment, compartments.0);
        assert_eq!(expected_second_compartment, compartments.1);
    }

    #[test]
    fn test_get_compartments_intersection() {
        let rucksack: Vec<char> = "vJrwpWtwJgWrhcsFMMfFFhFp".chars().collect();
        let compartments = rucksack_to_compartments(rucksack);
        let intersecting_chars = get_compartments_intersection(compartments);
        let c = intersecting_chars.first().unwrap();
        assert_eq!(*c, 'p');
    }
}
