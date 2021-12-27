use std::{cmp};
use std::collections::HashSet;

use crate::aoc_util::read_lines;

struct Count {
    one_count: i32,
    zero_count: i32
}

fn parse(lines: &Vec<String>) -> Vec<((i32,i32),(i32,i32))> {
    return lines.iter()
        .map(|line| {
            let x = line
                .split(" -> ")
                .map(|s2| {
                    let split = s2.split(",").collect::<Vec<&str>>();
                    return (split[0].parse::<i32>().unwrap(), split[1].parse::<i32>().unwrap());
                })
                .collect::<Vec<(i32,i32)>>();
            let (x1,y1) = x[0];
            let (x2,y2) = x[1];

            return ((x1,y1),(x2,y2));
        }
    )
    .collect::<Vec<((i32,i32),(i32,i32))>>();
}

fn part1(coords : &Vec<((i32,i32),(i32,i32))>) {
    let ncoords: Vec<(i32,i32)> = coords.iter()
        .flat_map(|((x1,y1),(x2,y2))| {
            let mut cs: Vec<(i32,i32)> = Vec::new();
            if x1 == x2 {
                let start = cmp::min(y1,y2).to_owned();
                let end = cmp::max(y1,y2).to_owned() + 1;
                for y in start..end {
                    cs.push((*x1,y));
                }
            }
            else if y1 == y2 {
                let start = cmp::min(x1,x2).to_owned();
                let end = cmp::max(x1,x2).to_owned() + 1;
                for x in start..end {
                    cs.push((x,*y1));
                }
            }
            else {
                if y2 > y1 && x2 > x1 {
                    for i in 0..((y2-y1)+1) {
                        cs.push((x1+i, y1+i))
                    }
                }
                else if y2 > y1 {
                    for i in 0..((y2-y1)+1) {
                        cs.push((x1-i, y1+i))
                    }
                }
                else if x2 > x1 {
                    for i in 0..((x2-x1)+1) {
                        cs.push((x1+i, y1-i))
                    }
                }
                else {
                    for i in 0..((x1-x2)+1) {
                        cs.push((x1-i, y1-i))
                    }
                }
            }

            return cs;
        })
        .collect();
    
    let mut once: HashSet<(i32,i32)> = HashSet::new();
    let mut twice: HashSet<(i32,i32)> = HashSet::new();
    
    for c in ncoords {
        if !once.contains(&c) {
            once.insert(c);
        }
        else if !twice.contains(&c) {
            twice.insert(c);
        }
    }
    
    println!("{}", twice.len())
}

pub fn run() {
    
    let lines = read_lines("input/day5.txt").collect::<Vec<String>>();
    let coords = parse(&lines);
    
    part1(&coords);
}