use std::{fs, path::Path};

pub fn read_input() -> String {
    let input_path = Path::new("./input/day02_input.txt");
    let input_str = fs::read_to_string(input_path).expect("Should find input files");
    input_str
}

pub fn solve_p1(input: &String) {
    let mut num_safe: u32 = 0;
    let reports = input.split('\n');
    for report in reports {
        let levels: Vec<i32> = report
            .split_whitespace()
            .map(|lvl| lvl.parse::<i32>().unwrap())
            .collect();
        let pair_iter = levels.windows(2);
        let diffs = pair_iter.map(|pair| pair[1] - pair[0]);

        let is_safe_inc = diffs.clone().all(|diff| (0 < diff) && (diff < 4));
        let is_safe_dec = diffs.clone().all(|diff| (-4 < diff) && (diff < 0));

        if is_safe_inc || is_safe_dec {
            num_safe += 1;
        }
    }
    println!("{num_safe}")
}

pub fn solve_p2(input: &String) {
    let reports = input.split('\n');
    for report in reports {
        let levels: Vec<i32> = report
            .split_whitespace()
            .map(|lvl| lvl.parse::<i32>().unwrap())
            .collect();
        let diffs = levels.windows(2).map(|pair| pair[1] - pair[0]);


    }
}

fn is_diff_safe_dampener(it: &impl Iterator<Item=i32>) -> bool {
    true
}
