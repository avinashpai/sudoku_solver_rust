fn main() {
    let mut puzzle: [[u8; 9]; 9] = [
        [5, 3, 0, 0, 7, 0, 0, 0, 0],
        [6, 0, 0, 1, 9, 5, 0, 0, 0],
        [0, 9, 8, 0, 0, 0, 0, 6, 0],
        [8, 0, 0, 0, 6, 0, 0, 0, 3],
        [4, 0, 0, 8, 0, 3, 0, 0, 1],
        [7, 0, 0, 0, 2, 0, 0, 0, 6],
        [0, 6, 0, 0, 0, 0, 2, 8, 0],
        [0, 0, 0, 4, 1, 9, 0, 0, 5],
        [0, 0, 0, 0, 8, 0, 0, 7, 9],
    ];

    let result: bool = solve(&mut puzzle);
    display_puzzle(&puzzle, result);
}

fn valid_subgrid(row: u8, col: u8, num: u8, puzzle: &[[u8; 9]; 9]) -> bool {
    let r = row - row % 3;
    let c = col - col % 3;

    for i in r..r + 3 {
        for j in c..c + 3 {
            if puzzle[i as usize][j as usize] == num {
                return false;
            }
        }
    }

    true
}

fn valid_col(col: u8, num: u8, puzzle: &[[u8; 9]; 9]) -> bool {
    for i in 0..9 {
        if puzzle[i][col as usize] == num {
            return false;
        }
    }

    true
}

fn valid_row(row: u8, num: u8, puzzle: &[[u8; 9]; 9]) -> bool {
    for i in 0..9 {
        if puzzle[row as usize][i] == num {
            return false;
        }
    }

    true
}

fn is_valid(row: u8, col: u8, num: u8, puzzle: &[[u8; 9]; 9]) -> bool {
    return valid_row(row, num, puzzle)
        && valid_col(col, num, puzzle)
        && valid_subgrid(row, col, num, puzzle);
}

fn solve(puzzle: &mut [[u8; 9]; 9]) -> bool {
    for row in 0..9 {
        for col in 0..9 {
            if puzzle[row as usize][col as usize] == 0 {
                for num in 1..=9 {
                    if is_valid(row, col, num, puzzle) {
                        puzzle[row as usize][col as usize] = num;
                        if solve(puzzle) {
                            return true;
                        }
                        puzzle[row as usize][col as usize] = 0;
                    }
                }
                return false;
            }
        }
    }

    true
}

fn display_puzzle(puzzle: &[[u8; 9]; 9], result: bool) -> () {
    for i in 0..9 {
        if i % 3 == 0 && i != 0 {
            println!("---------------------------------");
        }
        for j in 0..9 {
            if j % 3 == 0 && j != 0 {
                print!(" | ");
            }
            print!(" {} ", puzzle[i][j])
        }
        println!("");
    }
    println!("Solveable: {}", result);
}
