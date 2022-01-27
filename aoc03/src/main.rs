use std::fs;

#[derive(Clone)]
struct BitCount {
    one: u32,
    zero: u32
}

fn main() {
    let input_file_contents = fs::read_to_string("inputs/input-03.txt").unwrap();

    let diagnostics_report: Vec<&str> = input_file_contents
        .trim()
        .split("\r\n")
        .collect();

    println!("answer 3.1: {}", calculate_power_consumption(&diagnostics_report));
}

/// given a list of binary numbers, find the most common bits for each bit place
///
/// if 1 is the most common bit for a given place, add the bit's value to gamma_rate
/// if 0 is the most common bit for a given place, add the bit's value to epsilon_rate
///
/// returns gamma_rate * epsilon_rate
///
fn calculate_power_consumption(diagnostics_report: &Vec<&str>) -> u32 {
    let bit_length = diagnostics_report[0].len();
    let mut bit_counts = vec![BitCount { one: 0, zero: 0 }; bit_length];

    for entry in diagnostics_report {
        for (i, bit) in entry.chars().enumerate() {
            match bit {
                '1' => bit_counts[i].one += 1,
                _   => bit_counts[i].zero += 1

            }
        }
    }

    let mut gamma_rate = 0;
    let mut epsilon_rate = 0;

    for (i, bit_count) in bit_counts.iter().enumerate() {
        let bit_value = 1 << (bit_length - 1) - i;

        match bit_count {
            BitCount { one, zero } if one >= zero => gamma_rate += bit_value,
            _ => epsilon_rate += bit_value,
        }
    }

    gamma_rate * epsilon_rate
}
