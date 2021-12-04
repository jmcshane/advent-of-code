use std::env;
mod binary;

use binary::BinaryCounter;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let len = args[2].parse::<usize>().unwrap();
    let mut diag_counter = BinaryCounter::new(len);
    for i in 0..len {
        diag_counter.set_current_index(i);
        if let Ok(lines) = read_lines(filename) {
            // Consumes the iterator, returns an (Optional) String
            for line in lines {
                if let Ok(reading) = line {
                    diag_counter.reading_update(reading, i);
                }
            }
        }
        diag_counter.calculate_patterns()
    }
    println!("Total life support rating is {}", diag_counter.life_support_rating());
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
