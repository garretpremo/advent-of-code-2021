mod syntax_checker;

use std::fs;
use syntax_checker::*;

fn main() {
    let input_file_contents = fs::read_to_string("inputs/input-10.txt").unwrap();

    let lines = parse(input_file_contents);

    println!("answer 10.1: {}", calculate_illegality_score(&lines));
}

fn parse(input: String) -> Vec<String> {
    input.trim()
        .split("\r\n")
        .map(|string| String::from(string))
        .collect()
}

fn calculate_illegality_score(lines: &Vec<String>) -> u32 {
    let mut illegality_score = 0;

    for line in lines.iter() {
        match check(line) {
            SyntaxCheckerResult::Corrupt(c) => illegality_score += get_illegality_score(c),
            _ => {}
        }
    }

    illegality_score
}

#[test]
fn test_sample_input() {
    let sample_input = String::from("[({(<(())[]>[[{[]{<()<>>\r\n[(()[<>])]({[<{<<[]>>(\r\n{([(<{}[<>[]}>{[]{[(<()>\r\n(((({<>}<{<{<>}{[]{[]{}\r\n[[<[([]))<([[{}[[()]]]\r\n[{[{({}]{}}([{[{{{}}([]\r\n{<[[]]>}<{[{[{[]{()[[[]\r\n[<(<(<(<{}))><([]([]()\r\n<{([([[(<>()){}]>(<<{{\r\n<{([{{}}[<[[[<>{}]]]>[]]");
    let sample_lines = parse(sample_input);

    assert_eq!(calculate_illegality_score(&sample_lines), 26397);
}

