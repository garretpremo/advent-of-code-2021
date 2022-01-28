use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Clone)]
pub struct Position {
    pub row: usize,
    pub col: usize
}

pub struct Bingo {
    board: Vec<Vec<u32>>,
    number_position_map: HashMap<u32, Position>,
    marked_numbers: HashSet<u32>,
    won: bool
}

impl Bingo {
    pub fn new(board: Vec<Vec<u32>>) -> Bingo {

        let number_position_map: HashMap<u32, Position> = board.iter()
            .enumerate()
            .flat_map(|(row, row_data)| {
                row_data.iter()
                    .copied()
                    .enumerate()
                    .map(move |(col, value)| (value, Position { row, col }))
            })
            .collect();

        let marked_numbers = HashSet::new();

        Bingo { board, number_position_map, marked_numbers, won: false }
    }

    pub fn has_won(&self) -> bool {
        self.won
    }

    pub fn mark_number(&mut self, number: u32) {
        if !self.contains_number(number) {
            return
        }

        self.marked_numbers.insert(number);

        if self.get_marked_number_count() > 4 {
            self.check_bingo(number);
        }
    }

    pub fn sum_of_unmarked_numbers(&self) -> u32 {
        self.board.iter()
            .flat_map(|row| row.iter())
            .copied()
            .filter(|number| !self.is_number_marked(number))
            .sum()
    }

    fn check_bingo(&mut self, last_inserted_number: u32) {
        let Position { row, col } = self.get_position(last_inserted_number).unwrap();

        if self.is_row_winning(row) || self.is_col_winning(col) {
            self.won = true;
        }
    }

    fn is_row_winning(&self, row: usize) -> bool {
        self.board[row][0..5].iter().all(|number| self.is_number_marked(number))
    }

    fn is_col_winning(&self, col: usize) -> bool {
        self.board[0..5].iter().all(|row| self.is_number_marked(&row[col]))
    }

    fn get_marked_number_count(&self) -> usize {
        self.marked_numbers.len()
    }

    fn get_position(&self, number: u32) -> Option<Position> {
        if self.contains_number(number) {
            Some(self.number_position_map.get(&number).unwrap().clone())
        } else {
            None
        }
    }

    fn contains_number(&self, number: u32) -> bool {
        self.number_position_map.contains_key(&number)
    }

    fn is_number_marked(&self, number: &u32) -> bool {
        match self.marked_numbers.get(number) {
            Some(_) => true,
            None => false
        }
    }
}

#[test]
fn test_sample_input() {
    let test_board = vec![
        vec![14, 21, 17, 24,  4 ],
        vec![10, 16, 15,  9, 19 ],
        vec![18,  8, 23, 26, 20 ],
        vec![22, 11, 13,  6,  5 ],
        vec![2,  0, 12,  3,  7 ]
    ];

    let mut bingo = Bingo::new(test_board);

    let position = bingo.get_position(9).unwrap();

    assert_eq!(position.row, 1);
    assert_eq!(position.col, 3);

    let no_position = bingo.get_position(999);

    assert!(no_position.is_none());

    bingo.mark_number(7);
    bingo.mark_number(4);
    bingo.mark_number(9);
    bingo.mark_number(5);
    bingo.mark_number(11);
    bingo.mark_number(17);
    bingo.mark_number(23);
    bingo.mark_number(2);
    bingo.mark_number(0);
    bingo.mark_number(14);
    bingo.mark_number(21);
    bingo.mark_number(24);

    assert_eq!(bingo.has_won(), true);
    assert_eq!(bingo.sum_of_unmarked_numbers(), 188);
}

#[test]
fn test_col_win() {
    let test_board = vec![
        vec![14, 21, 17, 24,  4 ],
        vec![10, 16, 15,  9, 19 ],
        vec![18,  8, 23, 26, 20 ],
        vec![22, 11, 13,  6,  5 ],
        vec![2,  0, 12,  3,  7 ]
    ];

    let mut bingo = Bingo::new(test_board);

    bingo.mark_number(14);
    bingo.mark_number(10);
    bingo.mark_number(18);
    bingo.mark_number(22);
    bingo.mark_number(2);

    assert_eq!(bingo.has_won(), true);
}
