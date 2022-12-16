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

fn main() {
    let answer_1 = part_1("./input.txt");
    println!("Part 1: {}", answer_1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        assert_eq!(part_1("./test_input.txt"), 15);
    }

    // fn part_2_test() {
    //     assert_eq!(part2("./test_input.txt"), 12)
    // }
}
