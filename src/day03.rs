use std::iter;

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
    println!("Part 01: {result}")

    //    for m in matches_iter {
    //        println!("{:?}", m.as_str())
    //    }
    //    let (mul_indices, _): (Vec<usize>, Vec<_>) = input.match_indices("mul(").collect();
}

pub fn solve_p2() {
    let input = input::read(3);

    let re_do = Regex::new(r"do\(\)").unwrap();
    let re_dont = Regex::new(r"don\'t\(\)").unwrap();
    let re_mul = Regex::new(r"mul\(\d+\,\d+\)").unwrap();

    let do_matches = re_do.find_iter(&input).map(|m| m.end());
    let do_matches = iter::once(0 as usize).chain(do_matches);
    let mut dont_matches = re_dont.find_iter(&input).map(|m| m.start());

    let mut latest_dont_pos = 0;
    let mut total_result = 0;
    for pos in do_matches {
        if pos < latest_dont_pos {
            continue;
        };
        while let Some(dont_pos) = dont_matches.next() {
            if dont_pos < pos {
                continue;
            }
            //            println!("Matching on {pos}..{dont_pos}");
            let matches_iter = re_mul.find_iter(&input[pos..dont_pos]);
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
            total_result += result;
            latest_dont_pos = dont_pos;
            break;
        }
    }
    println!("Part 02: {total_result}");
}
