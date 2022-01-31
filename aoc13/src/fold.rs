pub enum Fold {
    X(usize),
    Y(usize)
}

impl Fold {
    pub fn from_str(s: &str) -> Fold {
        let instruction: Vec<&str> = s.split("=").collect();
        let coordinate = instruction[1].parse::<usize>().unwrap();

        match instruction[0] {
            instruction if instruction == "fold along y" => Fold::Y(coordinate),
            instruction if instruction == "fold along x" => Fold::X(coordinate),
            _ => panic!("invalid instruction")
        }
    }
}
