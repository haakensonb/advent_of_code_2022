use std::fs;

fn parse_and_create_stacks(stack_input: &str) -> Vec<Vec<char>> {
    let rev_lines: Vec<&str> = stack_input.lines().rev().collect();
    let stack_indexes: Vec<usize> = rev_lines[0].chars().enumerate().filter_map( |(i, c)| {
        if c != ' ' { Some(i) }
        else { None }
    }).collect();

    let mut stacks: Vec<Vec<char>> = Vec::new();
    for _ in 0..stack_indexes.len() {
        stacks.push(Vec::new());
    }

    for line in &rev_lines[1..] {
        for (i, c) in line.chars().enumerate() {
            if c.is_alphanumeric() {
                let index = stack_indexes.iter().position(|&x| x == i).unwrap();
                stacks[index].push(c);
            }
        }
    }

    stacks
}

#[derive(Debug)]
struct Move {
    num_times: u32,
    from: usize,
    to: usize,
}

fn parse_moves(moves_input: &str) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::new();
    for line in moves_input.lines() {
        let split_line: Vec<&str> = line.split(" ").collect();
        let num_times: u32 = split_line[1].parse().unwrap();
        let from: usize = split_line[3].parse().unwrap();
        let to: usize = split_line[5].parse().unwrap();
        moves.push(Move {num_times, from: from - 1, to: to - 1});
    }
    moves
}

fn execute_move(stacks: &mut Vec<Vec<char>>, m: &Move) {
    for _ in 0 .. m.num_times {
        let popped = stacks[m.from].pop().unwrap();
        stacks[m.to].push(popped);
    }
}

fn execute_moves(stacks: &mut Vec<Vec<char>>, moves: &Vec<Move>) {
    for m in moves {
        execute_move(stacks, m)
    }
}

fn part_1(stacks: &mut Vec<Vec<char>>, moves: &Vec<Move>) -> String {
    execute_moves(stacks, moves);
    let tops: Vec<String> = stacks.iter().map(|s| s[s.len()-1].to_string()).collect();
    tops.join("")
}

fn main() {
    let data = fs::read_to_string("input.txt").expect("Could not read file");
    let mut split_data = data.split("\n\n");
    let stack_input = split_data.next().unwrap();
    let moves_input = split_data.next().unwrap();

    let mut stacks: Vec<Vec<char>> = parse_and_create_stacks(stack_input);
    // println!("stacks: {:?}", stacks);

    let moves: Vec<Move> = parse_moves(moves_input);
    // println!("moves: {:?}", moves);

    let answer_1 = part_1(&mut stacks, &moves);
    println!("Part 1: {}", answer_1);
}
