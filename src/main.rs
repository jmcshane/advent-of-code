use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use grid::*;
use std::num;
use std::collections::*;
use phf::*;
use phf::phf_map;
use itertools::*;

const RADIX: u32 = 10;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let mut grid_strings : Vec<String> = Vec::new();
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for (_, line) in lines.enumerate() {
            if let Ok(input_line) = line {
                grid_strings.push(input_line);
            }
        }
    }
    let rows = grid_strings.len();
    let cols = grid_strings[0].len();
    let mut grid : Grid<u32>= Grid::new(rows, cols);
    for (i, val) in grid_strings.iter().enumerate() {
        for (j, int_string) in val.chars().enumerate() {
            grid[i][j] = int_string.to_digit(RADIX).unwrap();
        } 
    }
    let mut risk_levels : Vec<u32> = Vec::new();
    for i in 0..rows {
        for j in 0..cols {
            if i > 0 {
                if grid[i][j] >= grid[i - 1][j] {
                    continue
                }
            }
            if j > 0 {
                if grid[i][j] >= grid[i][j - 1] {
                    continue
                }
            }
            if i < rows - 1 {
                if grid[i][j] >= grid[i + 1][j] {
                    continue
                }
            }
            if j < cols - 1 {
                if grid[i][j] >= grid[i][j+1] {
                    continue
                }
            }
            risk_levels.push(grid[i][j] + 1)
        }
    }
    println!("{:?}", risk_levels);
    let sum: u32 = risk_levels.iter().sum();
    println!("Risk levels sum {}", sum);

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
