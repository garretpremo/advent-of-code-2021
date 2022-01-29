use crate::IO;
use std::collections::HashSet;

pub struct Decoder {
    top: char,
    top_left: char,
    top_right: char,
    middle: char,
    bottom_left: char,
    bottom_right: char,
    bottom: char,
}

impl Decoder {
    pub fn new() -> Decoder {
        Decoder { top: ' ', top_left: ' ', top_right: ' ', middle: ' ', bottom_left: ' ', bottom_right: ' ', bottom: ' ', }
    }

    /// input: a set of input/output digits, represented by a randomly assorted list of characters (each character representing a side of a digit).
    ///
    ///     e.g. abc -> 7 (since a digital representation of 7 is three sides, top, top-right and bottom-right
    ///          ac  -> 1 (                            "" of 1 is two sides, top-right and bottom-right
    ///
    ///     since the input contains all 10 digits, we can decipher which character
    ///     corresponds to each digits side by process of elimination
    ///
    ///     once we know what all digits look like, we can calculate the output
    ///
    pub fn decode(&mut self, io: &IO) -> u32 {

        let input_digits: Vec<HashSet<char>> = io.input.iter()
            .map(|digit| HashSet::from_iter(digit.chars()))
            .collect();

        let output_digits: Vec<HashSet<char>> = io.output.iter()
            .map(|digit| HashSet::from_iter(digit.chars()))
            .collect();

        let mut zero = &Default::default();
        let mut one = &Default::default();
        let mut two = &Default::default();
        let mut three = &Default::default();
        let mut four = &Default::default();
        let mut five = &Default::default();
        let mut six = &Default::default();
        let mut seven = &Default::default();
        let mut eight = &Default::default();
        let mut nine = &Default::default();

        // decode one, seven, four, and eight
        for digit in input_digits.iter() {
            match digit.len() {
                2 => one = digit,
                3 => seven = digit,
                4 => four = digit,
                7 => eight = digit,
                _ => {}
            }
        }

        // extrapolate top digit segment
        self.top = seven.difference(one).copied().collect::<Vec<char>>()[0];

        // decode three
        for digit_set in input_digits.iter() {
            if digit_set.len() != 5 {
                continue;
            }

            let intersection_4: HashSet<char> = four.intersection(&digit_set).copied().collect();
            let intersection_7: HashSet<char> = seven.intersection(&digit_set).copied().collect();

            if intersection_4.len() == 3 && intersection_7.len() == 3 {
                three = digit_set;

                // extrapolate top-left and middle segments
                self.top_left = four.difference(&intersection_4).copied().collect::<Vec<char>>()[0];
                self.middle = intersection_4.difference(&one).copied().collect::<Vec<char>>()[0];
            }
        }

        // decode zero
        for digit_set in input_digits.iter() {
            if digit_set.len() != 6 || digit_set.contains(&self.middle) {
                continue;
            }

            zero = digit_set;
        }

        // decode nine
        for digit_set in input_digits.iter() {
            if digit_set.len() == eight.len() {
                continue;
            }

            let intersection_7: HashSet<char> = seven.intersection(&digit_set).copied().collect();

            if intersection_7.len() == 3 && digit_set.contains(&self.top_left) && digit_set.contains(&self.middle) {
                nine = digit_set;
                let all_except_bottom: HashSet<char> = four.union(seven).copied().collect();

                // extrapolate bottom segment
                self.bottom = nine.difference(&all_except_bottom).copied().collect::<Vec<char>>()[0];
            }
        }

        // decode 5 & 6
        for digit_set in input_digits.iter() {
            if digit_set.eq(eight) || digit_set.eq(nine) {
                continue;
            }

            if !digit_set.contains(&self.top) || !digit_set.contains(&self.top_left) || !digit_set.contains(&self.middle) {
                continue;
            }

            match digit_set.len() {
                6 => six = digit_set,
                _ => five = digit_set
            }
        }

        // extrapolate remaining digit segments
        self.bottom_left = six.difference(five).copied().collect::<Vec<char>>()[0];
        self.top_right = nine.difference(five).copied().collect::<Vec<char>>()[0];
        self.bottom_right = one.difference(&HashSet::from([self.top_right])).copied().collect::<Vec<char>>()[0];

        let two_set = HashSet::from([self.top, self.top_right, self.middle, self.bottom_left, self.bottom]);
        // decode two
        for digit_set in input_digits.iter() {
            if digit_set.eq(&two_set) {
                two = digit_set;
            }
        }

        let mut output = 0u32;
        for (i, output_digit) in output_digits.iter().enumerate() {
            if output_digit.eq(zero) { continue; }

            let mut digit_value = 0;
            if output_digit.eq(one )    { digit_value = 1; }
            if output_digit.eq(two )    { digit_value = 2; }
            if output_digit.eq(three )  { digit_value = 3; }
            if output_digit.eq(four )   { digit_value = 4; }
            if output_digit.eq(five )   { digit_value = 5; }
            if output_digit.eq(six )    { digit_value = 6; }
            if output_digit.eq(seven )  { digit_value = 7; }
            if output_digit.eq(eight )  { digit_value = 8; }
            if output_digit.eq(nine )   { digit_value = 9; }

            output += digit_value * 10u32.pow(((output_digits.len() - 1) - i) as u32);
        }

        output
    }
}
