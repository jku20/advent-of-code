#![allow(non_snake_case)]
use std::io::{self, prelude::*};

static INPUT: &str = include_str!("input");

fn adj_pnts(x: usize, y: usize, g: &Vec<Vec<u32>>) -> Vec<(usize, usize)> {
    let mut out = vec![];
    if y > 0 {
        out.push((x, y - 1));
    }
    if x > 0 {
        out.push((x - 1, y));
    }
    if x + 1 < g.len() {
        out.push((x + 1, y));
    }
    if y + 1 < g[0].len() {
        out.push((x, y + 1));
    }
    out
}

fn lower_pnts(x: usize, y: usize, g: &Vec<Vec<u32>>) -> Vec<(usize, usize)> {
    adj_pnts(x, y, g).into_iter()
        .filter(|&(nx, ny)| g[nx][ny] != 9)
        .collect()
}

fn dfs(x: usize, y: usize, vis: &mut Vec<Vec<bool>>, g: &Vec<Vec<u32>>) -> i32 {
    if vis[x][y] {
        return 0;
    } 
    vis[x][y] = true;

    let mut cur = 1;
    for (nx, ny) in lower_pnts(x, y, g) {
        cur += dfs(nx, ny, vis, g);
    }
    cur
}

fn main() {
    let g = INPUT.lines()
        .map(|s| s.bytes()
             .map(|b| (b-b'0') as u32)
             .collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let (n, m) = (g.len(), g[0].len());

    let mut vis = vec![vec![false; m]; n];
    let mut basens = vec![];

    for i in 0..n {
        for j in 0..m {
            basens.push(dfs(i, j, &mut vis, &g) - 1);
        }
    }

    basens.sort_unstable();
    let bl = basens.len();
    println!("{}", basens[bl-1] * basens[bl-2] * basens[bl-3]);
}
