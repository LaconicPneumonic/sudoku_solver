use std::fs::File;
use std::io::{self, BufRead};
use sudoku_solver::{SudokuBoard, BACKTRACKING_SOLVER};

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

    let mut board_num = 0;

    let solving_function = BACKTRACKING_SOLVER;

    for line in lines {
        let board = SudokuBoard::from_condensed(line.unwrap().as_str());
        let solved_board = solving_function(&board);

        match solved_board {
            Some(board) => {
                println!("solved board {:} {:}", board_num, board.check());
            }
            None => {
                println!("No solution found");
            }
        }

        board_num += 1;

        break;
    }
}
