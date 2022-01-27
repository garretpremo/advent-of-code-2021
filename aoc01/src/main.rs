use std::fs;

fn main() {
    let input_file_contents = fs::read_to_string("inputs/input-01.txt").unwrap();

    let values: Vec<i32> = input_file_contents
        .trim()
        .split("\r\n")
        .map(|value| value.parse::<i32>().unwrap())
        .collect();

    let answer = count_measurements_larger_than_previous(&values);

    println!("answer: {}", answer);
}

fn count_measurements_larger_than_previous(values: &Vec<i32>) -> i32 {
    let mut previous = i32::MAX;
    let mut measurements_larger_than_previous = 0;

    for value in values {
        if *value > previous {
            measurements_larger_than_previous += 1;
        }
        previous = *value;
    }

    measurements_larger_than_previous
}
