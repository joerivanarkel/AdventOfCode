use std::{fs, ops::RemAssign};

fn main() {
    // Get Input
    let file_path = "./src/input.txt";
    let contents = fs::read_to_string(file_path).expect("Something went wrong reading the file");
    let binding = contents.replace("\r", "");
    let contents = binding.split('\n').collect::<Vec<&str>>();

    // loop through the input adn track iteratrion and x value
    let mut x_value = 1;
    let mut result = 0;
    let mut iteration = 0;

    for line in contents {
        if line.contains("noop") {
            iteration += 1;
            result += check_iteration(iteration, x_value)
        }
        if line.contains("addx") {
            iteration += 1;
            result += check_iteration(iteration, x_value);
            iteration += 1;
            result += check_iteration(iteration, x_value);
            

            let split_line = line.split(' ').last().unwrap();
            let x_value_to_add : i32 = split_line.parse().unwrap();
            x_value += x_value_to_add;
        }
    }

    println!("Result: {}", result);

    
}

fn check_iteration(iteration: i32, x_value: i32) -> i32 {
    match iteration {
        20 | 60 | 100 | 140 | 180 | 220 => {
            println!("Iteration: {}", iteration);
            
            let signal_strength = x_value * iteration;

            println!("X Value: {}", x_value);
            println!("Signal Strength: {}", signal_strength);
            println!("");

            return signal_strength;
        }
        i32::MIN..=19_i32 | 21_i32..=i32::MAX => {}
    }

    return 0;
}