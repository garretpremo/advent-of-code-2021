use std::fs;
use std::collections::HashSet;

#[derive(Hash, Eq, PartialEq)]
struct Point { x: usize, y: usize }

fn main() {
    let input_file_contents = fs::read_to_string("inputs/input-09.txt").unwrap();

    let height_map: Vec<Vec<u32>> = parse(input_file_contents);
    let low_points = find_low_points(&height_map);

    println!("answer 9.1: {}", calculate_risk(&height_map, &low_points));
}

fn parse(input: String) -> Vec<Vec<u32>> {
    let mut height_map = vec![];

    input.trim()
        .split("\r\n")
        .for_each(|row| {
            height_map.push(row.chars()
                .map(|height| height.to_digit(10).unwrap())
                .collect());
        });

    height_map
}

fn find_low_points(height_map: &Vec<Vec<u32>>) -> Vec<Point> {
    let rows = height_map.len();
    let cols = height_map.first().unwrap().len();

    let mut low_points = vec![];

    for (row, height_map_row) in height_map.iter().enumerate() {
        for (col, height) in height_map_row.iter().enumerate() {
            let above = match row {
                0 => height + 1,
                _ => height_map[row - 1][col]
            };

            let below = match row + 1 {
                row if row == rows => height + 1,
                _ => height_map[row + 1][col]
            };

            let left = match col {
                0 => height + 1,
                _ => height_map[row][col - 1]
            };

            let right = match col + 1 {
                col if col == cols => height + 1,
                _ => height_map[row][col + 1]
            };

            if height < &above && height < &below && height < &left && height < &right {
                low_points.push(Point { x: row, y: col })
            }
        }
    }

    low_points
}

fn calculate_risk(height_map: &Vec<Vec<u32>>, low_points: &Vec<Point>) -> u32 {
    low_points.iter()
        .fold(0, |acc, Point {x,y}| acc + height_map[*x][*y] + 1)
}

#[test]
fn test_sample_input() {
    let sample_input = String::from("2199943210\r\n3987894921\r\n9856789892\r\n8767896789\r\n9899965678");

    let sample_input: Vec<Vec<u32>> = parse(sample_input);

    assert_eq!(sample_input.len(), 5);

    let low_points = find_low_points(&sample_input);
    assert_eq!(low_points.len(), 4);

    let risk = calculate_risk(&sample_input, &low_points);
    assert_eq!(risk, 15);
}
