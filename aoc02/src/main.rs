use std::fs;

enum Direction { Forward, Down, Up }

struct Command {
    direction: Direction,
    units: u32
}

fn main() {
    let input_file_contents = fs::read_to_string("inputs/input-02.txt").unwrap();

    let commands: Vec<Command> = input_file_contents
        .trim()
        .split("\r\n")
        .map(|value| {
            let command_string = value.split(" ").collect::<Vec<&str>>();

            let direction = match command_string[0] {
                "forward" => Direction::Forward,
                "down" =>    Direction::Down,
                "up" =>      Direction::Up,
                _ =>         panic!("invalid direction")
            };

            let units = command_string[1].parse::<u32>().unwrap();

            Command { direction, units }
        })
        .collect();

    println!("answer 2.1: {}", calculate_position(&commands));
}

/// position is calculated by using a series of commands
///
/// Forward: increase distance by X units
/// Down: increase depth by X units
/// Up: decrease depth by X units
///
/// Returns distance * depth
fn calculate_position(commands: &Vec<Command>) -> u32 {
    let mut distance = 0;
    let mut depth = 0;

    for command in commands {
        match command {
            Command { direction: Direction::Forward, units } => distance += units,
            Command { direction: Direction::Down, units } => depth += units,
            Command { direction: Direction::Up, units } => depth -= units,
        }
    }

    distance * depth
}
