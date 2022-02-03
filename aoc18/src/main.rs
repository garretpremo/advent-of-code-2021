use std::fs;

#[derive(Clone, Debug, Eq, PartialEq)]
enum SnailNum {
    Nest(Box<SnailNumber>),
    Num(u32),
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct SnailNumber {
    left: SnailNum,
    right: SnailNum,
}

#[derive(Eq, PartialEq, Debug)]
enum Explode {
    DoExplode(u32, u32),
    AddLeft(u32),
    AddRight(u32),
    Added,
    None
}

#[derive(Eq, PartialEq, Debug)]
enum SplitResult {
    Split,
    None
}

impl SnailNum {
    fn try_explode(&mut self, depth: usize) -> Explode {
        match self {
            SnailNum::Nest(n) => n.try_explode(depth),
            _ => Explode::None
        }
    }

    fn is_nested(&self) -> bool {
        match self {
            SnailNum::Nest(_) => true,
            _ => false
        }
    }
}

impl SnailNumber {
    fn add(&self, other: &SnailNumber) -> SnailNumber {
        let mut sum = SnailNumber {
            left: SnailNum::Nest(Box::new(self.clone())),
            right: SnailNum::Nest(Box::new(other.clone()))
        };

        sum.reduce();

        sum
    }

    fn reduce(&mut self) -> u32 {
        let mut reductions = 0;

        loop {
            match self.try_explode(0) {
                Explode::None => {},
                _ => {
                    reductions += 1;
                    continue;
                }
            }

            match self.try_split() {
                SplitResult::None => break,
                _ => reductions += 1
            }
        }

        reductions
    }

    fn try_explode(&mut self, depth: usize) -> Explode {
        if depth >= 4 {
            match (&self.left, &self.right) {
                (SnailNum::Num(l), SnailNum::Num(r)) => return Explode::DoExplode(*l, *r),
                _ => {}
            }
        }

        let left_is_nested = self.left.is_nested();

        if left_is_nested {
            let left_explode = self.left.try_explode(depth + 1);

            match left_explode {
                Explode::DoExplode(left, right) => {
                    self.left = SnailNum::Num(0);
                    self.add_to_first_on_the_right(right, true);
                    return Explode::AddLeft(left)
                }
                Explode::AddLeft(n) => return Explode::AddLeft(n),
                Explode::AddRight(n) => {
                    self.add_to_first_on_the_right(n, true);
                    return Explode::Added;
                },
                Explode::Added => return Explode::Added,
                Explode::None => {}
            }
        }

        let right_is_nested = self.right.is_nested();

        if right_is_nested {
            let right_explode = self.right.try_explode(depth + 1);

            match right_explode {
                Explode::DoExplode(left, right) => {
                    self.right = SnailNum::Num(0);
                    self.add_to_first_on_the_left(left, true);
                    return Explode::AddRight(right);
                },
                Explode::AddRight(n) => return Explode::AddRight(n),
                Explode::AddLeft(n) => {
                    self.add_to_first_on_the_left(n, true);
                    return Explode::Added;
                },
                Explode::Added => return Explode::Added,
                Explode::None => {}
            }
        }

        Explode::None
    }

    fn add_to_first_on_the_left(&mut self, value: u32, ignore_first_right: bool) {
        if ignore_first_right {
            match &mut self.left {
                SnailNum::Nest(l) => l.add_to_first_on_the_left(value, false),
                SnailNum::Num(mut _value) => self.left = SnailNum::Num(_value + value)
            }
            return;
        }

        match &mut self.right {
            SnailNum::Nest(r) => r.add_to_first_on_the_left(value, false),
            SnailNum::Num(mut _value) => self.right = SnailNum::Num(_value + value)
        }
    }

