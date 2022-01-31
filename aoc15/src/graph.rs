use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

/// use a custom struct to store a danger value, and a point.
/// Implement Ord/PartialOrd so BinaryHeap can sort this struct by total_danger.
///     (effectively making the BinaryHeap a queue)
#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    total_danger: u32,
    point: Point
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.total_danger.cmp(&self.total_danger)
            .then_with(|| self.point.cmp(&other.point))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Hash, Eq, PartialEq, Ord, PartialOrd, Debug, Copy, Clone)]
pub struct Point {
    pub x: usize,
    pub y: usize
}

pub struct Graph {
    nodes: HashMap<Point, u32>,
    width: usize,
    height: usize,
}

impl Graph {
    pub fn new() -> Graph {
        Graph {
            nodes: HashMap::new(),
            width: 0,
            height: 0
        }
    }

    pub fn add_nodes(&mut self, danger_map: &Vec<Vec<u32>>) {
        self.height = danger_map.len();
        self.width = danger_map.first().unwrap().len();

        for (y, row) in danger_map.iter().enumerate() {
            for (x, danger) in row.iter().enumerate() {
                self.nodes.insert(Point { x, y }, danger + 0);
            }
        }
    }

    /// custom impl of dijkstra's algorithm
    ///     Use a BinaryHeap with a custom struct 'State' as a Queue
    pub fn find_least_dangerous_path(&self, source: Point, target: Point) -> u32 {

        let mut danger = vec![vec![u32::MAX; self.width]; self.height];
        let mut queue = BinaryHeap::new();

        danger[source.y][source.x] = 0;
        queue.push(State { total_danger: 0, point: source });

        while let Some(State {total_danger, point}) = queue.pop() {
            if point == target {
                return total_danger;
            }

            if total_danger > danger[point.y][point.x] {
                continue;
            }

            for neighbor in self.get_neighbor_points(&point) {
                let neighbor_danger = self.nodes.get(&neighbor).unwrap() + total_danger;

                if neighbor_danger < danger[neighbor.y][neighbor.x] {
                    danger[neighbor.y][neighbor.x] = neighbor_danger;
                    queue.push(State { total_danger: neighbor_danger, point: neighbor });
                }
            }
        }

        0
    }

    pub fn get_neighbor_points(&self, point: &Point) -> Vec<Point> {
        let mut points = vec![];
        if point.x != 0 {
            points.push(Point { x: point.x - 1, y: point.y });
        }

        if point.x < self.width - 1 {
            points.push(Point { x: point.x + 1, y: point.y });
        }

        if point.y != 0 {
            points.push(Point { x: point.x, y: point.y - 1 });
        }

        if point.y < self.height - 1 {
            points.push(Point { x: point.x, y: point.y + 1 });
        }

        points
    }

    #[allow(dead_code)]
    pub fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                print!("{}", self.nodes.get(&Point{ x, y }).unwrap());
            }
            print!("\n");
        }
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }
}


