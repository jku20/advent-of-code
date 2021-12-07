#![allow(non_snake_case)]
use std::io::{self, prelude::*};

static INPUT: &str = include_str!("input");
const N: usize = 256;

fn main() {
    let nums = INPUT.trim().split(",").map(|x| x.parse().unwrap()).collect::<Vec<i32>>();

    let mut ans = i32::MAX;
    for i in 0..2000 {
        ans = ans.min(nums.iter().map(|&x| ((x - i).abs() * ((x - i).abs() + 1) / 2)).sum());
    }

    println!("{}", ans);
}
