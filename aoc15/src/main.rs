mod graph;

use std::fs;
use graph::Graph;
use crate::graph::Point;

fn main() {
    let input_file_contents = fs::read_to_string("inputs/input-15.txt").unwrap();

    let danger_map = parse(input_file_contents);

    println!("answer 15.1: {}", find_least_dangerous_path(&danger_map));
}

fn parse(input: String) -> Vec<Vec<u32>> {
    input.trim()
        .split("\r\n")
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

fn find_least_dangerous_path(danger_map: &Vec<Vec<u32>>) -> u32 {
    let mut graph = Graph::new();
    graph.add_nodes(&danger_map);

    let start = Point{ x: 0, y: 0 };
    let end = Point { x: graph.get_width() - 1, y: graph.get_height() - 1 };

    graph.find_least_dangerous_path(start, end)
}

#[test]
fn test_sample_input() {
    let sample_input = String::from("1163751742\r\n1381373672\r\n2136511328\r\n3694931569\r\n7463417111\r\n1319128137\r\n1359912421\r\n3125421639\r\n1293138521\r\n2311944581");
    let danger_map = parse(sample_input);

    let mut graph = Graph::new();
    graph.add_nodes(&danger_map);

    let start = Point{ x: 0, y: 0 };
    let end = Point { x: graph.get_width() - 1, y: graph.get_height() - 1 };

    assert_eq!(graph.find_least_dangerous_path(start, end), 40);
}
