#[derive(Clone)]
pub struct SudokuBoard {
    pub board: [[u8; 9]; 9],
}

impl SudokuBoard {
    fn new() -> SudokuBoard {
        SudokuBoard { board: [[0; 9]; 9] }
    }

    pub fn from_condensed(condensed: &str) -> SudokuBoard {
        let mut board = SudokuBoard::new();

        for (i, c) in condensed.chars().enumerate() {
            let n = {
                if c == '.' {
                    0
                } else {
                    c.to_digit(10).unwrap() as u8
                }
            };
            board.board[i / 9][i % 9] = n;
        }

        board
    }

    pub(crate) fn get_row(&self, row: usize) -> [u8; 9] {
        self.board[row]
    }

    pub(crate) fn get_col(&self, col: usize) -> [u8; 9] {
        let mut column = [0; 9];
        for i in 0..9 {
            column[i] = self.board[i][col];
        }
        column
    }

    pub(crate) fn get_square(&self, row: usize, col: usize) -> [u8; 9] {
        let mut square = [0; 9];
        let row = row - row % 3;
        let col = col - col % 3;
        let mut k = 0;
        for i in row..row + 3 {
            for j in col..col + 3 {
                square[k] = self.board[i][j];
                k += 1;
            }
        }
        square
    }

    fn all_squares(&self) -> Vec<[u8; 9]> {
        let mut squares = Vec::new();
        for i in 0..3 {
            for j in 0..3 {
                squares.push(self.get_square(3 * i, 3 * j));
            }
        }
        squares
    }

    pub fn check(&self) -> bool {
        let rows = (0..9).map(|i| self.get_row(i));
        let cols = (0..9).map(|i| self.get_col(i));
        let squares = self.all_squares().into_iter().map(|s| s);

        for (unit, elements) in rows.chain(cols).chain(squares).enumerate() {
            let mut my_val = elements.clone();
            my_val.sort();
            for i in 0..9 as u8 {
                if my_val[i as usize] != i + 1 {
                    println!(
                        "{:?} failed. it is {} {}",
                        elements,
                        match unit {
                            0..=8 => "row",
                            9..=17 => "column",
                            18..=26 => "square",
                            _ => "unknown",
                        },
                        unit % 9
                    );

                    return false;
                }
            }
        }

        return true;
    }

    fn print(&self) {
        for i in 0..9 {
            for j in 0..9 {
                if j % 3 == 0 {
                    print!("  ");
                }
                print!("{}", self.board[i][j]);
            }

            if i % 3 == 2 {
                println!();
            }
            println!();
        }
    }
}

pub type SudokuSolver = fn(&SudokuBoard) -> Option<SudokuBoard>;

pub const BACKTRACKING_SOLVER: SudokuSolver = |initial_board| -> Option<SudokuBoard> {
    // solve the board
    let mut board_stack = vec![((0, 0), initial_board.clone())];

    while !board_stack.is_empty() {
        let (old_point, board) = board_stack.pop().unwrap();

        // find the first zero from the last point
        let first_zero = (old_point.0 * 9 + old_point.1..81)
            .map(|index| (index / 9, index % 9))
            .find(|(i, j)| board.board[*i][*j] == 0);

        if first_zero.is_none() {
            return Some(board);
        }

        let point = first_zero.unwrap();

        let mut possible_values = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let row = board.get_row(point.0);
        let col = board.get_col(point.1);
        let square = board.get_square(point.0, point.1);
        for k in 0..9 {
            if row[k] != 0 {
                possible_values.retain(|&x| x != row[k]);
            }
            if col[k] != 0 {
                possible_values.retain(|&x| x != col[k]);
            }
            if square[k] != 0 {
                possible_values.retain(|&x| x != square[k]);
            }
        }

        for value in possible_values {
            let mut new_board = board.clone();
            new_board.board[point.0][point.1] = value;
            board_stack.push((point, new_board));
        }
    }

    let ret = board_stack.pop();

    if ret.is_none() {
        return None;
    }

    Some(ret.unwrap().1)
};
