mod graph;

use std::fs;
use graph::Graph;
use graph::Edge;

fn main() {
    let input_file_contents = fs::read_to_string("inputs/input-12.txt").unwrap();

    let edges = parse(input_file_contents);
    let mut graph = Graph::new();
    graph.add_edges(&edges);

    println!("answer 12.1: {}", graph.count_distinct_paths());
}

fn parse(input: String) -> Vec<Edge> {
    input.trim()
        .split("\r\n")
        .map(|edge| {
            let edge_split: Vec<String> = edge.split("-").map(|s| String::from(s)).collect();
            Edge::new(edge_split[0].clone(), edge_split[1].clone())
        })
        .collect()
}

#[test]
fn test_sample_input() {
    let sample_input = String::from("start-A\r\nstart-b\r\nA-c\r\nA-b\r\nb-d\r\nA-end\r\nb-end");
    let edges = parse(sample_input);

    let mut graph = Graph::new();
    graph.add_edges(&edges);

    assert_eq!(graph.count_distinct_paths(), 10);
}

#[test]
fn test_complex_sample_input() {
    let sample_input = String::from("fs-end\r\nhe-DX\r\nfs-he\r\nstart-DX\r\npj-DX\r\nend-zg\r\nzg-sl\r\nzg-pj\r\npj-he\r\nRW-he\r\nfs-DX\r\npj-RW\r\nzg-RW\r\nstart-pj\r\nhe-WI\r\nzg-he\r\npj-fs\r\nstart-RW");
    let edges = parse(sample_input);

    let mut graph = Graph::new();
    graph.add_edges(&edges);

    assert_eq!(graph.count_distinct_paths(), 226);
}

