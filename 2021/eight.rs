//Woah, this is a really bad final code solution
#![allow(non_snake_case)]
use std::collections::{HashMap, HashSet};
use std::io::{self, prelude::*};

static INPUT: &str = include_str!("input");

fn share(a: &Vec<u8>, known: &HashMap<&Vec<u8>, usize>) -> HashSet<usize> {
    let mut out = HashSet::new();
    for i in known.keys() {
        out.insert(
            a.iter()
                .filter(|&x| i.iter().find(|&c| x == c) != None)
                .count(),
        );
    }
    out
}

fn main() {
    let ZERO: HashSet<usize> = [2, 3, 3, 6].into();
    let TWO: HashSet<usize> = [1, 2, 2, 5].into();
    let THREE: HashSet<usize> = [2, 3, 3, 5].into();
    let FIVE: HashSet<usize> = [1, 2, 3, 5].into();
    let SIX: HashSet<usize> = [1, 2, 3, 6].into();
    let NINE: HashSet<usize> = [2, 4, 3, 6].into();

    let t = INPUT.lines().map(|x| x.split("|").collect::<Vec<_>>());
    //0-9s
    let mut a = vec![];
    //www
    let mut b = vec![];

    for l in t {
        let a_i = l[0]
            .split_whitespace()
            .filter(|x| !x.is_empty())
            .map(|s| {
                let mut y = s.as_bytes().to_owned();
                y.sort();
                y
            })
            .collect::<Vec<_>>();
        let b_i = l[1]
            .split_whitespace()
            .filter(|x| !x.is_empty())
            .map(|s| {
                let mut y = s.as_bytes().to_owned();
                y.sort();
                y
            })
            .collect::<Vec<_>>();
        a.push(a_i);
        b.push(b_i);
    }

    let mut ans = 0;

    for (notes, to_solve) in a.iter().zip(b.iter()) {
        let mut mp1 = HashMap::new();
        for s in notes {
            //easy cases
            match s.len() {
                2 => mp1.insert(s, 1),
                4 => mp1.insert(s, 4),
                3 => mp1.insert(s, 7),
                7 => mp1.insert(s, 8),
                _ => None,
            };
        }
        let notes = notes
            .into_iter()
            .filter(|l| l.len() != 2 && l.len() != 4 && l.len() != 3 && l.len() != 7)
            .collect::<Vec<_>>();
        //we know 1, 4, 7, 8
        let mut nms = vec![];
        for &s in &notes {
            nms.push(share(s, &mp1));
        }
        for (h, &s) in nms.into_iter().zip(notes.iter()) {
            if h == ZERO {
                mp1.insert(s, 0);
            } else if h == TWO {
                mp1.insert(s, 2);
            } else if h == THREE {
                mp1.insert(s, 3);
            } else if h == FIVE {
                mp1.insert(s, 5);
            } else if h == SIX {
                mp1.insert(s, 6);
            } else if h == NINE {
                mp1.insert(s, 9);
            }
        }
        let mut cur = 0;
        for s in to_solve {
            cur *= 10;
            cur += mp1.get(s).unwrap();
        }
        ans += cur;
    }
    println!("{}", ans);
}
