use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use grid::*;
use std::num;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let mut input : Vec<usize> = vec![0;0];
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for (_, line) in lines.enumerate() {
            if let Ok(input_line) = line {
                for val in input_line.split(",") {
                    input.push(val.parse::<usize>().unwrap());
                }
            }
        }
    }
    calculate_fuel(&input);
}

fn calculate_fuel(input: &Vec<usize>) {
    let mut min_fuel_cost : i64 = 0;
    let mut current_fuel_cost : i64 = 0;
    for i in 100..2000 {
        for j in input.iter() {
            let total_moves = ((i as i64) - (*j as i64)).abs();

            current_fuel_cost += total_moves * (total_moves + 1) / 2
        }
        if current_fuel_cost < min_fuel_cost || i == 100 {
            min_fuel_cost = current_fuel_cost;
        }
        current_fuel_cost = 0;
    }
    println!("Min fuel cost is {}", min_fuel_cost)
}

fn gen_range(first: usize, second: usize) -> std::ops::RangeInclusive<usize> {
    if first > second {
        return std::ops::RangeInclusive::new(second, first);
    } else {
        return std::ops::RangeInclusive::new(first, second);
    }
}

struct Point {
    x: usize,
    y: usize
}

fn read_point(input: &str) -> Point {
    let two_sides : Vec<&str>= input.split(",").collect();
    return Point{x: two_sides[0].parse::<usize>().unwrap(), y: two_sides[1].parse::<usize>().unwrap()}
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
