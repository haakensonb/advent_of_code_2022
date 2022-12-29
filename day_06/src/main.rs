use std::{collections::HashSet, fs::read_to_string};

fn has_duplicate(input: Vec<char>) -> bool {
    let mut unique: HashSet<char> = HashSet::new();
    for c in input {
        if unique.contains(&c) {
            return true;
        } else {
            unique.insert(c);
        }
    }
    return false;
}

fn detect_marker(input: &str, group_size: usize) -> Result<usize, &str> {
    let input_chars: Vec<char> = input.chars().collect();
    let group_offset = group_size - 1;
    for i in group_offset..input_chars.len() {
        let group: Vec<char> = input[i-group_offset..=i].chars().collect();
        if !has_duplicate(group) {
            return Ok(i + 1);
        }
    }
    return Err("Could not find valid marker in input!");
}

fn part_1(input: &str) -> usize {
    let result = detect_marker(input, 4);
    return result.unwrap();
}

fn part_2(input: &str) -> usize {
    let result = detect_marker(input, 14);
    return result.unwrap();
}


fn main() {
    let data: String = read_to_string("input.txt").expect("Could not read file!");
    
    let answer_1 = part_1(&data);
    println!("Part 1: {}", answer_1);

    let answer_2 = part_2(&data);
    println!("Part 2: {}", answer_2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_duplicate_ex_1(){
        let input: Vec<char> = "mjqj".chars().collect();
        assert_eq!(has_duplicate(input), true);
    }

    #[test]
    fn test_has_duplicate_ex_2(){
        let input: Vec<char> = "jpqm".chars().collect();
        assert_eq!(has_duplicate(input), false);
    }

    #[test]
    fn test_part_1_ex_1(){
        assert_eq!(part_1("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
    }

    #[test]
    fn test_part_1_ex_2(){
        assert_eq!(part_1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
    }

    #[test]
    fn test_part_1_ex_3(){
        assert_eq!(part_1("nppdvjthqldpwncqszvftbrmjlhg"), 6);
    }

    #[test]
    fn test_part_1_ex_4(){
        assert_eq!(part_1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
    }

    #[test]
    fn test_part_1_ex_5(){
        assert_eq!(part_1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }

    #[test]
    fn test_part_2_ex_1(){
        assert_eq!(part_2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
    }
}