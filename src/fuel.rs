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
