pub mod add_sand;
pub mod draw_coords;

use core::time::Duration;
use std::fs;
use std::thread::sleep;

fn main() {
    println!("Hello, world!");
    let input = get_input();

    let mut board = create_board();

    let board = draw_coords::main(input, &mut board);
    let mut sand_number = 0;
    while check_bottom_row(board) {
        let board = add_sand::main(board);
        sand_number += 1;
        print_board(board);
        sleep(Duration::from_millis(100));
    }
    print_board(board);
    println!("Sand number: {}", sand_number);
}

fn check_bottom_row(board: &Vec<Vec<char>>) -> bool {
    let mut bottom_row = true;
    for col in &board[board.len() - 1] {
        if *col != '░' {
            bottom_row = false;
            break;
        }
    }
    return bottom_row;
}

fn create_board() -> Vec<Vec<char>> {
    // Min X: 460
    // Max X: 521
    // Min Y: 13
    // Max Y: 166

    let board = vec![vec!['░'; 521 - 460 + 1]; 166 - 13 + 1];
    // for row in &mut board {
    //     row.insert(39, '▒');
    // }
    print_board(&board);

    return board;
}

pub fn print_board(board: &Vec<Vec<char>>) {
    let mut line = 000;

    for row in board {
        print!("{:0>3}: ", line);
        for col in row {
            print!("{}", col);
        }
        println!();
        line += 1;
    }
    println!();
}

fn get_input() -> Vec<Vec<[i32; 2]>> {
    let file_path = "./src/input.txt";
    let binding = fs::read_to_string(file_path).expect("Something went wrong reading the file");
    let contents = binding.split('\n').collect::<Vec<&str>>();

    let lines = parse_input(contents);
    let differences = calculate_differences(lines);

    return differences;
}

fn parse_input(contents: Vec<&str>) -> Vec<Vec<[i32; 2]>> {
    let mut lines = Vec::new();
    for line in contents {
        let mut coordinates = Vec::new();

        let coords_line = line.split(" -> ").collect::<Vec<&str>>();
        for coord in coords_line {
            let coords = coord.split(",").collect::<Vec<&str>>();

            let x = coords[0].parse::<i32>().unwrap();
            let y = coords[1].parse::<i32>().unwrap();

            let coordinate = [x, y];
            coordinates.push(coordinate);
        }

        lines.push(coordinates);
    }

    return lines;
}

fn calculate_differences(lines: Vec<Vec<[i32; 2]>>) -> Vec<Vec<[i32; 2]>> {
    let mut differences = Vec::new();

    let min_x = lines
        .iter()
        .map(|line| line.iter().map(|coord| coord[0]).min().unwrap())
        .min()
        .unwrap();
    let max_x = lines
        .iter()
        .map(|line| line.iter().map(|coord| coord[0]).max().unwrap())
        .max()
        .unwrap();
    let min_y = lines
        .iter()
        .map(|line| line.iter().map(|coord| coord[1]).min().unwrap())
        .min()
        .unwrap();
    let max_y = lines
        .iter()
        .map(|line| line.iter().map(|coord| coord[1]).max().unwrap())
        .max()
        .unwrap();

    println!("Min X: {}", min_x);
    println!("Max X: {}", max_x);
    println!("Min Y: {}", min_y);
    println!("Max Y: {}", max_y);

    for line in lines {
        let mut difference = Vec::new();
        for i in 0..line.len() - 1 {
            if i == 0 {
                difference.push(line[0]);
                continue;
            }
            let x = line[i][0] - line[i - 1][0];
            let y = line[i][1] - line[i - 1][1];
            let coord = [x, y];
            difference.push(coord);
        }
        differences.push(difference);
    }
    println!("{:?}", differences);

    return differences;
}
