use crate::aoc_util::read_lines;
use std::collections::{HashSet};
use std::cmp;

fn maybe_flash(lines: &mut Vec<Vec<u32>>, i: i32, j: i32, inc: bool, flashed: &mut HashSet<(usize,usize)>) {
    if i < 0 {
        return;
    }
    let x = i as usize;
    if x >= lines.len() {
        return;
    }
    if j < 0 {
        return;
    }
    let y = j as usize;
    if y >= lines[x].len() {
        return;
    }

    if flashed.contains(&(x,y)) {
        return;
    }

    if inc {
        lines[x][y] = cmp::min(10, lines[x][y] + 1);
    }

    if lines[x][y] == 10 {
        flashed.insert((x,y));
        maybe_flash(lines, i+1, j+1, true, flashed);
        maybe_flash(lines, i+1, j, true, flashed);
        maybe_flash(lines, i+1, j-1, true,flashed);
        maybe_flash(lines, i, j+1, true,flashed);
        maybe_flash(lines, i, j-1, true,flashed);
        maybe_flash(lines, i-1, j+1, true,flashed);
        maybe_flash(lines, i-1, j, true,flashed);
        maybe_flash(lines, i-1, j-1, true,flashed);
    }
}

fn part1(lines: &mut Vec<Vec<u32>>) -> u64{
    let mut flashed: HashSet<(usize, usize)> = HashSet::new();

    for i in 0..lines.len() {
        for j in 0..lines[i].len() {
            print!("{} ",lines[i][j]);
        }
        println!("");
    }


    for i in 0..lines.len() {
        for j in 0..lines[i].len() {
            lines[i][j] = cmp::min(10, lines[i][j] + 1);
        }
    }
    
    for i in 0..lines.len() {
        for j in 0..lines[i].len() {
            maybe_flash(lines, i as i32, j as i32, false, &mut flashed);
        }
    }

    for i in 0..lines.len() {
        for j in 0..lines[i].len() {
            if lines[i][j] == 10 {
                lines[i][j] = 0;
            }
        }
    }

    return flashed.len() as u64;
}

pub fn run() {
    let mut lines: Vec<Vec<u32>> = read_lines("input/day11.txt")
        .map(|s| s.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let mut flashes: u64 = 0;
    for i in 0..10000 {

        println!("{}, {}", i, flashes);
        let inner_flashes = part1(&mut lines);

        if inner_flashes == 100 {
            println!("{}", i);
            break;
        }

        flashes += inner_flashes;
    }
    
    println!("{}", flashes);
}