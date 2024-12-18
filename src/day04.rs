use crate::input::read;

const XMAS: [char; 4] = ['X', 'M', 'A', 'S'];
const _TARGET_KERNEL: [[char; 7]; 7] = [
    ['S', ' ', ' ', 'S', ' ', ' ', 'S'],
    [' ', 'A', ' ', 'A', ' ', 'A', ' '],
    [' ', ' ', 'M', 'M', 'M', ' ', ' '],
    ['S', 'A', 'M', 'X', 'M', 'A', 'S'],
    [' ', ' ', 'M', 'M', 'M', ' ', ' '],
    [' ', 'A', ' ', 'A', ' ', 'A', ' '],
    ['S', ' ', ' ', 'S', ' ', ' ', 'S'],
];
const _TARGET_QUADRANT_REVERSE: [[char; 4]; 4] = [
    ['S', ' ', ' ', 'S'],
    [' ', 'A', ' ', 'A'],
    [' ', ' ', 'M', 'M'],
    ['S', 'A', 'M', 'X'],
];
const _TARGET_QUADRANT: [[char; 4]; 4] = [
    ['X', 'M', 'A', 'S'],
    ['M', 'M', ' ', ' '],
    ['A', ' ', 'A', ' '],
    ['S', ' ', ' ', 'S'],
];

pub fn solve_p1() {
    let input = read(4);
    let input: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let (n_rows, n_cols) = (input.len(), input.first().unwrap().len());
    //    println!("DIM {n_rows},{n_cols}");
    //    println!("{:?}", &input[0]);
    let mut total_matches = 0 as u32;

    // Check the "normal" horizontal case
    for r in 0..n_rows {
        for c in 0..(n_cols - 3) {
            if input[r][c] != 'X' {
                continue;
            }
            let horizontal_match = (input[r][c + 1] == XMAS[1])
                && (input[r][c + 2] == XMAS[2])
                && (input[r][c + 3] == XMAS[3]);
            total_matches += horizontal_match as u32;
        }
    }

    // Check the "normal" vertical case
    for r in 0..(n_rows - 3) {
        for c in 0..n_cols {
            if input[r][c] != 'X' {
                continue;
            }
            let vertical_match = (input[r + 1][c] == XMAS[1])
                && (input[r + 2][c] == XMAS[2])
                && (input[r + 3][c] == XMAS[3]);
            total_matches += vertical_match as u32;
        }
    }

    // Check the "reverse" horizontal case
    for r in 0..n_rows {
        for c in 3..n_cols {
            if input[r][c] != 'X' {
                continue;
            }
            let horizontal_match = (input[r][c - 1] == XMAS[1])
                && (input[r][c - 2] == XMAS[2])
                && (input[r][c - 3] == XMAS[3]);
            total_matches += horizontal_match as u32;
        }
    }

    // Check the "reverse" vertical case
    for r in 3..n_rows {
        for c in 0..n_cols {
            if input[r][c] != 'X' {
                continue;
            }
            let vertical_match = (input[r - 1][c] == XMAS[1])
                && (input[r - 2][c] == XMAS[2])
                && (input[r - 3][c] == XMAS[3]);
            total_matches += vertical_match as u32;
        }
    }

    // Check the diagonal SW case
    for r in 0..(n_rows - 3) {
        for c in 3..n_cols {
            if input[r][c] != 'X' {
                continue;
            }
            let diagonal_match_sw = (input[r + 1][c - 1] == XMAS[1])
                && (input[r + 2][c - 2] == XMAS[2])
                && (input[r + 3][c - 3] == XMAS[3]);
            total_matches += diagonal_match_sw as u32;
        }
    }

    // Check the diagonal NE case
    for r in 3..n_rows {
        for c in 0..(n_cols - 3) {
            if input[r][c] != 'X' {
                continue;
            }
            let diagonal_match_ne = (input[r - 1][c + 1] == XMAS[1])
                && (input[r - 2][c + 2] == XMAS[2])
                && (input[r - 3][c + 3] == XMAS[3]);
            total_matches += diagonal_match_ne as u32;
        }
    }

    // Check the diagonal SE case
    for r in 0..(n_rows - 3) {
        for c in 0..(n_cols - 3) {
            if input[r][c] != 'X' {
                continue;
            }
            let diagonal_match_ne = (input[r + 1][c + 1] == XMAS[1])
                && (input[r + 2][c + 2] == XMAS[2])
                && (input[r + 3][c + 3] == XMAS[3]);
            total_matches += diagonal_match_ne as u32;
        }
    }

    // Check the diagonal NW case
    for r in 3..n_rows {
        for c in 3..n_cols {
            if input[r][c] != 'X' {
                continue;
            }
            let diagonal_match_ne = (input[r - 1][c - 1] == XMAS[1])
                && (input[r - 2][c - 2] == XMAS[2])
                && (input[r - 3][c - 3] == XMAS[3]);
            total_matches += diagonal_match_ne as u32;
        }
    }

    println!("Part01: {}", total_matches);
}

pub fn solve_p2() {
    let input = read(4);
    let input: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let (n_rows, n_cols) = (input.len(), input.first().unwrap().len());
    //    println!("DIM {n_rows},{n_cols}");
    //    println!("{:?}", &input[0]);
    let mut total_matches = 0 as u32;

    for r in 1..n_rows - 1 {
        for c in 1..n_cols - 1 {
            if input[r][c] != 'A' {
                continue;
            }
            if (input[r - 1][c - 1] == 'M'
                && input[r + 1][c + 1] == 'S'
                && input[r + 1][c - 1] == 'M'
                && input[r - 1][c + 1] == 'S')
                || (input[r - 1][c - 1] == 'S'
                    && input[r + 1][c + 1] == 'M'
                    && input[r + 1][c - 1] == 'M'
                    && input[r - 1][c + 1] == 'S')
                || (input[r - 1][c - 1] == 'M'
                    && input[r + 1][c + 1] == 'S'
                    && input[r + 1][c - 1] == 'S'
                    && input[r - 1][c + 1] == 'M')
                || (input[r - 1][c - 1] == 'S'
                    && input[r + 1][c + 1] == 'M'
                    && input[r + 1][c - 1] == 'S'
                    && input[r - 1][c + 1] == 'M')
            {
                total_matches += 1
            }
        }
    }
    println!("Part02: {}", total_matches);
}
