#![allow(non_snake_case)]
use std::io::{self, prelude::*};

static INPUT: &str = include_str!("input");

fn main() {
    println!("{}",
        INPUT
        .lines()
        .map(|x| x.parse().unwrap())
        .collect::<Vec<u32>>()
        .windows(3)
        .map(|x| x[0] + x[1] + x[2])
        .collect::<Vec<u32>>()
        .windows(2)
        .filter(|x| x[1] > x[0])
        .count()
    );
}
