use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use grid::*;
use std::num;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let mut input : Vec<i32> = vec![0;7];
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for (_, line) in lines.enumerate() {
            if let Ok(input_line) = line {
                for val in input_line.split(",") {
                    input[val.parse::<usize>().unwrap()]+= 1;
                }
            }
        }
    }
    println!("{:?}", input);
    // 0-> 1421
    // 1-> 1401
    // 2-> 1191
    // 3-> 1154
    // 4-> 1034
    // 5-> 950
    // 6 -> 905
    let mut fish_counter = vec![0];
    for i in 0..80 {
        let mut new_fish = 0;
        for regen in fish_counter.iter_mut() {
            if *regen == 0 {
                new_fish+= 1;
                *regen = 6;
            } else {
                *regen = *regen - 1;
            }
        }
        for _ in 0..new_fish {
            fish_counter.push(8);
        }
        if i > 72 {
            input[79-i] = input[79-i] * i32::try_from(fish_counter.len()).unwrap();
            println!("there are {} fish on day {}", fish_counter.len(), i);
        }
    }
    println!("{:?}", input);
    println!("{}", input.iter().sum::<i32>());
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
