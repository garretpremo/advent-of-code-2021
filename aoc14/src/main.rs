use std::collections::HashMap;
use std::fs;

fn main() {
    let input_file_contents = fs::read_to_string("inputs/input-14.txt").unwrap();

    let (starting_sequence, pairs) = parse(input_file_contents);

    println!("answer 14.1: {}", grow_polymer_and_find_diff(&starting_sequence, &pairs, 10));
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
fn grow_polymer_and_find_diff(starting_sequence: &String, pairs: &HashMap<String, char>, steps: usize) -> u32 {
    let mut element_count_map: HashMap<char, u32> = HashMap::new();

    for output in pairs.values() {
        element_count_map.insert(output.clone(), 0);
    }

    for element in starting_sequence.chars() {
        if element_count_map.contains_key(&element) {
            let value = element_count_map.get(&element).unwrap();
            let new_value = value + 1;
            element_count_map.insert(element.clone(), new_value);
        } else {
            element_count_map.insert(element.clone(), 1);
        }
    }

    grow_polymer(starting_sequence, pairs, &mut element_count_map, steps);

    let mut min = u32::MAX;
    let mut max = 0;

    for count in element_count_map.values() {
        min = min.min(*count);
        max = max.max(*count);
    }

    max - min
}

fn grow_polymer(sequence: &String, pairs: &HashMap<String, char>, element_count_map: &mut HashMap<char, u32>, steps: usize) {
    if steps == 0 { return; }

    let mut new_sequence = String::from("");
    for i in 0..sequence.len() - 1 {
        let pair = &sequence[i..=i+1];
        let output = pairs.get(pair).unwrap();

        let count = element_count_map.get(output).unwrap();
        let new_count = count + 1;
        element_count_map.insert(output.clone(), new_count);

        let part_a = pair.chars().nth(0).unwrap();
        let part_b = pair.chars().nth(1).unwrap();

        if i == 0 {
            new_sequence = format!("{}{}{}{}", new_sequence, part_a, output, part_b);
        } else {
            new_sequence = format!("{}{}{}", new_sequence, output, part_b);
        }
    }

    grow_polymer(&new_sequence, pairs, element_count_map, steps - 1);
}

#[test]
fn test_sample_input() {
    let sample_input = String::from("NNCB\r\n\r\nCH -> B\r\nHH -> N\r\nCB -> H\r\nNH -> C\r\nHB -> C\r\nHC -> B\r\nHN -> C\r\nNN -> C\r\nBH -> H\r\nNC -> B\r\nNB -> B\r\nBN -> B\r\nBB -> N\r\nBC -> B\r\nCC -> N\r\nCN -> C");
    let (starting_sequence, pairs) = parse(sample_input);

    assert_eq!(grow_polymer_and_find_diff(&starting_sequence, &pairs, 10), 1588);
}

