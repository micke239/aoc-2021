use crate::aoc_util::read_lines;
use std::collections::{LinkedList,HashMap};

// fn g (map: &HashMap<char, i32>, c: char) -> &i32 {
//     match map.get(&c) {
//         Some(n) => n,
//         None => &0
//     }
// }

fn part1(lines: &Vec<String>) {
    let mut map: HashMap<char, char> = HashMap::new();
    map.insert('{', '}');
    map.insert('(', ')');
    map.insert('<', '>');
    map.insert('[', ']');
    
    let mut map2: HashMap<char, u64> = HashMap::new();
    map2.insert(')', 3 as u64);
    map2.insert(']', 57 as u64);
    map2.insert('}', 1197 as u64);
    map2.insert('>', 25137 as u64);

    let mut map3: HashMap<char, u64> = HashMap::new();
    map3.insert('(', 1 as u64);
    map3.insert('[', 2 as u64);
    map3.insert('{', 3 as u64);
    map3.insert('<', 4 as u64);
    
    let mut s2 = lines.into_iter().filter_map(|s| {
        let mut stack: LinkedList<char> = LinkedList::new();
        for c in s.chars() {
            let valid: bool = match c {
                '(' | '{' | '<' | '[' => {
                    stack.push_back(c);
                    true
                },
                ')' | '}' | '>' | ']' => {
                    match stack.pop_back() {
                        Some(c2) => { map[&c2] == c },
                        None => false
                    }
                },
                _ => panic!()
            };
            
            if !valid {
                // println!("{}", c);
                return None;
            }
        }

        let mut sumit = 0 as u64;
        while !stack.is_empty() {
            let c3 = stack.pop_back().unwrap();
            sumit *= 5;
            sumit += map3[&c3]; 
        }

        return Some(sumit); // invalid
    })
    .collect::<Vec<u64>>();

    s2.sort();

    
    println!("{}", s2[s2.len()/2]);
}

pub fn run() {
    let lines: Vec<String> = read_lines("input/day10.txt")
        .collect();

    part1(&lines);
    
}