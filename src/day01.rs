use std::{
    cmp::{max, min},
    fs,
    path::Path,
};

use itertools::sorted;

pub fn read_input() -> String {
    let input_path = Path::new("./input/day01_input.txt");
    let input_str = fs::read_to_string(input_path).expect("Should find input files");
    input_str
}

pub fn solve_p1(input: String) {
    let in_vals: Vec<&str> = input.split('\n').collect();

    let (left, right): (Vec<u32>, Vec<u32>) = in_vals
        .into_iter()
        .map(|r| {
            let srow = r.split_whitespace().collect::<Vec<&str>>();
            (
                srow[0].parse::<u32>().unwrap(),
                srow[1].parse::<u32>().unwrap(),
            )
        })
        .collect();

    let (left_sorted, right_sorted): (Vec<u32>, Vec<u32>) =
        (sorted(left).collect(), sorted(right).collect());
    //    println!("{:?}\n{:?}", &left_sorted[0..10], &right_sorted[0..10]);

    let diff: u32 = left_sorted
        .iter()
        .zip(right_sorted.iter())
        .map(|(l, r)| max(l, r) - min(l, r))
        .sum();
    println!("Part 1 answer: {diff:?}")
}
