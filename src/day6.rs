use std::collections::HashMap;

use crate::aoc_util::read_lines;

fn iterate(fishy : &HashMap<i32, u64>) -> HashMap<i32, u64> {
    let mut fishy_map: HashMap<i32, u64> = HashMap::new();
    
    for (key, value) in fishy {
        match key {
            0 => {
                if fishy_map.contains_key(&6) {
                    fishy_map.insert(6, fishy_map[&6] + value);
                }
                else {
                    fishy_map.insert(6, *value);
                }
                fishy_map.insert(8, *value);
            },
            n => {
                let nn = n - 1;
                if fishy_map.contains_key(&nn) {
                    fishy_map.insert(nn, fishy_map[&nn] + value);
                }
                else {
                    fishy_map.insert(nn, *value);
                }
            }
        }
    }

    return fishy_map;
}

fn print(fishy : &HashMap<i32, u64>) {
    for (key, value) in fishy {
        println!("{}, {}", key, value);
    }
}

pub fn run() {
    
    let lines = read_lines("input/day6.txt").collect::<Vec<String>>();
    let fishy: Vec<i32> = lines[0].split(",").map(|s| s.parse().unwrap()).collect();
    let mut fishy_map: HashMap<i32, u64> = HashMap::new();

    for fish in fishy {
        if fishy_map.contains_key(&fish) {
            fishy_map.insert(fish, fishy_map[&fish] + 1);
        }
        else {
            fishy_map.insert(fish, 1);
        }
    }

    for it in 0..256 {
        print(&fishy_map);
        fishy_map = iterate(&fishy_map);
        println!("{}", it);
    }
    print(&fishy_map);
    
    println!("{}", fishy_map.iter().map(|(_k, v)| v).sum::<u64>())
}