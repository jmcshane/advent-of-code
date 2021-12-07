use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use grid::*;
use std::num;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let max = args[2].parse::<usize>().unwrap();
    let mut grid = Grid::from_vec(vec![0;max*max], max);
    println!("Grid size is {:?}", grid.size());
    for i in std::ops::RangeInclusive::new(122, 120) {
        println!("I is {}", i);
    }
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for (_, line) in lines.enumerate() {
            if let Ok(input_line) = line {
                let two_sides : Vec<&str>= input_line.split("->").collect();
                let left = read_point(two_sides[0].trim());
                let right = read_point(two_sides[1].trim());
                if left.x == right.x {
                    for i in gen_range(left.y, right.y) {
                        grid[right.x][i] = grid[right.x][i] + 1;
                    }
                } else if left.y == right.y {
                    for i in gen_range(left.x, right.x) {
                        grid[i][right.y] = grid[i][right.y] + 1;
                    }
                } else if (i32::try_from(left.x).unwrap() - i32::try_from(right.x).unwrap()).abs() == (i32::try_from(left.y).unwrap() - i32::try_from(right.y).unwrap()).abs() {
                    let diff = (i32::try_from(left.x).unwrap() - i32::try_from(right.x).unwrap()).abs();
                    for i in 0..=diff {
                        let index_add = usize::try_from(i).unwrap();
                        let x_val = match left.x > right.x {
                            true => left.x - index_add,
                            false => left.x + index_add,
                        };
                        let y_val = match left.y > right.y {
                            true => left.y - index_add,
                            false => left.y + index_add,
                        };
                        grid[x_val][y_val] = grid[x_val][y_val] + 1;
                    }

                }
            }
        }
    }
    let mut count = 0;
    for i in 0..max {
        for j in 0..max {
            if grid[i][j] > 1 {
                count = count + 1;
            }
        }
    }
    println!("Avoid {} spaces", count);
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
