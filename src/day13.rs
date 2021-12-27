use crate::aoc_util::read_lines;
use std::collections::{HashSet};

pub fn run() {

    let lines = read_lines("input/day13.txt");
    let mut coords: HashSet<(i32,i32)> = HashSet::new();
    let mut folds: Vec<(String, i32)> = Vec::new();

    let mut parsing_coords = true;
    for line in lines {
        if line == "" {
            parsing_coords = false;
        } else if parsing_coords {
            let split: Vec<i32> = line.split(",").map(|s| s.parse().unwrap()).collect();
            coords.insert((split[0], split[1]));
        } else {
            let split: Vec<&str> = line[11..].split("=").collect();
            folds.push((String::from(split[0]), split[1].parse().unwrap()))
        }
    }

//     let max_x2 = coords.iter()
//     .map(|(x, _y)| *x)
//     .max()
//     .unwrap();
// let max_y2 = coords.iter()
//     .map(|(_x, y)| *y)
//     .max()
//     .unwrap();
    // for i in 0..max_x2 {
    //     for j in 0..max_y2 {
    //         if coords.contains(&(i,j)) {
    //             print!("#");
    //         } else {
    //             print!(".");
    //         }
    //     }
    //     println!("");
    // }

    for (fold_dir, fold_v) in &folds {
        if fold_dir == "x" {
            coords = coords.into_iter()
                .map(|(x,y)| {
                    if x < *fold_v {
                        (x,y)
                    }
                    else {
                        let n_x = fold_v - (x - fold_v);
                        (n_x, y)
                    }
                })
                .collect();
        }
        else {
            coords = coords.into_iter()
                .map(|(x,y)| {
                    if y < *fold_v {
                        (x,y)
                    }
                    else {
                        let n_y = fold_v - (y - fold_v);
                        (x, n_y)
                    }
                })
                .collect();
        }
    }
    
    let max_x = coords.iter()
    .map(|(x, _y)| *x)
    .max()
    .unwrap();
let max_y = coords.iter()
    .map(|(_x, y)| *y)
    .max()
    .unwrap();

    for i in -1..(max_y+2) {
        for j in -1..(max_x+2) {
            if coords.contains(&(j,i)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
 

    println!("{}", coords.len())
}