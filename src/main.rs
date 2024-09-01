use std::fs::File;
use std::io::{self, BufRead};

#[derive(Clone)]
struct SudokuBoard {
    board: [[u8; 9]; 9],
}

impl SudokuBoard {
    fn new() -> SudokuBoard {
        SudokuBoard { board: [[0; 9]; 9] }
    }

    fn from_lines(lines: impl Iterator<Item = String>) -> SudokuBoard {
        let mut board = SudokuBoard::new();

        for (i, line) in lines.enumerate() {
            for (j, c) in line.chars().enumerate() {
                let n = c.to_digit(10).unwrap() as u8;
                board.board[i][j] = n;
            }
        }

        board
    }

    fn get_row(&self, row: usize) -> [u8; 9] {
        self.board[row]
    }

    fn get_col(&self, col: usize) -> [u8; 9] {
        let mut column = [0; 9];
        for i in 0..9 {
            column[i] = self.board[i][col];
        }
        column
    }

    fn get_square(&self, row: usize, col: usize) -> [u8; 9] {
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

    fn check(&self) -> bool {
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

    fn solve(&self) -> Option<SudokuBoard> {
        // solve the board

        let mut board_stack = vec![self.clone()];

        while !board_stack.is_empty() {
            let board = board_stack.pop().unwrap();

            let first_non_zero = (0..9)
                .map(|i| (0..9).map(move |j| (i, j)))
                .flatten()
                .find(|(i, j)| board.board[*i][*j] == 0);

            if first_non_zero.is_none() {
                return Some(board);
            }

            let (i, j) = first_non_zero.unwrap();

            let mut possible_values = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
            let row = board.get_row(i);
            let col = board.get_col(j);
            let square = board.get_square(i, j);
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
                new_board.board[i][j] = value;
                board_stack.push(new_board);
            }
        }

        return board_stack.pop();
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

fn main() {
    let sudoku_file = std::env::args().nth(1).expect("missing file name");
    let file = File::open(sudoku_file);
    let file = match file {
        Ok(file) => file,
        Err(error) => {
            panic!("Problem opening the file: {:?}", error)
        }
    };

    let lines = io::BufReader::new(file).lines();

    let mut sudoku_buffer: Vec<String> = vec![];

    let mut board_num = 0;

    for line in lines.chain(vec![Ok("TERMINATOR".to_string())]) {
        let line = line.unwrap();

        if line.starts_with("Grid") || line == "TERMINATOR" {
            if sudoku_buffer.len() > 0 {
                let board = SudokuBoard::from_lines(sudoku_buffer.clone().into_iter());
                let solved_board = board.solve();

                match solved_board {
                    Some(board) => {
                        println!("solved board {:} {:}", board_num, board.check());
                    }
                    None => {
                        println!("No solution found");
                    }
                }

                board_num += 1;

                sudoku_buffer.clear();
            }
        } else {
            sudoku_buffer.push(line);
        }
    }
}
