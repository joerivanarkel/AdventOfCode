use std::fs;

fn main() {
    // Get Input
    let file_path = "./src/input.txt";
    let contents = fs::read_to_string(file_path).expect("Something went wrong reading the file");
    let contents = contents.split('\n');

    let mut iteration = 1;
    let mut x_value_list: Vec<i32> = Vec::new();
    x_value_list.push(1);

    for line in contents {
        if line.contains("noop") {
            iteration += 1;
            check_iteration(iteration, &mut x_value_list);
        }
        if line.contains("addx") {
            iteration += 2;
            check_iteration(iteration, &mut x_value_list);

            let split_line = line.split(' ').last().unwrap();
            let x_value_to_add = split_line.parse::<i32>().unwrap();
            x_value_list.push(x_value_to_add);
        }
    }
}

fn check_iteration(iteration: i32, x_value_list: &mut Vec<i32>) {
    match iteration {
        20 | 60 | 100 | 140 | 180 | 220 => {
            println!("Iteration: {}", iteration);

            let x_value = x_value_list.iter().sum::<i32>();
            let signal_strength = x_value * iteration;

            println!("X Value: {}", x_value);
            println!("Signal Strength: {}", signal_strength);
            println!("");
        }
        i32::MIN..=19_i32 | 21_i32..=i32::MAX => {}
    }
}
