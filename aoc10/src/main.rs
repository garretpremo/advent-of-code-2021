mod syntax_checker;

use std::fs;
use syntax_checker::*;

fn main() {
    let input_file_contents = fs::read_to_string("inputs/input-10.txt").unwrap();

    let lines = parse(input_file_contents);

    println!("answer 10.1: {}", calculate_illegality_score(&lines));
    println!("answer 10.2: {}", calculate_middle_completion_score(&lines));
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

fn calculate_middle_completion_score(lines: &Vec<String>) -> u64 {
    let mut completion_scores: Vec<u64> = vec![];

    for line in lines.iter() {
        match check(line) {
            SyntaxCheckerResult::Incomplete(completion) => {
                let mut score = 0u64;

                for c in completion.chars() {
                    score = score * 5;
                    score += get_completion_score(c);
                }
                completion_scores.push(score);
            },
            _ => {}
        }
    }

    completion_scores.sort();
    completion_scores[completion_scores.len() / 2]
}

#[test]
fn test_sample_input() {
    let sample_input = String::from("[({(<(())[]>[[{[]{<()<>>\r\n[(()[<>])]({[<{<<[]>>(\r\n{([(<{}[<>[]}>{[]{[(<()>\r\n(((({<>}<{<{<>}{[]{[]{}\r\n[[<[([]))<([[{}[[()]]]\r\n[{[{({}]{}}([{[{{{}}([]\r\n{<[[]]>}<{[{[{[]{()[[[]\r\n[<(<(<(<{}))><([]([]()\r\n<{([([[(<>()){}]>(<<{{\r\n<{([{{}}[<[[[<>{}]]]>[]]");
    let sample_lines = parse(sample_input);

    assert_eq!(calculate_illegality_score(&sample_lines), 26397);
    assert_eq!(calculate_middle_completion_score(&sample_lines), 288957);
}

