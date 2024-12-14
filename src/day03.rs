use itertools::Itertools;
use regex::Regex;

use crate::input;

pub fn solve_p1() {
    let input = input::read(3);
    let re = Regex::new(r"mul\(\d+\,\d+\)").unwrap();
    let matches_iter = re.find_iter(&input);
    let result: u32 = matches_iter
        .map(|m| {
            let mul_str = m.as_str();
            // For a string mul(X,Y) I here want the substring "X,Y" only
            let inside_paren = &mul_str[4..mul_str.len() - 1];
            let (one, two): (u32, u32) = inside_paren
                .split(',')
                .map(|n| n.parse::<u32>().unwrap())
                .collect_tuple()
                .unwrap();
            one * two
        })
        .sum();
    println!("{result}")

    //    for m in matches_iter {
    //        println!("{:?}", m.as_str())
    //    }
    //    let (mul_indices, _): (Vec<usize>, Vec<_>) = input.match_indices("mul(").collect();
}

pub fn solve_p2() {
    let input = input::read(3);
    let re_active = Regex::new(r"(?=(do\(\).*don\'t\(\)))(?:(?!don\'t).)*").unwrap();
    let valid_substrings = re_active.find_iter(&input);
    let valid_str = valid_substrings.map(|m| m.as_str()).join("");

    let re_mul = Regex::new(r"mul\(\d+\,\d+\)").unwrap();
    let matches_iter = re_mul.find_iter(&valid_str);
    let result: u32 = matches_iter
        .map(|m| {
            let mul_str = m.as_str();
            // For a string mul(X,Y) I here want the substring "X,Y" only
            let inside_paren = &mul_str[4..mul_str.len() - 1];
            let (one, two): (u32, u32) = inside_paren
                .split(',')
                .map(|n| n.parse::<u32>().unwrap())
                .collect_tuple()
                .unwrap();
            one * two
        })
        .sum();
    println!("{result}")
}
