#![allow(non_snake_case)]
use std::io::{self, prelude::*};

static INPUT: &str = include_str!("input");

fn main() {
    let mut x = 0;
    let mut y = 0;
    let mut a = 0;
    for line in INPUT.lines() {
        let mut cmd = line.split(' ');
        match cmd.next().unwrap() {
            "up"      => a -= cmd.next().unwrap().parse::<i32>().unwrap(),
            "down"    => a += cmd.next().unwrap().parse::<i32>().unwrap(),
            "forward" => {
                let s = cmd.next().unwrap().parse::<i32>().unwrap();
                x += s;
                y += a * s;
            }
            _ => panic!("bad input"),
        }
    }
    println!("{}", x * y);
}
