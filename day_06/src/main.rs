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

fn part_1(input: &str) -> Result<usize, &str> {
    let input_chars: Vec<char> = input.chars().collect();
    for i in 3..input_chars.len() {
        let last_4: Vec<char> = input[i-3..=i].chars().collect();
        if !has_duplicate(last_4) {
            return Ok(i + 1);
        }
    }
    return Err("Could not find valid marker in input!");
}

fn main() {
    let data: String = read_to_string("input.txt").expect("Could not read file!");
    
    let answer_1 = part_1(&data);
    println!("Part 1: {}", answer_1.unwrap());
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
        assert_eq!(part_1("mjqjpqmgbljsphdztnvjfqwrcgsmlb").unwrap(), 7);
    }

    #[test]
    fn test_part_1_ex_2(){
        assert_eq!(part_1("bvwbjplbgvbhsrlpgdmjqwftvncz").unwrap(), 5);
    }

    #[test]
    fn test_part_1_ex_3(){
        assert_eq!(part_1("nppdvjthqldpwncqszvftbrmjlhg").unwrap(), 6);
    }

    #[test]
    fn test_part_1_ex_4(){
        assert_eq!(part_1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg").unwrap(), 10);
    }

    #[test]
    fn test_part_1_ex_5(){
        assert_eq!(part_1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw").unwrap(), 11);
    }
}