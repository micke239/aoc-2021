use crate::aoc_util::{read_lines, neighbours_2d_bounds_no_diag};
use std::collections::{HashMap,VecDeque};
use std::time::{Instant};

pub fn find_path(from: (i64,i64), to: (i64,i64), map: &HashMap<(i64,i64), i64>, cheapest: &mut HashMap<(i64,i64), i64>) /*-> Vec<HashSet<(i64,i64)>>*/ {
    let mut queue: VecDeque<((i64,i64),(i64,i64))> = VecDeque::new();

    queue.push_back(((-1,-1), from));

    while let Some((past_node, n)) = queue.pop_front() {
        let weight = cheapest[&past_node] + map[&n];
        
        if cheapest.contains_key(&n) && weight >= cheapest[&n] || cheapest.contains_key(&to) && weight >= cheapest[&to] {
            continue;
        }

        cheapest.insert(n, weight);
        
        if n == to {
            continue;
        }
        
        let neighbours: Vec<(i64,i64)> = neighbours_2d_bounds_no_diag(n, to);
        for neighbour in neighbours {
            queue.push_back((n, neighbour));
        }
    } 
}

pub fn run() {
    let lines = read_lines("input/day15.txt");
    let mut map: HashMap<(i64,i64), i64> = HashMap::new();

    let mut i = 0;
    let mut j = 0;
    for line in lines {
        j = 0;
        for char in line.chars() {
            map.insert((i, j), char.to_digit(10).unwrap() as i64);
            j += 1;
        }
        i += 1;
    }

    for k in 0..5 {
        for l in 0..5 {
            if k == 0 && l == 0 {
                continue;
            }

            for x in 0..i {
                for y in 0..j {
                    let n = map[&(x,y)];
                    let n_x = k * i + x;
                    let n_y = l * j + y;

                    let mut n_n = n + k + l;
                    if n_n > 9 {
                        n_n = n_n % 9;
                    }

                    map.insert((n_x,n_y),n_n);
                }
            }
        }
    }
    
    let to = (i * 5 - 1, j * 5 - 1);
    let mut cheapest: HashMap<(i64, i64), i64> = HashMap::new();
    cheapest.insert((-1,-1), 0);

    let start = Instant::now();
    find_path((0,0), to, &map, &mut cheapest);
    println!("{} in {:?}", cheapest[&to] - map[&(0,0)], start.elapsed());
}