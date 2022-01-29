use std::fs;

fn main() {
    let input_file_contents = fs::read_to_string("inputs/input-06.txt").unwrap();

    let timers: Vec<usize> = input_file_contents
        .trim()
        .split(",")
        .map(|counter| counter.parse::<usize>().unwrap())
        .collect();

    println!("answer 6.1: {}", calculate_reproduction(&timers, 80));
}

fn calculate_reproduction(initial_timers: &Vec<usize>, days: u32) -> usize {
    let mut timers = vec![0; 9];
    let mut fish_total = initial_timers.len();

    for timer in initial_timers {
        timers[*timer] += 1;
    }

    for _ in 0..days {
        let new_fish = timers[0];

        for i in 0..8 {
            timers[i] = timers[i + 1];
        }

        fish_total += new_fish;
        timers[6] += new_fish;
        timers[8] = new_fish;
    }

    fish_total
}

#[test]
fn test_sample_input() {
    let sample_input = vec![3,4,3,1,2];

    assert_eq!(calculate_reproduction(&sample_input, 18), 26);
    assert_eq!(calculate_reproduction(&sample_input, 80), 5934);
}
