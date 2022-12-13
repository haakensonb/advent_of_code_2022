use std::fs;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
enum EndState {
    Unkown,
    Loss,
    Draw,
    Win,
}

struct GameRound {
    opponent: Shape,
    player: Shape,
    end_state: EndState,
}

impl GameRound {
    fn new(opponent_letter: &str, player_letter: &str) -> GameRound {
        GameRound {
            opponent: GameRound::letter_to_shape(opponent_letter),
            player: GameRound::letter_to_shape(player_letter),
            end_state: EndState::Unkown,
        }
    }

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

    fn get_round_score(&self) -> u32 {
        match self.end_state {
            EndState::Loss => 0,
            EndState::Draw => 3,
            EndState::Win => 6,
            _ => 0,
        }
    }

    fn get_player_shape_score(&self) -> u32 {
        match self.player {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }

    fn get_total_score(&mut self) -> u32 {
        self.play_round();
        self.get_round_score() + self.get_player_shape_score()
    }
}

fn parse_input(filepath: &str) -> Vec<GameRound> {
    let data = fs::read_to_string(filepath).expect("Could not read file");
    let lines: Vec<&str> = data.trim().split("\n").collect();
    let mut game_rounds: Vec<GameRound> = Vec::new();
    for line in lines {
        let split_line: Vec<&str> = line.split(" ").collect();
        game_rounds.push(GameRound::new(split_line[0], split_line[1]));
    }
    game_rounds
}

fn part_1(filepath: &str) -> u32 {
    let mut game_rounds: Vec<GameRound> = parse_input(filepath);
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
}
