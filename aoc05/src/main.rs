use std::fs;
use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq, Debug)]
struct Point { x: u32, y: u32 }
struct Line { start: Point, end: Point }

impl Point {
    fn from_str(string: &str) -> Point {
        let coordinates: Vec<u32> = string
            .split(",")
            .map(|coordinate| coordinate.parse::<u32>().unwrap())
            .collect();

        Point { x: coordinates[0], y: coordinates[1] }
    }
}

fn main() {
    let input_file_contents = fs::read_to_string("inputs/input-05.txt").unwrap();

    let lines: Vec<Line> = input_file_contents
        .trim()
        .split("\r\n")
        .map(|line| {
            let coordinates: Vec<&str> = line.split(" -> ").collect();

            let start = Point::from_str(coordinates[0]);
            let end = Point::from_str(coordinates[1]);

            Line { start, end }
        })
        .collect();

    println!("answer 5.1: {}", calculate_intersections(&lines));
}

fn calculate_intersections(lines: &Vec<Line>) -> u32 {
    let mut intersection_counts = 0;
    let mut coordinate_count_map: HashMap<Point, u32> = HashMap::new();

    let mut increment_coordinate_map = |point: Point| {
        if coordinate_count_map.contains_key(&point) {
            let old_count = coordinate_count_map.get(&point).unwrap();
            let new_count = old_count + 1;
            coordinate_count_map.insert(point, new_count);

            if new_count == 2 {
                intersection_counts += 1;
            }
        } else {
            coordinate_count_map.insert(point, 1);
        }
    };

    for Line { start, end } in lines {
        if start.x != end.x && start.y != end.y {
            continue;
        }

        if start.x == end.x {
            let start_y = u32::min(start.y, end.y);
            let end_y = u32::max(start.y, end.y);

            for i in start_y..=end_y {
                increment_coordinate_map(Point { x: start.x, y: i });
            }
        } else if start.y == end.y {
            let start_x = u32::min(start.x, end.x);
            let end_x = u32::max(start.x, end.x);

            for i in start_x..=end_x {
                increment_coordinate_map(Point { x: i, y: start.y });
            }
        }
    }

    intersection_counts
}

#[test]
fn test_point_map() {
    let mut coordinate_count_map: HashMap<Point, u32> = HashMap::new();

    let p1 = Point { x: 15, y: 10 };
    coordinate_count_map.insert(p1, 1);

    assert!(coordinate_count_map.contains_key(&Point { x: 15, y: 10 }));
}

#[test]
fn test_sample_input() {
    let sample_input = vec![
        Line { start: Point { x: 0, y: 9 }, end: Point { x: 5, y: 9 } },
        Line { start: Point { x: 8, y: 0 }, end: Point { x: 0, y: 8 } },
        Line { start: Point { x: 9, y: 4 }, end: Point { x: 3, y: 4 } },
        Line { start: Point { x: 2, y: 2 }, end: Point { x: 2, y: 1 } },
        Line { start: Point { x: 7, y: 0 }, end: Point { x: 7, y: 4 } },
        Line { start: Point { x: 6, y: 4 }, end: Point { x: 2, y: 0 } },
        Line { start: Point { x: 0, y: 9 }, end: Point { x: 2, y: 9 } },
        Line { start: Point { x: 3, y: 4 }, end: Point { x: 1, y: 4 } },
        Line { start: Point { x: 0, y: 0 }, end: Point { x: 8, y: 8 } },
        Line { start: Point { x: 5, y: 5 }, end: Point { x: 8, y: 2 } },
    ];

    assert_eq!(calculate_intersections(&sample_input), 5);
}
