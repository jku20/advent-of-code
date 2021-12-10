#![allow(non_snake_case)]
use std::io::{self, prelude::*};
use std::collections::{VecDeque, HashMap, HashSet};

static INPUT: &str = include_str!("input");

fn mod_stk(stk: &mut VecDeque<u8>, c: u8) -> Option<u8> {
    let opn: HashSet<u8> = [b'(', b'[', b'{', b'<'].into();
    if opn.contains(&c) {
        stk.push_front(c);
        None
    } else {
        stk.pop_front()
    }
}

fn corrupt(l: &Vec<u8>) -> bool {
    let mtc: HashMap<u8, u8> = [(b')', b'('), (b']', b'['), (b'}', b'{'), (b'>', b'<')].into();
    let mut stk = VecDeque::<u8>::new();
    l.iter()
        .map(|&c| {
            let a = mod_stk(&mut stk, c);
            a.is_some() && a.unwrap() != mtc[&c]
        })
        .any(|v| v)
}

fn score(l: &Vec<u8>) -> u64 {
    let mut stk = VecDeque::<u8>::new();
    for c in l {
        mod_stk(&mut stk, *c);
    }

    let scr: HashMap<u8, u64> = [(b'(', 1), (b'[', 2), (b'{', 3), (b'<', 4)].into();
    stk.iter().fold(0, |acc, c| acc * 5 + scr[c])
}

fn main() {
    let mut scrs = INPUT.lines()
        .map(|s| s.bytes().collect::<Vec<_>>())
        .filter(|l| !corrupt(l))
        .map(|l| score(&l))
        .collect::<Vec<_>>();
    scrs.sort_unstable();
    println!("{}", scrs[scrs.len() / 2]);
}
