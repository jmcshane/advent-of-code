use prefix_sum::summable::Summable;

pub struct BinaryCounter {
    zeros: Vec<i32>,
    ones: Vec<i32>,
    len: usize
}

impl BinaryCounter {
    pub fn new(len: usize) -> BinaryCounter {
        return BinaryCounter{zeros: vec![0; len], ones: vec![0; len], len: len};
    }

    pub fn add_reading(mut self: Self, input: String) -> BinaryCounter {
        for (i,c) in input.chars().enumerate() { 
            match c {
                '0' => self.zeros[i] = self.zeros[i] + 1,
                '1' => self.ones[i] = self.ones[i] + 1,
                _ => println!("unexpected character")
            }
        }
        return self;
    }

    pub fn power_consumption(self: Self) -> i32 {
        let mut gamma = 0;
        let mut epsilon = 0;
        let base: i32 = 2;
        for i in 0..self.len {
            match self.zeros[i] > self.ones[i] {
                true =>  epsilon = epsilon + base.pow((self.len - i - 1).try_into().unwrap()),
                false => gamma = gamma + base.pow((self.len - i - 1).try_into().unwrap())
            }
        }
        return gamma * epsilon;
    }
}
