use crate::aoc_util::read_lines;
use std::collections::{HashMap};

pub fn run() {


    
    let lines = read_lines("input/day14.txt");
    let mut str: String = String::new();
    let mut pairs: HashMap<(char, char), char> = HashMap::new();

    let mut starting = true;
    for line in lines {
        if line == "" {
            starting = false;
        } else if starting {
            str = line;
        } else {
            let split: Vec<&str> = line.split(" -> ").collect();
            pairs.insert((split[0].chars().next().unwrap(), split[0].chars().next_back().unwrap()), split[1].chars().next().unwrap());
        }
    } //NBCCNBBBCBHCB //NNCB

    let mut pair_cnt: HashMap<(char, char), u64> = HashMap::new();
    for i in 0..(str.len()-1) {
        let chars: Vec<char> = (&str[i..(i+2)]).chars().collect();
        let first_char = chars[0];
        let second_char = chars[1];
        
        let key = (first_char, second_char);
        if pair_cnt.contains_key(&key) {
            pair_cnt.insert(key, pair_cnt[&key] + 1);
        } else {
            pair_cnt.insert(key, 1);
        }
    }

    let mut new_pair_cnt: HashMap<(char, char), u64> = HashMap::new();
    for _k in 0..40 {
        for (pair, old_cnt) in &pair_cnt {

            let (c1, c2) = pair.clone();
            let new_char = pairs[&pair];

            let p1 = (c1, new_char);
            let p2 = (new_char, c2);
            
            if new_pair_cnt.contains_key(&p1) {
                new_pair_cnt.insert(p1, new_pair_cnt[&p1] + old_cnt);
            } else {
                new_pair_cnt.insert(p1, old_cnt.clone());
            }

            if new_pair_cnt.contains_key(&p2) {
                new_pair_cnt.insert(p2, new_pair_cnt[&p2] + old_cnt);
            } else {
                new_pair_cnt.insert(p2, old_cnt.clone());
            }
        }

        pair_cnt = new_pair_cnt;
        new_pair_cnt = HashMap::new();
    }

    let mut counts: HashMap<char, u64> = HashMap::new();

    counts.insert(str.chars().next_back().unwrap(), 1);
    for ((c1,_c2), i) in pair_cnt {
        if counts.contains_key(&c1) {
            counts.insert(c1, counts[&c1] + i);
        }
        else {
            counts.insert(c1, i);
        }
    }


    let mut c2: Vec<(char, u64)> = counts.into_iter().collect();

    c2.sort_by(|(_c1, c1_2), (_c2, c2_2)| c1_2.partial_cmp(c2_2).unwrap());

    let (_cx, min) = c2[0];
    let (_cx, max) = c2[c2.len()-1];

    println!("{}", max - min);
}