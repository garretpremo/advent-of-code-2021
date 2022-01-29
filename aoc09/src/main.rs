use std::fs;
use std::collections::HashSet;

#[derive(Hash, Eq, PartialEq, Clone)]
struct Point { row: usize, col: usize }

fn main() {
    let input_file_contents = fs::read_to_string("inputs/input-09.txt").unwrap();

    let height_map: Vec<Vec<u32>> = parse(input_file_contents);
    let low_points = find_low_points(&height_map);

    println!("answer 9.1: {}", calculate_risk(&height_map, &low_points));
    println!("answer 9.2: {}", calculate_size_of_three_largest_basins(&height_map, &low_points));
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
                low_points.push(Point { row, col })
            }
        }
    }

    low_points
}

fn calculate_risk(height_map: &Vec<Vec<u32>>, low_points: &Vec<Point>) -> u32 {
    low_points.iter()
        .fold(0, |acc, Point { row: x, col: y }| acc + height_map[*x][*y] + 1)
}

fn calculate_size_of_three_largest_basins(height_map: &Vec<Vec<u32>>, low_points: &Vec<Point>) -> u32 {
    let mut basin_sizes = vec![];

    for point in low_points {
        basin_sizes.push(calculate_basin_size(height_map, point))
    }

    basin_sizes.sort();

    let mut size = 1;

    for i in basin_sizes.len()-3..basin_sizes.len() {
        size = size * basin_sizes[i];
    }

    size
}

fn calculate_basin_size(height_map: &Vec<Vec<u32>>, point: &Point) -> u32 {
    let mut point_set = HashSet::new();

    count_larger_adjacent_points(height_map, point, &mut point_set);

    point_set.len() as u32
}

fn count_larger_adjacent_points(height_map: &Vec<Vec<u32>>, point: &Point, point_set: &mut HashSet<Point>) {
    let rows = height_map.len();
    let cols = height_map.first().unwrap().len();
    let height = height_map[point.row][point.col];
    let mut points_to_check = vec![];

    if point.row != 0 {
        points_to_check.push(Point { row: point.row - 1, col: point.col });
    }

    if point.row < rows - 1 {
        points_to_check.push(Point { row: point.row + 1, col: point.col, });
    }

    if point.col != 0 {
        points_to_check.push(Point { row: point.row, col: point.col - 1});
    }

    if point.col < cols - 1 {
        points_to_check.push(Point { row: point.row, col: point.col + 1});
    }

    for point_to_check in points_to_check.iter() {
        let height_to_check = height_map[point_to_check.row][point_to_check.col];

        if height < height_to_check && height_to_check < 9 {
            count_larger_adjacent_points(height_map, point_to_check, point_set);
        }
    }

    point_set.insert(point.clone());
}

#[test]
fn test_sample_input() {
    let sample_input = String::from("2199943210\r\n3987894921\r\n9856789892\r\n8767896789\r\n9899965678");

    let height_map: Vec<Vec<u32>> = parse(sample_input);

    assert_eq!(height_map.len(), 5);

    let low_points = find_low_points(&height_map);
    assert_eq!(low_points.len(), 4);

    let risk = calculate_risk(&height_map, &low_points);
    assert_eq!(risk, 15);

    let basin_size = calculate_basin_size(&height_map, &low_points[0]);
    assert_eq!(basin_size, 3);
    let basin_size = calculate_basin_size(&height_map, &low_points[1]);
    assert_eq!(basin_size, 9);

    assert_eq!(calculate_size_of_three_largest_basins(&height_map, &low_points), 1134);
}
