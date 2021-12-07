use std::env;
mod bingo;

use bingo::Board;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let mut boards: Vec<Board> = Vec::new();
    let mut input: String = String::from("");
    let mut board_index = 0;
    let mut current_board = bingo::Board::new();
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for (i, line) in lines.enumerate() {
            if let Ok(input_line) = line {
                if i == 0 {
                    input.push_str(&input_line);
                } else if i % 6 == 1 && i != 1 {
                    board_index = board_index + 1;
                } else if i % 6 == 2 {
                    boards.push(current_board);
                    current_board = bingo::Board::new();
                }
                if i > 1 && i % 6 != 1 {
                    boards[board_index].add_line(input_line)
                }
            }
        }
    }

    let mut completed_boards = 0;
    for called_number in input.split(",") {
        let mut complete = false;
        for i in 0..board_index {
            let has_won = boards[i].call_number(called_number.to_string());
            if has_won {
                completed_boards = completed_boards + 1;
                if completed_boards == board_index {
                    boards[i].print_win_output(called_number.to_string());
                    complete = true
                }
            }
        }
        if complete {
            break;
        }
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
