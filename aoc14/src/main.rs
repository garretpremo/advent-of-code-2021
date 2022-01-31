use std::collections::HashMap;
use std::fs;

fn main() {
    let input_file_contents = fs::read_to_string("inputs/input-14.txt").unwrap();

    let (starting_sequence, pairs) = parse(input_file_contents);

    println!("answer 14.1: {}", grow_polymer_and_find_diff(&starting_sequence, &pairs, 10));
    println!("answer 14.2: {}", grow_polymer_and_find_diff(&starting_sequence, &pairs, 40));
}

fn parse(input: String) -> (String, HashMap<String, char>) {

    let lines: Vec<&str> = input.trim()
        .split("\r\n")
        .collect();

    let starting_sequence = String::from(lines[0]);

    let mut pairs: HashMap<String, char> = HashMap::new();

    lines[2..lines.len()].into_iter()
        .for_each(|line| {
            let pair: Vec<&str> = line.split(" -> ").collect();
            let output = pair[1].chars().collect::<Vec<char>>()[0];

            pairs.insert(String::from(pair[0]), output);
        });

    (starting_sequence, pairs)
}

/// finds the difference between the most plentiful element and the least plentiful element after growing the polymer
fn grow_polymer_and_find_diff(starting_sequence: &String, pairs: &HashMap<String, char>, steps: usize) -> u64 {
    let mut element_count_map: HashMap<char, u64> = HashMap::new();
    let mut sequence_map: HashMap<String, u64> = HashMap::new();

    for output in pairs.values() {
        element_count_map.insert(output.clone(), 0);
    }

    // initialize element count map
    for element in starting_sequence.chars() {
        if element_count_map.contains_key(&element) {
            let value = element_count_map.get(&element).unwrap();
            let new_value = value + 1;
            element_count_map.insert(element.clone(), new_value);
        } else {
            element_count_map.insert(element.clone(), 1);
        }
    }

    // initialize sequence map
    for i in 0..starting_sequence.len() - 1 {
        let pair = &starting_sequence[i..=i+1];

        let new_count = match sequence_map.get(pair) { Some(n) => n + 1, _ => 1 };

        sequence_map.insert(String::from(pair), new_count);
    }

    grow_polymer(&sequence_map, pairs, &mut element_count_map, steps);

    let mut min = u64::MAX;
    let mut max = 0;

    for count in element_count_map.values() {
        min = min.min(*count);
        max = max.max(*count);
    }

    max - min
}

fn grow_polymer(sequence_map: &HashMap<String, u64>, pairs: &HashMap<String, char>, element_count_map: &mut HashMap<char, u64>, steps: usize) {
    if steps == 0 { return; }

    let mut new_sequence_map: HashMap<String, u64> = HashMap::new();

    let mut insert_pair = |pair: &String, count: u64| {
        let new_count = match new_sequence_map.get(pair) { Some(n) => n + count, _ => count };
        new_sequence_map.insert(String::from(pair), new_count);
    };

    for (pair, count) in sequence_map {
        let output = pairs.get(pair).unwrap();

        // increment element count
        let element_count = element_count_map.get(output).unwrap();
        let new_element_count = element_count + count;
        element_count_map.insert(output.clone(), new_element_count);

        let part_a = pair.chars().nth(0).unwrap();
        let part_b = pair.chars().nth(1).unwrap();

        insert_pair(&format!("{}{}", part_a, output), *count);
        insert_pair(&format!("{}{}", output, part_b), *count);
    }

    grow_polymer(&new_sequence_map, pairs, element_count_map, steps - 1);
}

#[test]
fn test_sample_input() {
    let sample_input = String::from("NNCB\r\n\r\nCH -> B\r\nHH -> N\r\nCB -> H\r\nNH -> C\r\nHB -> C\r\nHC -> B\r\nHN -> C\r\nNN -> C\r\nBH -> H\r\nNC -> B\r\nNB -> B\r\nBN -> B\r\nBB -> N\r\nBC -> B\r\nCC -> N\r\nCN -> C");
    let (starting_sequence, pairs) = parse(sample_input);

    assert_eq!(grow_polymer_and_find_diff(&starting_sequence, &pairs, 10), 1588);
    assert_eq!(grow_polymer_and_find_diff(&starting_sequence, &pairs, 40), 2188189693529);
}

