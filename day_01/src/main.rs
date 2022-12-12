use std::fs;

fn parse_input(data: String) -> Vec<u32> {
    let split_double_newline: Vec<&str> = data.split("\n\n").collect();
    let mut result: Vec<u32> = Vec::new();
    for s in split_double_newline {
        let each: Vec<u32> = s.split("\n").map(|s| s.parse().unwrap()).collect();
        result.push(each.iter().sum());
    }
    return result;
}

fn read_and_parse(filepath: &str) -> Vec<u32> {
    let data = fs::read_to_string(filepath).expect("Could not open file!");
    parse_input(data)
}

fn top_n_sum(nums: Vec<u32>, n: usize) -> u32 {
    let mut new_nums = nums.clone();
    new_nums.sort();
    new_nums.iter().rev().take(n).sum()
}

fn part_1(filepath: &str) -> u32 {
    let nums = read_and_parse(filepath);
    top_n_sum(nums, 1)
}

fn part_2(filepath: &str) -> u32 {
    let nums = read_and_parse(filepath);
    top_n_sum(nums, 3)
}

fn main() {
    println!("Part 1: {}", part_1("./input.txt"));
    println!("Part 2: {}", part_2("./input.txt"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_ex() {
        assert_eq!(part_1("./test_input.txt"), 24000);
    }

    #[test]
    fn part_2_ex() {
        assert_eq!(part_2("./test_input.txt"), 45000);
    }
}
