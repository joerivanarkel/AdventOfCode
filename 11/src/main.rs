use std::fs;

fn main() {
    let lines = get_input();
    let mut i = 0;

    while lines.len() > 0 {
        let result = create_monkey(lines, i as i32);

        i = result.0 as usize;
        let monkey = result.1;

        println!("Monkey: {:?}", monkey);
    }
}

fn get_input() -> Vec<String> {
    let file_path = "./src/input.txt";
    let contents = fs::read_to_string(file_path).expect("Something went wrong reading the file");
    let binding = contents.replace("\r", "");
    let contents = binding.split('\n').collect::<Vec<&str>>();

    let mut lines = Vec::new();

    for line in contents {
        let formatted_line = line.replace("\r", "").replace("\n", "");
        lines.push(formatted_line);
    }

    return lines;
}

fn create_monkey(contents: Vec<String>, iteration: i32) -> (i32, Monkey) {
    let monkey_id = contents[iteration as usize]
        .replace("Monkey ", "")
        .parse::<i32>()
        .unwrap();
    let starting_items = contents[iteration as usize + 1]
        .replace("Starting Items: ", "")
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect::<Vec<i32>>();
    let operation = contents[iteration as usize + 2].replace("Operation: ", "");
    let test_case = contents[iteration as usize + 3]
        .replace("Test Case: ", "")
        .parse::<i32>()
        .unwrap();
    let true_case = contents[iteration as usize + 4]
        .replace("True Case: ", "")
        .parse::<i32>()
        .unwrap();
    let false_case = contents[iteration as usize + 5]
        .replace("False Case: ", "")
        .parse::<i32>()
        .unwrap();

    let monkey = Monkey {
        id: monkey_id,
        starting_items: starting_items,
        operation: operation,
        test_case: test_case,
        true_case: true_case,
        false_case: false_case,
    };

    println!("Monkey: {:?}", monkey);

    return (iteration, monkey);
}

#[derive(Debug)]
struct Monkey {
    id: i32,
    starting_items: Vec<i32>,
    operation: String,
    test_case: i32,
    true_case: i32,
    false_case: i32,
}
