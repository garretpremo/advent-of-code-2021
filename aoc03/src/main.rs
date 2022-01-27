use std::fs;

#[derive(Clone)]
struct BitCount {
    one: u32,
    zero: u32
}

impl BitCount {
    fn new() -> BitCount {
        BitCount { one: 0, zero: 0 }
    }

    fn add_bit(&mut self, bit: char) {
        match bit {
            '1' => self.one += 1,
            _ => self.zero += 1
        }
    }
}

fn main() {
    let input_file_contents = fs::read_to_string("inputs/input-03.txt").unwrap();

    let diagnostics_report: Vec<&str> = input_file_contents
        .trim()
        .split("\r\n")
        .collect();

    println!("answer 3.1: {}", calculate_power_consumption(&diagnostics_report));
    println!("answer 3.2: {}", calculate_life_support_rating(&diagnostics_report));
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
    let mut bit_counts = vec![BitCount::new(); bit_length];

    for entry in diagnostics_report {
        for (i, bit) in entry.chars().enumerate() {
            bit_counts[i].add_bit(bit);
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

fn calculate_life_support_rating(diagnostics_report: &Vec<&str>) -> u32 {
    let oxygen_generator_rating = calculate_oxygen_generator_rating(diagnostics_report);
    let co2_scrubber_rating = calculate_co2_scrubber_rating(diagnostics_report);

    oxygen_generator_rating * co2_scrubber_rating
}

fn calculate_oxygen_generator_rating(diagnostics_report: &Vec<&str>) -> u32 {
    calculate_rating(diagnostics_report, 0, true)
}

fn calculate_co2_scrubber_rating(diagnostics_report: &Vec<&str>) -> u32 {
    calculate_rating(diagnostics_report, 0, false)
}

fn calculate_rating(diagnostics_report: &Vec<&str>, start_bit: usize, most_common: bool) -> u32 {
    if diagnostics_report.len() == 1 {
        return u32::from_str_radix(diagnostics_report[0], 2).unwrap();
    }

    let common_bit = get_common_bit(diagnostics_report, start_bit, most_common);

    let filtered_report: Vec<&str> = diagnostics_report
        .iter()
        .filter(|entry| entry.chars().nth(start_bit).unwrap() == common_bit)
        .copied()
        .collect();

    calculate_rating(&filtered_report, start_bit + 1, most_common)
}

fn get_common_bit(diagnostics_report: &Vec<&str>, start_bit: usize, most_common: bool) -> char {
    let mut bit_count = BitCount::new();

    for entry in diagnostics_report {
        bit_count.add_bit(entry.chars().nth(start_bit).unwrap());
    }

    match (most_common, bit_count) {
        (true, BitCount { one, zero }) =>   if one >= zero { '1' } else { '0' },
        (false, BitCount { one, zero }) =>  if one >= zero { '0' } else { '1' },
    }
}

/// Test values from the problem
#[test]
fn test_calculate_ratings() {
    let test_diagnostics_report = vec![
        "00100",
        "11110",
        "10110",
        "10111",
        "10101",
        "01111",
        "00111",
        "11100",
        "10000",
        "11001",
        "00010",
        "01010"
    ];

    assert_eq!(calculate_oxygen_generator_rating(&test_diagnostics_report), 23);
    assert_eq!(calculate_co2_scrubber_rating(&test_diagnostics_report), 10);
}
