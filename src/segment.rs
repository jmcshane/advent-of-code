static UNIQUE_LENGTHS: &'static [usize] = &[2, 3, 4, 7];
static KEYWORDS: phf::Map<&'static str, i32> = phf_map! {
    "2" => 1,
    "3" => 7,
    "4" => 4,
    "7" => 8,
};

fn calculate_value(input: Vec<&str>) -> i32 {
    let int_map = create_map(input[0]);
    let mut code = vec![0; 0];
    for value in input[1].split::<&str>(" ") {
        if value == "" {
            continue
        }
        code.push(*int_map.get(&value.chars().sorted().collect::<String>()).unwrap())
    }
    let mut result = 0;
    let base : i32 = 10;
    for (i, v) in code.iter().enumerate() {
        result += v * base.pow((3 - i).try_into().unwrap())
    }
    return result;
}

fn create_map(input: &str) -> HashMap<String, i32> {
    let mut map_values = HashMap::new();
    let mut unique_entries = HashMap::new();
    let blinking_values : Vec<&str> = input.split(" ").collect();
    loop {
        for value in blinking_values.iter() {
            let ordered_value = value.chars().sorted().collect::<String>();
            if map_values.contains_key(&ordered_value) {
                continue;
            }
            if UNIQUE_LENGTHS.contains(&value.chars().count()) {
                let get_val = &value.chars().count().to_string();
                map_values.insert(ordered_value, *KEYWORDS.get(get_val).unwrap());
                unique_entries.insert(*KEYWORDS.get(get_val).unwrap(), *value);
                continue
            } 
            let len_str = value.chars().count();
            if len_str == 5 {
                //could be 2,3,5
                match unique_entries.get(&1){
                    Some(one_value) => {
                        let mut diff_char_count = 0;
                        for one_char in one_value.chars() {
                            if !value.contains(one_char) {
                                diff_char_count += 1;
                            }
                        }
                        if diff_char_count == 0 {
                            map_values.insert(ordered_value, 3);
                            unique_entries.insert(3, *value);
                            println!("Inserted 3");
                            continue
                        }
                    },
                    None => (),
                }
                match unique_entries.get(&4) {
                    Some(four_value) => {
                        let mut diff_char_count = 0;
                        for four_char in four_value.chars() {
                            if !value.contains(four_char) {
                                diff_char_count += 1;
                            }
                        }
                        if diff_char_count == 2 {
                            map_values.insert(ordered_value, 2);
                            println!("Inserted 2");
                            continue
                        } else if unique_entries.contains_key(&3) {
                            map_values.insert(ordered_value, 5);
                            println!("inserted 5");
                            continue
                        }
                    },
                    None => (),
                }
            }
            if len_str == 6 {
                //could be 0, 6, 9
                //match 9 from 4
                match unique_entries.get(&4) {
                    Some(four_value) => {
                        let mut diff_char_count = 0;
                        for four_char in four_value.chars() {
                            if !value.contains(four_char) {
                                diff_char_count += 1;
                            }
                        }
                        if diff_char_count == 0 {
                            map_values.insert(ordered_value, 9);
                            unique_entries.insert(9, *value);
                            println!("inserted 9");
                            continue
                        }
                    },
                    None => (),
                }
                match unique_entries.get(&1) {
                    Some(one_value) => {
                        let mut diff_char_count = 0;
                        for one_char in one_value.chars() {
                            if !value.contains(one_char) {
                                diff_char_count += 1;
                            }
                        }
                        if diff_char_count == 0 && unique_entries.contains_key(&9) {
                            map_values.insert(ordered_value, 0);
                            println!("inserted 0");
                            continue
                        } else if diff_char_count == 1 {
                            map_values.insert(ordered_value, 6);
                            println!("inserted 6");
                            continue
                        }
                    },
                    None => (),
                }
            }
        }
        if map_values.keys().len() == 10 {
            break
        }
    }
    return map_values;
}
fn calculate_unique(input: &str) -> i32 {
    let mut increment = 0;
    for val in input.split(" ") {
        if UNIQUE_LENGTHS.contains(&val.chars().count()) {
            increment = increment + 1;
        }
    }
    return increment;
}
