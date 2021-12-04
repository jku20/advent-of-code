#![allow(non_snake_case)]
use std::io::{self, prelude::*};

static INPUT: &str = include_str!("input");

//m = 0 is oxy, m = 1 is co2
fn oxyco2(l: &mut Vec<Vec<usize>>, m: usize) -> usize {
    let mut d = 0;
    while l.len() != 1 {
        let c = if l.iter().filter(|&b| b[d] == 1usize).count() >= (l.len() + 1) / 2 { 1 - m } else { m };
        l.retain(|x| x[d] == c);
        d += 1;
    }
    l[0].iter().fold(0, |acc, &x| (acc<<1) + x)
}

fn main() {
    let mut a = INPUT.lines().map(|s|s.bytes().map(|b| (b-b'0') as usize).collect::<Vec<_>>()).collect::<Vec<_>>();
    let mut b = a.clone();

    let oxy = oxyco2(&mut a, 0);
    let co2 = oxyco2(&mut b, 1);

    println!("{}", oxy*co2);
}
