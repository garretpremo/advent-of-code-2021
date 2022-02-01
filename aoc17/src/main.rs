use std::fs;

struct Point { x: u32, y: i64 }
struct Rectangle { top_left: Point, bottom_right: Point }

impl Rectangle {
    fn is_within(&self, p: &Point) -> bool {
        return p.x >= self.top_left.x && p.y <= self.top_left.y
            && p.x <= self.bottom_right.x && p.y >= self.bottom_right.y
    }

    fn is_beyond(&self, p: &Point) -> bool {
        p.x > self.bottom_right.x || p.y < self.bottom_right.y
    }
}

struct Probe {
    position: Point,
    x_velocity: u32,
    y_velocity: i64,
}

impl Probe {
    fn new(x_velocity: u32, y_velocity: i64) -> Probe {
        Probe { position: Point { x: 0, y: 0 }, x_velocity, y_velocity }
    }

    fn simulate_step(&mut self) {
        self.position.x += self.x_velocity;
        self.position.y += self.y_velocity;

        self.x_velocity = self.x_velocity.saturating_sub(1);
        self.y_velocity -= 1;
    }
}

fn main() {
    let input_file_contents = fs::read_to_string("inputs/input-17.txt").unwrap();

    let target = parse(input_file_contents);

    println!("answer 17.1: {}", find_highest_possible_y_position(&target));
}

fn parse(input: String) -> Rectangle {
    let target = input.trim().split(": ").collect::<Vec<&str>>()[1];
    let coordinates = target.split(", ").collect::<Vec<&str>>();
    let x_coordinates = coordinates[0][2..coordinates[0].len()]
        .split("..")
        .map(|coord| coord.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let y_coordinates = coordinates[1][2..coordinates[1].len()]
        .split("..")
        .map(|coord| coord.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    Rectangle {
        top_left:       Point { x: x_coordinates[0], y: y_coordinates[1] },
        bottom_right:   Point { x: x_coordinates[1], y: y_coordinates[0] }
    }
}

fn find_highest_possible_y_position(target: &Rectangle) -> i64 {
    let x_min = target.top_left.x;
    let x_max = target.bottom_right.x;

    // assuming y is always negative
    let y_max = i64::abs(target.bottom_right.y) + 1;

    let mut min_x_velocity = 1;
    while nth_triangle_number(min_x_velocity) < x_min {
        min_x_velocity += 1;
    }
    let mut max_x_velocity = min_x_velocity;
    while nth_triangle_number(max_x_velocity) < x_max {
        max_x_velocity += 1;
    }

    let mut overall_max_y = 0;

    for x_vel in min_x_velocity..=max_x_velocity {
        for y_vel in 0..y_max {
            let mut probe = Probe::new(x_vel, y_vel);
            let mut max_y = overall_max_y.clone();

            loop {
                probe.simulate_step();
                max_y = max_y.max(probe.position.y);

                if target.is_within(&probe.position) {
                    overall_max_y = overall_max_y.max(max_y.clone());
                    break;
                }

                if target.is_beyond(&probe.position) {
                    break;
                }
            }
        }
    }

    overall_max_y
}

fn nth_triangle_number(n: u32) -> u32 {
    (n * (n + 1)) / 2
}

#[test]
fn test_sample_input() {
    let sample_input = String::from("target area: x=20..30, y=-10..-5");
    let target = parse(sample_input);

    assert_eq!(target.top_left.x, 20);
    assert_eq!(target.top_left.y, -5);
    assert_eq!(target.bottom_right.x, 30);
    assert_eq!(target.bottom_right.y, -10);

    assert_eq!(find_highest_possible_y_position(&target), 45);
}
