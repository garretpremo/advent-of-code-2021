use std::collections::HashSet;

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub struct Point {
    x: usize,
    y: usize
}

impl Point {
    pub fn from_str(s: &str) -> Point {
        let coordinates: Vec<usize> = s.split(",").map(|s| s.parse::<usize>().unwrap()).collect();
        Point { x: coordinates[0], y: coordinates[1] }
    }
}

pub struct Paper {
    dots: HashSet<Point>
}

impl Paper {
    pub fn new() -> Paper {
        Paper { dots: HashSet::new() }
    }

    pub fn add_dots(&mut self, points: &Vec<Point>) {
        for point in points {
            self.dots.insert(point.clone());
        }
    }

    pub fn fold_up(&mut self, y: usize) {
        let mut dots_to_fold = vec![];
        let mut dots_on_fold = vec![];

        for dot in self.dots.iter() {
            if dot.y > y {
                dots_to_fold.push(dot.clone());
            } else if dot.y == y {
                dots_on_fold.push(dot.clone());
            }
        }

        for dot in dots_on_fold {
            self.dots.remove(&dot);
        }

        for dot in dots_to_fold {
            let diff = dot.y - y;
            self.dots.remove(&dot);

            if diff <= y {
                self.dots.insert(Point { x: dot.x, y: y - diff });
            }
        }
    }

    pub fn fold_left(&mut self, x: usize) {
        let mut dots_to_fold = vec![];
        let mut dots_on_fold = vec![];

        for dot in self.dots.iter() {
            if dot.x > x {
                dots_to_fold.push(dot.clone());
            } else if dot.x == x {
                dots_on_fold.push(dot.clone());
            }
        }

        for dot in dots_on_fold {
            self.dots.remove(&dot);
        }

        for dot in dots_to_fold {
            let diff = dot.x - x;
            self.dots.remove(&dot);

            if diff <= x {
                self.dots.insert(Point { x: x - diff, y: dot.y });
            }
        }
    }

    pub fn count_dots(&self) -> usize {
        self.dots.len()
    }

    pub fn print_dots(&self) {
        let mut max_x = 0;
        let mut max_y = 0;

        for dot in self.dots.iter() {
            max_x = max_x.max(dot.x);
            max_y = max_y.max(dot.y);
        }

        for y in 0..=max_y {
            for x in 0..=max_x {
                if self.dots.contains(&Point { x, y }) {
                    print!("#");
                } else {
                    print!(" ");
                }
            }
            print!("\n");
        }
    }
}
