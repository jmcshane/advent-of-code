use std::env;
use prefix_sum::summable::Summable;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let mut position = Movement{x:0,y:0,aim:0};
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(direction) = line {
                position.add_assign(apply_direction(direction))
            }
        }
    }
    println!("Moved horizontally {} and vertically {}", position.x, position.y)
}

struct Movement {
    x: i32,
    y: i32,
    aim: i32,
}

impl Summable for Movement {
    fn zero() -> Self {
        return Movement{x: 0, y:0, aim: 0}
    }
    fn add_assign_ref(&mut self, rhs: &Self) {
    }
    fn sub_assign_ref(&mut self, rhs: &Self) {
    }
    //only use add_assign for now
    fn add_assign(&mut self, rhs: Self) {
        self.x.add_assign(rhs.x);
        self.y.add_assign(rhs.x * self.aim);
        self.aim.add_assign(rhs.aim);
    }
    fn sub_assign(&mut self, rhs: Self) {
    }
}

fn apply_direction(input: String) -> Movement {
    let split : Vec<&str> = input.split(" ").collect();
    let distance = split[1].parse::<i32>().unwrap();
    match (*split[0]).to_lowercase().as_str() {
        "up" => return Movement{x:0, y:0, aim: -1 * distance},
        "down" => return Movement{x: 0, y:0, aim: distance},
        "forward" => return Movement{x: distance, y: 0, aim: 0},
        _ => println!("Invalid instruction"),
    }
    return Movement{x:0,y:0, aim:0};
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
