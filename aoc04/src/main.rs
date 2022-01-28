mod bingo;

use bingo::Bingo;
use std::fs;

fn main() {
    let input_file_contents = fs::read_to_string("inputs/input-04.txt").unwrap();

    let input: Vec<&str> = input_file_contents
        .trim()
        .split("\r\n")
        .collect();

    let number_draws = parse_number_draws(input[0]);
    let bingo_boards = parse_boards(&input);

    println!("answer 4.1: {}", calculate_winning_board_and_score(number_draws, bingo_boards));
}

fn calculate_winning_board_and_score(number_draws: Vec<u32>, mut bingo_boards: Vec<Bingo>) -> u32 {

    let mut winning_board_index = None;
    let mut winning_draw = None;
    let board_count = bingo_boards.len();

    'outer: for draw in number_draws.into_iter() {

        for board_index in 0..board_count {
            let board = &mut bingo_boards[board_index];

            board.mark_number(draw);

            if board.has_won() {
                winning_board_index = Some(board_index);
                winning_draw = Some(draw);
                break 'outer;
            }
        }
    }

    bingo_boards[winning_board_index.unwrap()].sum_of_unmarked_numbers() * winning_draw.unwrap()
}

fn parse_number_draws(number_draws_csv: &str) -> Vec<u32> {
    number_draws_csv.split(",")
        .map(|value| value.parse::<u32>().unwrap())
        .collect()
}

fn parse_boards(input: &Vec<&str>) -> Vec<Bingo> {
    let mut board_data: Vec<Vec<u32>> = vec![];
    let mut boards: Vec<Bingo> = vec![];

    for i in 1..input.len() {
        let line = input[i];

        if line == "" {
            if board_data.len() == 5 {
                boards.push(Bingo::new(board_data));
                board_data = vec![];
            }
            continue;
        }

        let board_row = line.split_whitespace()
            .map(|value| value.parse::<u32>().unwrap())
            .collect();

        board_data.push(board_row);
    }

    boards
}
