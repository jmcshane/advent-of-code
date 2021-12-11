struct Point {
    x: usize,
    y: usize
}

fn read_point(input: &str) -> Point {
    let two_sides : Vec<&str>= input.split(",").collect();
    return Point{x: two_sides[0].parse::<usize>().unwrap(), y: two_sides[1].parse::<usize>().unwrap()}
}