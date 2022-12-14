// ▓

pub fn main(board: &mut Vec<Vec<char>>) -> &mut Vec<Vec<char>> {
    let lowest_level = is_cell_below_empty(board, 0);
    println!("{:?}", lowest_level);

    let lowest_level_diagonal =
        is_cell_diagonal_below_empty(board, lowest_level[0], lowest_level[1]);
    println!("{:?}", lowest_level_diagonal);

    let board = add_sand(board, lowest_level_diagonal);

    return board;
}

fn is_cell_below_empty(board: &mut Vec<Vec<char>>, level: i32) -> [i32; 2] {
    let level_usize = (level + 1) as usize;

    if level == 153 {
        return [level, 39];
    }

    if board[level_usize][39] == '█' || board[level_usize][39] == '▓' {
        return [level, 39];
    } else {
        return is_cell_below_empty(board, level + 1);
    }
}

fn is_cell_diagonal_below_empty(board: &mut Vec<Vec<char>>, level: i32, axis: i32) -> [i32; 2] {
    let lowest_level = [level, axis];

    let level_usize = (level + 1) as usize;
    let axis_left_usize = (axis - 1) as usize;
    let axis_right_usize = (axis + 1) as usize;

    if level == 153 {
        return lowest_level;
    }

    if board[level_usize][axis_left_usize] == '░' {
        return is_cell_diagonal_below_empty(board, level + 1, axis - 1);
    } else if board[level_usize][axis_right_usize] == '░' {
        return is_cell_diagonal_below_empty(board, level + 1, axis + 1);
    } else {
        return lowest_level;
    }
}

fn add_sand(board: &mut Vec<Vec<char>>, location: [i32; 2]) -> &mut Vec<Vec<char>> {
    board[location[0] as usize][location[1] as usize] = '▓';

    return board;
}
