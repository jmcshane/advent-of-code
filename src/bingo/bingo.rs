#[derive(Debug)]
pub struct Board {
    pub grid: Vec<String>,
    pub matched_indexes: Vec<i32>,
    add_index: usize,
    pub already_complete: bool
}

impl Board {
    pub fn new() -> Board {
        return Board{grid: vec!["".to_string(); 25], matched_indexes: vec![0; 25], add_index: 0, already_complete: false};
    }

    pub fn add_line(&mut self, text: String) { 
        let split = text.split(" ");
        for input_value in split {
            if input_value == "" || input_value == "\n" {
                continue
            }
            self.grid[self.add_index] = input_value.to_string();
            self.add_index = self.add_index + 1;
        }
    }

    pub fn call_number(&mut self, number: String) -> bool {
        if self.already_complete {
            return false;
        }
        let possible_index = self.grid.iter().position(|r| r.to_string() == number);
        match possible_index {
            Some(index) => {
                self.matched_indexes[index] = 1;
                let result = check_win_condition(&self.matched_indexes);
                if result {
                    self.already_complete = true
                }
                return result
            }
            _ => return false,
        };
    }

    pub fn print_win_output(&self, called_number: String) {
        let mut sum = 0;
        for (i, called) in self.matched_indexes.iter().enumerate() {
            if *called == 0 {
                sum += self.grid[i].parse::<i32>().unwrap();
            }
        }
        println!("Score is {}", sum * called_number.parse::<i32>().unwrap());
    }
}

fn check_win_condition(indexes: &Vec<i32>) -> bool {
    if indexes.len() < 5 {
        return false;
    }
    for i in 0..5 {
        let mut row_cond = true;
        let mut column_cond = true;
        for j in 0..5 {
            if indexes[j + 5*i] == 0 {
                row_cond = false;
            }
            if indexes[i + 5*j] == 0 {
                column_cond = false;
            }
        }
        if row_cond || column_cond {
            return true;
        }
    }
    return false;
}