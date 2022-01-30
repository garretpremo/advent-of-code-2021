#[derive(PartialEq, Debug)]
pub enum SyntaxCheckerResult {
    Ok(usize), // length of the result
    Incomplete,
    Corrupt(char)
}

pub fn get_illegality_score(c: char) -> u32 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("invalid character detected: {}", c)
    }
}

pub fn check(line: &str) -> SyntaxCheckerResult {
    let mut target_i = 0;
    for (i, c) in line.chars().enumerate() {
        if i != target_i {
            continue;
        }

        match check_closing_char(&line[i +1..line.len()], c) {
            SyntaxCheckerResult::Ok(length) => target_i += length,
            result => return result
        }

        target_i += 1;
    }

    SyntaxCheckerResult::Ok(line.len())
}

fn check_closing_char(slice: &str, starting_char: char) -> SyntaxCheckerResult {
    let mut target_i = 0;
    for (i, c) in slice.chars().enumerate() {
        if i != target_i {
            continue;
        }

        if is_closing_char(c) {
            if c != get_closing_character(starting_char) {
                return SyntaxCheckerResult::Corrupt(c);
            }

            return SyntaxCheckerResult::Ok(i+1);

        } else if is_opening_char(c) {
            match check_closing_char(&slice[i+1..slice.len()], c) {
                SyntaxCheckerResult::Ok(length) => target_i += length,
                result => return result
            };
        } else {
            panic!("invalid character detected: {}", c);
        }

        target_i += 1;
    }

    SyntaxCheckerResult::Incomplete
}

fn is_opening_char(c: char) -> bool {
    return c == '(' || c == '{' || c == '[' || c == '<';
}

fn is_closing_char(c: char) -> bool {
    return c == ')' || c == '}' || c == ']' || c == '>';
}

fn get_closing_character(starting_char: char) -> char {
    match starting_char {
        '(' => ')',
        '{' => '}',
        '[' => ']',
        '<' => '>',
        _ => panic!("invalid starting character {}", starting_char)
    }
}

#[test]
fn basic_test() {
    let test = "{}";
    assert_eq!(check(test), SyntaxCheckerResult::Ok(2));
}

#[test]
fn basic_nested_test() {
    let test = "{()}";
    assert_eq!(check(test), SyntaxCheckerResult::Ok(4));
}

#[test]
fn basic_multiple_chunk_test() {
    let test = "{}[]()<>";
    assert_eq!(check(test), SyntaxCheckerResult::Ok(8));
}

#[test]
fn complex_test() {
    let test = "{{{}<>()}{()}}";
    assert_eq!(check(test), SyntaxCheckerResult::Ok(14));
}

#[test]
fn basic_corrupt_test() {
    let test = "{>";
    assert_eq!(check(test), SyntaxCheckerResult::Corrupt('>'));
}

#[test]
fn nested_corrupt_test() {
    let test = "{([<]])}";
    assert_eq!(check(test), SyntaxCheckerResult::Corrupt(']'));
}

#[test]
fn multiple_chunk_corrupt_test() {
    let test = "{}()[]<]";
    assert_eq!(check(test), SyntaxCheckerResult::Corrupt(']'));
}

#[test]
fn incomplete_test() {
    let test1 = "[()";
    let test2 = "[[";
    let test3 = "[({})[";
    assert_eq!(check(test1), SyntaxCheckerResult::Incomplete);
    assert_eq!(check(test2), SyntaxCheckerResult::Incomplete);
    assert_eq!(check(test3), SyntaxCheckerResult::Incomplete);
}
