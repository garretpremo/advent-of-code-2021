mod decoder;

use std::fs;
use decoder::Decoder;

pub struct IO {
    input: Vec<String>,
    output: Vec<String>
}

fn main() {
    let input_file_contents = fs::read_to_string("inputs/input-08.txt").unwrap();

    let signals: Vec<IO> = parse(input_file_contents);

    println!("answer 8.1: {}", count_easy_digits(&signals));
    println!("answer 8.2: {}", calculate_output(&signals));
}

fn parse(input: String) -> Vec<IO> {
    input.trim()
        .split("\r\n")
        .map(|signal| {
            let io: Vec<&str> = signal.split(" | ").collect();
            IO {
                input: io[0].split(" ").map(|str| String::from(str)).collect(),
                output: io[1].split(" ").map(|str| String::from(str)).collect()
            }
        })
        .collect()
}

/// counts the number of digits that are either a 1, 4, 7, or 8
fn count_easy_digits(signals: &Vec<IO>) -> u32 {
    signals.iter()
        .flat_map(|io| io.output.iter())
        .fold(0, |acc, signal| {
            match signal.len() {
                2 => acc + 1, // 1
                3 => acc + 1, // 7
                4 => acc + 1, // 4
                7 => acc + 1, // 8
                _ => acc
            }
        })
}

fn calculate_output(signals: &Vec<IO>) -> u32 {
    let mut output = 0;

    for signal in signals {
        let mut decoder = Decoder::new();
        output += decoder.decode(signal);
    }

    output
}

#[test]
fn test_sample_input() {

    let sample_input = String::from("be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe\r\nedbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc\r\nfgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg\r\nfbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb\r\naecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea\r\nfgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb\r\ndbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe\r\nbdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef\r\negadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb\r\ngcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce\r\n");

    let sample_signals = parse(sample_input);

    assert_eq!(count_easy_digits(&sample_signals), 26);
    assert_eq!(calculate_output(&sample_signals), 61229);
}
