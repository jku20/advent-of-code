#![allow(non_snake_case)]
use std::io::{self, prelude::*};

static INPUT: &str = include_str!("input");

fn score(b: &Vec<Vec<u32>>, d: &Vec<u32>) -> u32 {
    let mut mp = vec![vec![false; b[0].len()]; b.len()];

    let mut i = 0;
    while !done(&mp) {
        pt(&b, &mut mp, d[i]);
        i += 1;
    }

    let mut scr = 0;
    for j in 0..b.len() {
        for k in 0..b[j].len() {
            if !mp[j][k] {
                scr += b[j][k];
            }
        }
    }
    scr * d[i-1]
}

fn time(b: &Vec<Vec<u32>>, d: &Vec<u32>) -> usize {
    let mut mp = vec![vec![false; b[0].len()]; b.len()];

    let mut i = 0;
    while !done(&mp) {
        pt(&b, &mut mp, d[i]);
        i += 1;
    }
    i
}

fn pt(b: &Vec<Vec<u32>>, mp: &mut Vec<Vec<bool>>, n: u32) {
    for i in 0..b.len() {
        for j in 0..b[i].len() {
            if b[i][j] == n {
                mp[i][j] = true;
            }
        }
    }
}

fn done(b: &Vec<Vec<bool>>) -> bool {
    for i in 0..b.len() {
        let mut c1 = true;
        let mut c2 = true;
        for j in 0..b[i].len() {
            c1 &= b[i][j];
            c2 &= b[j][i];
        }
        if c1 || c2 { return true; }
    }
    false
}


fn main() {
    //ugly IO I should really do iteratively
    let mut it = INPUT.lines();
    let d = it.next().unwrap().split(",").map(|x| x.parse().unwrap()).collect();
    let boards: Vec<Vec<Vec<_>>> = it.filter(|s| !s.is_empty())
        .collect::<Vec<_>>().windows(5).step_by(5).map(|b| 
            b.into_iter().map(|l| l.split_whitespace().map(|n| 
                n.parse().unwrap()
        ).collect()).collect()
    ).collect();

    let st = boards.into_iter().map(|b| (time(&b, &d), score(&b, &d))).collect::<Vec<_>>();
    let first = st.iter().min().unwrap();
    let last = st.iter().max().unwrap();

    println!("first: {}", first.1);
    println!("last: {}", last.1);
}
