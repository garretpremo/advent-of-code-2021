use std::{fs, usize};
use std::collections::HashMap;

fn main() {
    let input_file_contents = fs::read_to_string("inputs/input-07.txt").unwrap();

    let positions: Vec<usize> = input_file_contents
        .trim()
        .split(",")
        .map(|counter| counter.parse::<usize>().unwrap())
        .collect();

    println!("answer 7.1: {}", calculate_most_fuel_efficient_lateral_movements(&positions));
}

fn calculate_most_fuel_efficient_lateral_movements(positions: &Vec<usize>) -> usize {
    let mut max = 0;
    let mut min = usize::MAX;
    let mut sum = 0;
    let mut position_count_map: HashMap<usize, usize> = HashMap::new();

    for position in positions {
        max = max.max(*position);
        min = min.min(*position);
        sum += *position;

        if position_count_map.contains_key(position) {
            let new_count = position_count_map.get(position).unwrap() + 1;
            position_count_map.insert(*position, new_count);
        } else {
            position_count_map.insert(*position, 1);
        }
    }

    // only check a max of 50% of the positions, centered on the average
    let total_positions = positions.len();
    let average = sum / positions.len();
    let lower_bound = average.saturating_sub(total_positions / 4);
    let upper_bound = usize::min(max, average + (total_positions / 4));

    let mut fuel_consumption = vec![0usize; upper_bound - lower_bound];

    for (position, count) in position_count_map.iter() {
        for i in 0..fuel_consumption.len() {
            let diff = usize::max(i, *position) - usize::min(i, *position);
            fuel_consumption[i] += diff * count;
        }
    }

    fuel_consumption.into_iter().min().unwrap()
}

#[test]
fn test_sample_input() {
    let sample_input = vec![16,1,2,0,4,2,7,1,2,14];

    assert_eq!(calculate_most_fuel_efficient_lateral_movements(&sample_input), 37);
}
