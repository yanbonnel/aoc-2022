use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};

pub fn day6() {
    let line = include_str!("./day6.txt")
        .split("\n").next().unwrap();

    let mut step1_result = None;
    let mut step2_result = None;

    for index in 0..line.len() {
        if index >= 3 && line.chars().skip(index - 3).take(4).collect::<HashSet<_>>().len() == 4 &&
            step1_result.is_none()
        {
            step1_result = Some(index+1);
        }

        if index >= 13 && line.chars().skip(index - 13).take(14).collect::<HashSet<_>>().len() == 14 &&
            step2_result.is_none()
        {
            step2_result = Some(index+1);
        }
    }

    println!("Step 1 : {}", step1_result.unwrap());
    println!("Step 2 : {}", step2_result.unwrap());

}