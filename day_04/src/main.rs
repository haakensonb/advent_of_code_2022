use std::fs;

struct Pair {
    s1_start: u32,
    s1_end: u32,
    s2_start: u32,
    s2_end: u32
}

impl TryFrom<&str> for Pair {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let values: Result<Vec<u32>, _> = value.split(['-', ',']).map(|v| v.parse()).collect();
        match values {
            Ok(values) => {
                // Could panic here so kind of defeats the purpose of trying to handle errors with a Result
                // but I'm not sure of a better way to do this right now.
                let (s1_start, s1_end, s2_start, s2_end) = (values[0], values[1], values[2], values[3]);
                Ok(Pair {s1_start, s1_end, s2_start, s2_end})
            },
            Err(_) => Err("Could not parse input")
        }
    }
}

impl Pair {
    fn has_overlap(&self) -> bool {
        ((self.s1_start <= self.s2_start) && (self.s1_end >= self.s2_end))
            || ((self.s2_start <= self.s1_start) && (self.s2_end >= self.s1_end))
    }
}

fn part_1(data: &str) -> usize{
    let pairs: Vec<Pair> = data.lines().filter_map(|line| Pair::try_from(line).ok()).collect();
    pairs
        .iter()
        .filter(|p| p.has_overlap())
        .count()
}

fn main() {
    let data = fs::read_to_string("input.txt").expect("Input file can't be read");
    let answer_1 = part_1(&data);
    println!("Part 1: {}", answer_1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let data = fs::read_to_string("test_input.txt").expect("Input file can't be read");
        assert_eq!(part_1(&data), 2);
    }
}