use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use grid::*;
use std::num;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let mut counter: i32 = 0;
    let mut input : Vec<&str> = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for (_, line) in lines.enumerate() {
            if let Ok(input_line) = line {
                counter += calculate_unique(input_line.split("|").collect::<Vec<&str>>()[1])
            }
        }
    }
    println!("There are {} unique values", counter);
}

static UNIQUE_LENGTHS: &'static [usize] = &[2, 3, 4, 7];

fn calculate_unique(input: &str) -> i32 {
    let mut increment = 0;
    for val in input.split(" ") {
        if UNIQUE_LENGTHS.contains(&val.chars().count()) {
            increment = increment + 1;
        }
    }
    return increment;
}


fn gen_range(first: usize, second: usize) -> std::ops::RangeInclusive<usize> {
    if first > second {
        return std::ops::RangeInclusive::new(second, first);
    } else {
        return std::ops::RangeInclusive::new(first, second);
    }
}



// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
