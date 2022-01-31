mod paper;
mod fold;

use std::fs;
use paper::Point;
use paper::Paper;
use fold::Fold;

fn main() {
    let input_file_contents = fs::read_to_string("inputs/input-13.txt").unwrap();

    let (points, instructions) = parse(input_file_contents);

    println!("answer 13.1: {}", count_dots_after_folding(&points, &instructions, 1, false));
    println!("answer 13.2: {}", count_dots_after_folding(&points, &instructions, instructions.len(), true));
}

fn parse(input: String) -> (Vec<Point>, Vec<Fold>) {
    let mut parsed_all_points = false;
    let mut points = vec![];
    let mut instructions = vec![];

    input.trim()
        .split("\r\n")
        .for_each(|string| {
            if string == "" {
                parsed_all_points = true;
                return;
            }

            match parsed_all_points {
                false => points.push(Point::from_str(string)),
                true => instructions.push(Fold::from_str(string))
            }
        });

    (points, instructions)
}

fn count_dots_after_folding(dots: &Vec<Point>, instructions: &Vec<Fold>, folds: usize, print: bool) -> usize {
    use Fold::{X, Y};

    let mut paper = Paper::new();
    paper.add_dots(dots);

    for i in 0..folds {
        match instructions[i] {
            X(x) => paper.fold_left(x),
            Y(y) => paper.fold_up(y)
        }
    }

    if print {
        paper.print_dots();
    }

    paper.count_dots()
}

#[test]
fn test_sample_input() {
    let sample_input = String::from("6,10\r\n0,14\r\n9,10\r\n0,3\r\n10,4\r\n4,11\r\n6,0\r\n6,12\r\n4,1\r\n0,13\r\n10,12\r\n3,4\r\n3,0\r\n8,4\r\n1,10\r\n2,14\r\n8,10\r\n9,0\r\n\r\nfold along y=7\r\nfold along x=5");
    let (points, instructions) = parse(sample_input);

    assert_eq!(count_dots_after_folding(&points, &instructions, instructions.len(), true), 16);
}