    fn add_to_first_on_the_right(&mut self, value: u32, ignore_first_left: bool) {
        if ignore_first_left {
            match &mut self.right {
                SnailNum::Nest(r) => r.add_to_first_on_the_right(value, false),
                SnailNum::Num(mut _value) => self.right = SnailNum::Num(_value + value)
            }
            return;
        }

        match &mut self.left {
            SnailNum::Nest(l) => l.add_to_first_on_the_right(value, false),
            SnailNum::Num(mut _value) => self.left = SnailNum::Num(_value + value)
        }
    }

    fn try_split(&mut self) -> SplitResult {

        let left_split = match &mut self.left {
            SnailNum::Num(n) => {
                if *n >= 10 {
                    self.left = split_number(n);
                    return SplitResult::Split;
                }
                SplitResult::None
            },
            SnailNum::Nest(nested) => nested.try_split()
        };

        if left_split == SplitResult::Split {
            return SplitResult::Split;
        }

        match &mut self.right {
            SnailNum::Num(n) => {
                if *n >= 10 {
                    self.right = split_number(n);
                    return SplitResult::Split;
                }
                return SplitResult::None
            },
            SnailNum::Nest(nested) => return nested.try_split()
        };
    }

    fn calculate_magnitude(&self) -> u32 {
        let left_magnitude = match &self.left {
            SnailNum::Num(value) => value * 3,
            SnailNum::Nest(nested_value) => nested_value.calculate_magnitude() * 3
        };

        let right_magnitude = match &self.right {
            SnailNum::Num(value) => value * 2,
            SnailNum::Nest(nested_value) => nested_value.calculate_magnitude() * 2
        };

        left_magnitude + right_magnitude
    }
}

fn main() {
    let input_file_contents = fs::read_to_string("inputs/input-18.txt").unwrap();

    let numbers = parse(input_file_contents);

    println!("18.1 answer: {}", add_snail_numbers(&numbers));
}

fn parse(input: String) -> Vec<SnailNumber> {
    input.trim()
        .split("\r\n")
        .map(|line| create_snail_number_from_string(line))
        .collect()
}

fn add_snail_numbers(numbers: &Vec<SnailNumber>) -> u32 {

    let mut results: Vec<SnailNumber> = vec![];

    for i in 0..numbers.len() - 1 {
        let a = match i {
            0 => &numbers[0],
            _ => &results[i-1]
        };

        let b = &numbers[i+1];

        let sum = a.add(b);
        results.push(sum);
    }

    results[results.len() - 1].calculate_magnitude()
}

fn create_snail_number_from_string(string: &str) -> SnailNumber {
    let left_is_nested = match string.chars().nth(1).unwrap() { '[' => true, _ => false };
    let left_length = get_length_of_snail_number_string(&string[1..string.len()]);

    let left = match left_is_nested {
        true => {
            SnailNum::Nest(Box::new(create_snail_number_from_string(&string[1..1+left_length])))
        },
        false => {
            let left_value = string[1..1+left_length].parse::<u32>().unwrap();
            SnailNum::Num(left_value)
        }
    };

    let right_start = left_length + 2;

    let right_is_nested = match string.chars().nth(right_start).unwrap() { '[' => true, _ => false };
    let right_length = get_length_of_snail_number_string(&string[right_start..string.len()]);

    let right = match right_is_nested {
        true => SnailNum::Nest(Box::new(create_snail_number_from_string(&string[right_start..right_start+right_length]))),
        false => {
            let right_value = string[right_start..right_start+right_length].parse::<u32>().unwrap();
            SnailNum::Num(right_value)
        }
    };

    SnailNumber { left, right }
}

fn get_length_of_snail_number_string(string: &str) -> usize {
    let mut length = 0;
    let mut break_at_comma = false;
    let mut opening_brackets = 0;
    let mut closing_brackets = 0;

    for (index, c) in string.chars().enumerate() {
        match c {
            '[' => opening_brackets += 1,
            ']' => closing_brackets += 1,
            ',' if break_at_comma => break,
            _ if index == 0 => break_at_comma = true,
            _ => {}
        }

        if closing_brackets > opening_brackets { break; }

        length += 1;

        if opening_brackets > 0 && opening_brackets == closing_brackets {
            break;
        }
    }

    length
}

fn split_number(n: &u32) -> SnailNum {
    let half = n / 2;
    let half_rounded_up = half + (n % 2);
    let split = format!("[{},{}]", half, half_rounded_up);
    SnailNum::Nest(Box::new(create_snail_number_from_string(split.as_str())))
}

#[test]
fn test_sample_input() {
    let sample_input = String::from("[[[[[9,8],1],2],3],4]");
    parse(sample_input);
}

#[test]
fn test_reduce() {
    let a = create_snail_number_from_string("[[[[4,3],4],4],[7,[[8,4],9]]]");
    let b = create_snail_number_from_string("[1,1]");

    let mut c = a.add(&b);

    c.reduce();

    let result = create_snail_number_from_string("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");

    assert_eq!(c, result);
}

#[test]
fn test_parse() {
    let input = "[[1,2],[3,4]]";
    let snail_num = create_snail_number_from_string(input);
    let should_be = SnailNumber {
        left: SnailNum::Nest(Box::new(SnailNumber {
            left: SnailNum::Num(1),
            right: SnailNum::Num(2)
        })),
        right: SnailNum::Nest(Box::new(SnailNumber {
            left: SnailNum::Num(3),
            right: SnailNum::Num(4)
        })),
    };

    assert_eq!(snail_num, should_be);
}

#[test]
fn test_double_digit_parse() {
    create_snail_number_from_string("[10,11]");
}

#[test]
fn test_add() {
    let a = create_snail_number_from_string("[1,2]");
    let b = create_snail_number_from_string("[[3,4],5]");

    let c = a.add(&b);

    let c_should_be = create_snail_number_from_string("[[1,2],[[3,4],5]");

    assert_eq!(c, c_should_be);
}

#[test]
fn test_split() {
    let mut a = create_snail_number_from_string("[10,11]");

    a.try_split();
    a.try_split();

    println!("{:?}", a);
}

#[test]
fn test_magnitude() {
    let a = create_snail_number_from_string("[[9,1],[1,9]]");
    let b = create_snail_number_from_string("[[1,2],[[3,4],5]]");
    let c = create_snail_number_from_string("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
    let d = create_snail_number_from_string("[[[[1,1],[2,2]],[3,3]],[4,4]]");
    let e = create_snail_number_from_string("[[[[3,0],[5,3]],[4,4]],[5,5]]");
    let f = create_snail_number_from_string("[[[[5,0],[7,4]],[5,5]],[6,6]]");
    let g = create_snail_number_from_string("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]");

    assert_eq!(a.calculate_magnitude(), 129);
    assert_eq!(b.calculate_magnitude(), 143);
    assert_eq!(c.calculate_magnitude(), 1384);
    assert_eq!(d.calculate_magnitude(), 445);
    assert_eq!(e.calculate_magnitude(), 791);
    assert_eq!(f.calculate_magnitude(), 1137);
    assert_eq!(g.calculate_magnitude(), 3488);
}

#[test]
fn test_complex_example_1() {
    let a = create_snail_number_from_string("[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]");
    let b = create_snail_number_from_string("[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]");

    let sum = a.add(&b);
    assert_eq!(sum, create_snail_number_from_string("[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]"));
}

#[test]
fn test_complex_example() {
    let input = String::from("[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]\r\n[[[5,[2,8]],4],[5,[[9,9],0]]]\r\n[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]\r\n[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]\r\n[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]\r\n[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]\r\n[[[[5,4],[7,7]],8],[[8,3],8]]\r\n[[9,3],[[9,9],[6,[4,9]]]]\r\n[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]\r\n[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]");
    let numbers = parse(input);

    assert_eq!(add_snail_numbers(&numbers), 4140);
}

#[test]
fn test_simple_adding() {
    let input = String::from("[1,1]\r\n[2,2]\r\n[3,3]\r\n[4,4]\r\n[5,5]\r\n[6,6]");
    let numbers = parse(input);

    add_snail_numbers(&numbers);
}
