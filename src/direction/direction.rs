
pub mod moving {
    pub struct Movement {
        pub x: i32,
        pub y: i32,
        aim: i32,
    }
    impl Movement {
        pub fn new() -> Movement {
            Movement{x: 0, y: 0, aim: 0}
        }
    }
    use prefix_sum::summable::Summable;

    impl Summable for Movement {
        fn zero() -> Self {
            return Movement::new();
        }
        fn add_assign_ref(&mut self, _rhs: &Self) {
        }
        fn sub_assign_ref(&mut self, _rhs: &Self) {
        }
        //only use add_assign for now
        fn add_assign(&mut self, rhs: Self) {
            self.x.add_assign(rhs.x);
            self.y.add_assign(rhs.x * self.aim);
            self.aim.add_assign(rhs.aim);
        }
        fn sub_assign(&mut self, _rhs: Self) {
        }
    }

    pub fn print_output(input: Movement) {
        println!("Horizontal movement {}, Vertical movement {}", input.x, input.y)
    }

    pub fn apply_direction(input: String) -> Movement {
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
}