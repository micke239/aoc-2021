use crate::aoc_util::read_lines;
use std::collections::{HashMap,HashSet};

pub fn walk(paths: &HashMap<String, HashSet<String>>, path: &String, past_travel: &HashSet<String>, has_twice: bool) -> Vec<Vec<String>>{
    let mut p: Vec<Vec<String>> = Vec::new();

    for pat in &paths[path] {
        let is_small = pat.chars().all(char::is_lowercase);
        if is_small && past_travel.contains(pat) {
            continue;
        }

        if pat == "end" {
            p.push(vec![String::from(pat)]);
            continue;
        }
        
        if is_small && !has_twice {
            let n_p = walk(paths, pat, past_travel, true);
            for n_p_p in n_p {
                let mut i_p: Vec<String> = Vec::new();
                i_p.push(String::from(pat));
                for p_p in n_p_p {
                    i_p.push(p_p);
                }
                p.push(i_p);
            }
        }

        let mut pt: HashSet<String> = past_travel.into_iter().map(String::from).collect();
        pt.insert(String::from(pat));

        let n_p = walk(paths, pat, &pt, has_twice);
        for n_p_p in n_p {
            let mut i_p: Vec<String> = Vec::new();
            i_p.push(String::from(pat));
            for p_p in n_p_p {
                i_p.push(p_p);
            }
            p.push(i_p);
        }
    }
    
    return p;
}


pub fn run() {
    let lines = read_lines("input/day12.txt")
        .map(|s| {
            let split: Vec<&str> = s.split("-").collect();
            return (String::from(split[0]), String::from(split[1]));
        });
    
    let mut ways: HashMap<String, HashSet<String>> = HashMap::new();
    for (s1, s2) in lines {
        if let Some(set) = ways.get_mut(&s1) {
            set.insert(String::from(&s2));
        }
        else {
            let mut set: HashSet<String> = HashSet::new();
            set.insert(String::from(&s2));
            ways.insert(String::from(&s1), set); 
        }

        if let Some(set) = ways.get_mut(&s2) {
            set.insert(s1);
        }
        else {
            let mut set: HashSet<String> = HashSet::new();
            set.insert(s1);
            ways.insert(s2, set); 
        }
    }

    for (s, i_way) in &ways {
        let s2 = i_way.into_iter().map(String::from).collect::<Vec<String>>().join(",");
        println!("{}: {}", s, s2);
    }

    let mut s = HashSet::<String>::new();
    s.insert(String::from("start"));
    let ways2 = walk( &ways, &String::from("start"), &s, false);

    let ways3 = (&ways2).into_iter()
        .map(|x| x.into_iter().map(String::from).collect::<Vec<String>>().join(","))
        .collect::<HashSet<String>>();

    for i_way in &ways3 {
        println!("start,{}", i_way);
    }

    println!("{}", ways3.len());
    
}