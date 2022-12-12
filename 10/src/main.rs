use std::{fs, ops::RemAssign};

fn main() {
    // Get Input
    let file_path = "./src/input.txt";
    let contents = fs::read_to_string(file_path).expect("Something went wrong reading the file");
    let binding = contents.replace("\r", "");
    let contents = binding.split('\n').collect::<Vec<&str>>();

    // loop through the input adn track iteratrion and x value
    let mut x_value = 1;
    let mut iteration = 0;

    let mut signal_strengths = vec![vec![0; 40]; 6];

    for line in contents {
        if line.contains("noop") {
            iteration += 1;
            check_iteration(iteration, x_value)
        }
        if line.contains("addx") {
            iteration += 1;
            check_iteration(iteration, x_value);
            iteration += 1;
            check_iteration(iteration, x_value);
            

            let split_line = line.split(' ').last().unwrap();
            let x_value_to_add : i32 = split_line.parse().unwrap();
            x_value += x_value_to_add;
        }
    }


    
}

fn check_iteration(iteration: i32, x_value: i32) {
    //check if cycle is between 1 and 40
    if 1 <= iteration && iteration <= 40 {
        println!("Iteration: {}", iteration);
        
        let signal_strength = x_value * iteration;

        println!("X Value: {}", x_value);
        println!("Signal Strength: {}", signal_strength);
        println!("");
    }

    return 0;
}j

