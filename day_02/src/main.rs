use std::fs;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Shape {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, Clone, Copy)]
pub enum EndState {
    Unkown,
    Loss,
    Draw,
    Win,
}

pub trait GameRound {
    fn letter_to_shape(letter: &str) -> Shape {
        match letter {
            "A" => Shape::Rock,
            "B" => Shape::Paper,
            "C" => Shape::Scissors,
            "Y" => Shape::Paper,
            "X" => Shape::Rock,
            "Z" => Shape::Scissors,
            _ => panic!("Invalid input shape!"),
        }
    }

    fn letter_to_end_state(letter: &str) -> EndState {
        match letter {
            "X" => EndState::Loss,
            "Y" => EndState::Draw,
            "Z" => EndState::Win,
            _ => panic!("Invalid input shape!"),
        }
    }

    fn end_state_to_score(&self, end_state: EndState) -> u32 {
        match end_state {
            EndState::Loss => 0,
            EndState::Draw => 3,
            EndState::Win => 6,
            _ => 0,
        }
    }

    fn shape_to_score(&self, shape: Shape) -> u32 {
        match shape {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }
}

#[derive(Clone, Copy)]
struct Part1Round {
    opponent: Shape,
    player: Shape,
    end_state: EndState,
}

impl GameRound for Part1Round {}

impl Part1Round {
    fn new(input_line: String) -> Part1Round {
        let split_line: Vec<&str> = input_line.split(" ").collect();
        let opponent_letter = split_line[0];
        let player_letter = split_line[1];
        Part1Round {
            opponent: <Part1Round as GameRound>::letter_to_shape(opponent_letter),
            player: <Part1Round as GameRound>::letter_to_shape(player_letter),
            end_state: EndState::Unkown,
        }
    }

    fn play_round(&mut self) {
        match (self.opponent, self.player) {
            (Shape::Rock, Shape::Rock) => self.end_state = EndState::Draw,
            (Shape::Paper, Shape::Paper) => self.end_state = EndState::Draw,
            (Shape::Scissors, Shape::Scissors) => self.end_state = EndState::Draw,
            (Shape::Rock, Shape::Scissors) => self.end_state = EndState::Loss,
            (Shape::Scissors, Shape::Paper) => self.end_state = EndState::Loss,
            (Shape::Paper, Shape::Rock) => self.end_state = EndState::Loss,
            (_, _) => self.end_state = EndState::Win,
        }
    }

    fn get_total_score(&mut self) -> u32 {
        self.play_round();
        self.shape_to_score(self.player) + self.end_state_to_score(self.end_state)
    }
}

// There isn't really a need for a different struct here but I am experimenting
#[derive(Clone, Copy)]
struct Part2Round {
    opponent: Shape,
    player: Option<Shape>,
    end_state: EndState,
}

impl GameRound for Part2Round {}

impl Part2Round {
    fn new(input_line: String) -> Part2Round {
        let split_line: Vec<&str> = input_line.split(" ").collect();
        let opponent_letter = split_line[0];
        let end_state_letter = split_line[1];
        Part2Round {
            opponent: <Part2Round as GameRound>::letter_to_shape(opponent_letter),
            player: None,
            end_state: <Part2Round as GameRound>::letter_to_end_state(end_state_letter),
        }
    }

    fn play_round_with_strategy(&mut self) {
        match (self.opponent, self.end_state) {
            (Shape::Rock, EndState::Win) => self.player = Some(Shape::Paper),
            (Shape::Rock, EndState::Loss) => self.player = Some(Shape::Scissors),
            (Shape::Rock, EndState::Draw) => self.player = Some(Shape::Rock),
            (Shape::Paper, EndState::Win) => self.player = Some(Shape::Scissors),
            (Shape::Paper, EndState::Loss) => self.player = Some(Shape::Rock),
            (Shape::Paper, EndState::Draw) => self.player = Some(Shape::Paper),
            (Shape::Scissors, EndState::Win) => self.player = Some(Shape::Rock),
            (Shape::Scissors, EndState::Loss) => self.player = Some(Shape::Paper),
            (Shape::Scissors, EndState::Draw) => self.player = Some(Shape::Scissors),
            (_, _) => panic!("EndState is unknown, can't determine strategy"),
        }
    }

    fn get_total_score(&mut self) -> u32 {
        self.play_round_with_strategy();
        self.shape_to_score(self.player.unwrap()) + self.end_state_to_score(self.end_state)
    }
}

fn parse_input(filepath: &str) -> Vec<String> {
    let data = fs::read_to_string(filepath).expect("Could not read file");
    let lines: Vec<String> = data.trim().split("\n").map(|s| s.to_string()).collect();
    lines
}

fn part_1(filepath: &str) -> u32 {
    let lines: Vec<String> = parse_input(filepath);
    let mut game_rounds: Vec<Part1Round> = Vec::new();
    for line in lines {
        let round = Part1Round::new(line);
        game_rounds.push(round);
    }
    game_rounds.iter_mut().map(|x| x.get_total_score()).sum()
}

fn part_2(filepath: &str) -> u32 {
    let lines: Vec<String> = parse_input(filepath);
    let mut game_rounds: Vec<Part2Round> = Vec::new();
    for line in lines {
        let round = Part2Round::new(line);
        game_rounds.push(round);
    }
    game_rounds.iter_mut().map(|x| x.get_total_score()).sum()
}

fn main() {
    let answer_1 = part_1("./input.txt");
    println!("Part 1: {}", answer_1);

    let answer_2 = part_2("./input.txt");
    println!("Part 2: {}", answer_2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        assert_eq!(part_1("./test_input.txt"), 15);
    }

    #[test]
    fn part_2_test() {
        assert_eq!(part_2("./test_input.txt"), 12)
    }
}
