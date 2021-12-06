#![allow(non_snake_case)]
use std::io::{self, prelude::*};

static INPUT: &str = include_str!("input");
const N: usize = 256;

fn main() {
    let ls = INPUT.trim().split(",").map(|x| x.parse().unwrap()).collect::<Vec<usize>>();
    let mut fish = vec![0,0,0,0,0,0,0,0,0];

    for i in ls {
        fish[i] += 1;
    }

    for _ in 0..N {
        let nw_fsh = fish[0];
        fish[7] += fish[0];
        fish[0] = 0;
        for j in 1..9 {
            fish[j-1] += fish[j];
            fish[j] = 0;
        }
        fish[8] += nw_fsh;
    }

    println!("{}", fish.into_iter().sum::<usize>());
}
