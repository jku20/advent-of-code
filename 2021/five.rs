#![allow(non_snake_case)]
use std::io::{self, prelude::*};

static INPUT: &str = include_str!("input");
const N: usize = 1000;

fn drw_ln(x1: i32, y1: i32, x2: i32, y2: i32, b: &mut Vec<Vec<usize>>) {
    let dx = if x1 > x2 { -1 } else if x1 < x2 { 1 } else { 0 };
    let dy = if y1 > y2 { -1 } else if y1 < y2 { 1 } else { 0 };

    let d = ((x2-x1) * dx).max((y2-y1) * dy) + 1;

    for i in 0..d {
        b[(x1 + i*dx) as usize][(y1 + i*dy) as usize] += 1;
    }
}

fn main() {
    let ls = INPUT.lines().map(|s| {
        s.split(&[',', ' ', '-', '>'][..])
            .filter(|x| !x.is_empty())
            .map(|x| x.parse().unwrap())
            .collect()
    }).collect::<Vec<Vec<_>>>();

    let mut b = vec![vec![0; N]; N];

    for l in ls {
        drw_ln(l[0], l[1], l[2], l[3], &mut b);
    }

    println!("{}", b.into_iter().flatten().filter(|&x| x >= 2).count());
}
