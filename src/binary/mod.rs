pub struct BinaryCounter {
    zeros: Vec<i32>,
    ones: Vec<i32>,
    len: usize,
    generator_pattern: String,
    generator_zeros: i32,
    generator_ones: i32,
    scrubber_pattern: String,
    scrubber_zeros: i32,
    scrubber_ones: i32,
    index: usize,
}

fn check_pattern(pattern: &String, i: usize, len: usize) -> i32 {
    let base: i32 = 2;
    if pattern.chars().nth(i) == Some('1') {
        return base.pow((len - i - 1).try_into().unwrap());
    }
    return 0
}

impl BinaryCounter {
    pub fn new(len: usize) -> BinaryCounter {
        return BinaryCounter{zeros: vec![0; len], ones: vec![0; len], len: len, generator_pattern: String::from(""), generator_zeros: 0, generator_ones: 0, scrubber_pattern: String::from(""), scrubber_zeros: 0, scrubber_ones: 0, index: 0};
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

    pub fn set_current_index(&mut self, index: usize) {
        self.index = index
    }

    pub fn reading_update(&mut self, input: String, index: usize) {
        if input[0..index] == self.generator_pattern {
            match input.chars().nth(index).unwrap() {
                '0' => self.generator_zeros = self.generator_zeros + 1,
                '1' => self.generator_ones = self.generator_ones + 1,
                _ => println!("unexpected char")
            }
        }
        if input[0..index] == self.scrubber_pattern {
            match input.chars().nth(index).unwrap() {
                '0' => self.scrubber_zeros = self.scrubber_zeros + 1,
                '1' => self.scrubber_ones = self.scrubber_ones + 1,
                _ => println!("unexpected char")
            }
        }
    }

    pub fn calculate_patterns(&mut self) {
        if self.scrubber_ones == 0 {
            self.scrubber_pattern.push_str("0")
        } else if self.scrubber_zeros == 0 {
            self.scrubber_pattern.push_str("1")
        } else {
            match self.scrubber_ones < self.scrubber_zeros {
                true => self.scrubber_pattern.push_str("1"),
                false => self.scrubber_pattern.push_str("0")
            }
        }
        match self.generator_ones < self.generator_zeros {
            true => self.generator_pattern.push_str("0"),
            false => self.generator_pattern.push_str("1")
        }
        self.scrubber_ones = 0;
        self.scrubber_zeros = 0;
        self.generator_ones = 0;
        self.generator_zeros = 0;
    }

    pub fn life_support_rating(self: Self) -> i32 {
        let mut scrubber_rating = 0;
        let mut generator_rating = 0;

        for i in 0..self.len {
            scrubber_rating += check_pattern(&self.scrubber_pattern, i, self.len);
            generator_rating += check_pattern(&self.generator_pattern, i, self.len);
        }
        println!("Scrubber rating is {}, generator rating is {}", scrubber_rating, generator_rating);
        return scrubber_rating * generator_rating;
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
