use std::fs;

fn main() {
    let input_file_contents = fs::read_to_string("inputs/input-01.txt").unwrap();

    let values: Vec<i32> = input_file_contents
        .trim()
        .split("\r\n")
        .map(|value| value.parse::<i32>().unwrap())
        .collect();

    let answer_1_1 = count_measurements_larger_than_previous(&values);
    let answer_1_2 = count_measurement_windows_larger_than_previous(&values, 3);

    println!("answer 1.1: {}", answer_1_1);
    println!("answer 1.2: {}", answer_1_2);
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

fn count_measurement_windows_larger_than_previous(values: &Vec<i32>, window_size: usize) -> i32 {
    let mut measurement_windows = vec![];

    for i in 0..values.len() {
        let value = values[i];
        measurement_windows.push(value);

        for j in 1..window_size {
            if j > i { break; }

            measurement_windows[i - j] += value;
        }
    }

    count_measurements_larger_than_previous(&measurement_windows)
}
