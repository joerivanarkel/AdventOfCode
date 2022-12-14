use crate::print_board;

pub fn main(lines: Vec<Vec<[i32; 2]>>, board: &mut Vec<Vec<char>>) -> &mut Vec<Vec<char>> {
    let mut j = 0;

    for i in lines {
        draw_line(i, board);
        println!("Line {}:", j);
        j += 1;
    }
    print_board(&board);
    return board;
}

fn draw_line(line: Vec<[i32; 2]>, board: &mut Vec<Vec<char>>) -> &mut Vec<Vec<char>> {
    let mut start_x = line[0][0] - 460;
    let mut start_y = line[0][1];

    for i in 1..line.len() {
        let x_diff = line[i][0];
        let y_diff = line[i][1];
        if x_diff == 0 {
            for j in 0..y_diff {
                let start_y_usize = (start_y + j) as usize;
                board[start_y_usize][start_x as usize] = '█';
            }
        } else if y_diff == 0 {
            for j in 0..x_diff {
                let start_x_usize = (start_x + j) as usize;
                board[start_y as usize][start_x_usize] = '█';
            }
        }
    }
    return board;
}
