use crate::aoc_util::read_lines;
use std::collections::HashSet;

fn part1(lines: &Vec<(Vec<String>, Vec<String>)>) {
    let count: i32 = (&lines)
        .into_iter()
        .map(|(_input, output)|
            output.into_iter()
                .map(|o| match o.len() {
                    2 => 1,
                    4 => 1,
                    3 => 1,
                    7 => 1,
                    _y => 0
                })
                .sum::<i32>()
        )
        .sum();

    println!("{}", count)
}

pub fn run() {
    let lines: Vec<(Vec<String>, Vec<String>)> = read_lines("input/day8.txt")
        .map(|s| {
            let split: Vec<&str> = s.split(" | ").collect();
            let input: Vec<String> = split[0].split(" ").map(String::from).collect();
            let output: Vec<String> = split[1].split(" ").map(String::from).collect();

            return (input, output);
        })
        .collect();

    part1(&lines);
    
    let count: u64 = lines
        .into_iter()
        .map(|(input, output)| {
            let mut positions: Vec<HashSet<char>> = vec![HashSet::new(),HashSet::new(),HashSet::new(),HashSet::new(),HashSet::new(),HashSet::new(),HashSet::new(),HashSet::new(),HashSet::new(),HashSet::new()];

            for i in &input {
                match i.len() {
                    2 => positions[1] = i.as_bytes().iter().map(|b| *b as char).collect(),
                    4 => positions[4] = i.as_bytes().iter().map(|b| *b as char).collect(),
                    3 => positions[7] = i.as_bytes().iter().map(|b| *b as char).collect(),
                    7 => positions[8] = i.as_bytes().iter().map(|b| *b as char).collect(),
                    _ => println!("no good")
                }
            }

            // 6
            for i in (&input).iter().filter(|s| s.len() == 6) {
                let p: HashSet<char> = i.as_bytes().iter().map(|b| *b as char).collect();
                let n: HashSet<&char> = p.intersection(&positions[1]).collect();
                if n.len() == 1 {
                    positions[6] = p;
                }
            }

            // 5
            for i in (&input).iter().filter(|s| s.len() == 5) {
                let p: HashSet<char> = i.as_bytes().iter().map(|b| *b as char).collect();
                let n: HashSet<&char> = p.intersection(&positions[6]).collect();
                if n.len() == 5 {
                    positions[5] = p;
                }
            }

            // 2, 3
            for i in (&input).iter().filter(|s| s.len() == 5) {
                let p: HashSet<char> = i.as_bytes().iter().map(|b| *b as char).collect();
                let n: HashSet<&char> = p.intersection(&positions[5]).collect();
                if n.len() == 4 {
                    positions[3] = p;
                }
                else if n.len() == 3 {
                    positions[2] = p;
                }
            }

            
            // 9
            for i in (&input).iter().filter(|s| s.len() == 6) {
                let p: HashSet<char> = i.as_bytes().iter().map(|b| *b as char).collect();
                let n: HashSet<&char> = p.intersection(&positions[3]).collect();
                if n.len() == 5 {
                    positions[9] = p;
                }
            }

            // 0
            for i in (&input).iter().filter(|s| s.len() == 6) {
                let p: HashSet<char> = i.as_bytes().iter().map(|b| *b as char).collect();
                let n9: HashSet<&char> = p.intersection(&positions[9]).collect();
                let n6: HashSet<&char> = p.intersection(&positions[6]).collect();
                if n9.len() == 5 && n6.len() == 5 {
                    positions[0] = p;
                }
            }

            let mut output_string: String = String::new();

            for s in output {
                let p: HashSet<char> = s.as_bytes().iter().map(|b| *b as char).collect();
                for i in 0..positions.len() {
                    if positions[i].len() == p.len() {
                        let n: HashSet<&char> = p.intersection(&positions[i]).collect();
                        if n.len() == p.len() {
                            output_string += &i.to_string();
                            break;
                        }
                    }
                    println!("fail");
                }
            }
            let n22 = output_string.parse::<u64>().unwrap();
            return n22;
        })
        .sum();

    println!("{}", count)
}