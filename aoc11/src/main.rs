use std::fs;

struct IndexRanges {
    row_min: usize,
    row_max: usize,
    col_min: usize,
    col_max: usize
}

struct Cell {
    row: usize,
    col: usize
}

fn main() {
    let input_file_contents = fs::read_to_string("inputs/input-11.txt").unwrap();

    let matrix = parse(input_file_contents);

    println!("answer 11.1: {}", count_flashes(&matrix, 100));
    println!("answer 11.2: {}", calculate_first_synchronization_step(&matrix));
}

fn parse(input: String) -> Vec<Vec<u32>> {
    let mut matrix = vec![];

    input.trim()
        .split("\r\n")
        .for_each(|string| {
            matrix.push(string.chars()
                .map(|height| height.to_digit(10).unwrap())
                .collect());
        });

    matrix
}

fn calculate_first_synchronization_step(matrix: &Vec<Vec<u32>>) -> u32 {
    let height = matrix.len() as u32;
    let width = matrix.first().unwrap().len() as u32;
    let mut mutable_matrix = matrix.clone();
    let mut step = 1;

    loop {
        match do_step(&mut mutable_matrix) {
            flashes if flashes == height * width => break,
            _ => {}
        }

        step += 1;
    }

    step
}

fn count_flashes(matrix: &Vec<Vec<u32>>, steps: usize) -> u32 {
    let mut flashes = 0;
    let mut mutable_matrix = matrix.clone();

    for _step in 0..steps {
        flashes += do_step(&mut mutable_matrix);
    }

    flashes
}

fn do_step(matrix: &mut Vec<Vec<u32>>) -> u32 {
    let mut flashes = 0;

    // first, increment all by 1
    for matrix_row in matrix.iter_mut() {
        for energy in matrix_row.iter_mut() {
            *energy += 1;
        }
    }

    // for any nodes > 9, cause them to increment all neighbors by 1
    let mut cells_incrementing_neighbors = vec![];

    for (row, matrix_row) in matrix.iter().enumerate() {
        for (col, energy) in matrix_row.iter().enumerate() {
            if *energy == 10 {
                cells_incrementing_neighbors.push(Cell { row, col });
            }
        }
    }

    for Cell { row, col } in cells_incrementing_neighbors {
        increment_neighbors(matrix, row, col);
    }

    // set any node with energy > 9 back to 0 and increment flashes
    for matrix_row in matrix.iter_mut() {
        for energy in matrix_row.iter_mut() {
            if *energy > 9 {
                *energy = 0;
                flashes += 1;
            }
        }
    }

    flashes
}

fn increment_neighbors(matrix: &mut Vec<Vec<u32>>, row: usize, col: usize) {
    let IndexRanges { row_min, row_max, col_min, col_max } = get_neighbor_index_ranges(matrix.len(), matrix[row].len(), row, col);

    for i in row_min..=row_max {
        for j in col_min..=col_max {
            if row == i &&  col == j {
                continue;
            }

            if matrix[i][j] < 9 {
                matrix[i][j] += 1;
            } else if matrix[i][j] == 9 {
                // since count_flashes() checks for 10, intentionally set energy to 11 here
                // so this isn't called twice for the same node
                matrix[i][j] += 2;
                increment_neighbors(matrix, i, j);
            }
        }
    }
}

fn get_neighbor_index_ranges(height: usize, width: usize, row: usize, col: usize) -> IndexRanges {
    let row_min = match row == 0 { true => 0, false => row - 1 };
    let row_max = match row == height - 1 { true => row, false => row + 1 };
    let col_min = match col == 0 { true => 0, false => col - 1 };
    let col_max = match col == width - 1 { true => col, false => col + 1 };

    IndexRanges { row_min, row_max, col_min, col_max }
}

#[allow(dead_code)]
fn print_matrix(matrix: &Vec<Vec<u32>>) {
    println!();
    for row in matrix.iter() {
        for energy in row.iter() {
            print!("{}", match *energy { energy if energy < 10 => energy, _ => 9});
        }
        print!("\n");
    }
}

#[test]
fn test_sample_input() {
    let sample_input = String::from("5483143223\r\n2745854711\r\n5264556173\r\n6141336146\r\n6357385478\r\n4167524645\r\n2176841721\r\n6882881134\r\n4846848554\r\n5283751526");
    let sample_matrix = parse(sample_input);

    assert_eq!(count_flashes(&sample_matrix, 10), 204);
    assert_eq!(count_flashes(&sample_matrix, 100), 1656);
    assert_eq!(calculate_first_synchronization_step(&sample_matrix), 195);
}

