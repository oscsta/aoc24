use std::{cmp::Ordering, collections::{HashMap, HashSet}};

use itertools::Itertools;

use crate::input;

pub fn solve_p1() {
    let input = input::read(5);
    let mut input = input.split("\n\n");
    let (rules, updates) = (input.next().unwrap(), input.next().unwrap());
    let (rules, updates) = (rules.split('\n'), updates.split('\n'));

    // Transform rules into Vec of tuple of u32 to more easily use them later
    let rules = rules.map(|r| {
        let mut rule = r.split('|');
        (
            rule.next().unwrap().parse::<u32>().unwrap(),
            rule.next().unwrap().parse::<u32>().unwrap(),
        )
    });

    let mut sum = 0;
    'outer: for update in updates {
        // Build map to easily check index of each page
        let page_num_map: HashMap<u32, usize> = update
            .split(',')
            .enumerate()
            .map(|(idx, num)| (num.parse().unwrap(), idx))
            .collect();

        'inner: for rule in rules.clone() {
            // This is not great use of match. Maybe some "if let" thingy is better
            match (page_num_map.get(&rule.0), page_num_map.get(&rule.1)) {
                (Some(first), Some(second)) => {
                    if first > second {
                        continue 'outer;
                    }
                }
                (_, _) => continue 'inner,
            }
        }

        let updatev: Vec<u32> = update.split(',').map(|num| num.parse().unwrap()).collect();
        sum += updatev[updatev.len() / 2];
    }
    println!("Part01: {sum}");
}

pub fn solve_p2() {
    let input = input::read(5);
    let mut input = input.split("\n\n");
    let (rules, updates) = (input.next().unwrap(), input.next().unwrap());
    let (rules, updates) = (rules.split('\n'), updates.split('\n'));

    let rules = rules.map(|r| {
        let mut rule = r.split('|');
        (
            rule.next().unwrap().parse::<u32>().unwrap(),
            rule.next().unwrap().parse::<u32>().unwrap(),
        )
    });

    let mut rule_set: HashMap<u32, HashSet<u32>> = HashMap::new();
    for rule in rules {
        rule_set.entry(rule.0).or_default().insert(rule.1);
    }

    let comparison = |one, two| {
        if let (Some(set1), Some(set2)) = (rule_set.get(one), rule_set.get(two)) {
            if set1.contains(two) {
                true
            } else if set2.contains(one) {
                false
            } else {
                true
            }
        } else {
            true
        }
    };

    let mut sum = 0;
    for update in updates {
        let update = update.split(',').map(|n| n.parse::<u32>().unwrap());

        if update.clone().is_sorted_by(|one, two| {
            if let (Some(set1), Some(set2)) = (rule_set.get(one), rule_set.get(two)) {
                if set1.contains(two) {
                    true
                } else if set2.contains(one) {
                    false
                } else {
                    true
                }
            } else {
                true
            }
        }) {
            continue
        }

        let update = update.sorted_by(|one, two| {
            if let (Some(set1), Some(set2)) = (rule_set.get(one), rule_set.get(two)) {
                if set1.contains(two) {
                    Ordering::Less
                } else if set2.contains(one) {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            } else {
                Ordering::Equal
            }
        });

        let update: Vec<u32> = update.collect();
        sum += update[update.len() / 2];
    }
    println!("Part02: {sum}");
}
