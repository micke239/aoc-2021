use crate::aoc_util::{read_lines};
use std::time::{Instant};
use std::collections::{HashMap};

fn step(initial_state: &HashMap<(usize, usize), char>, (max_x,max_y): (usize,usize)) -> Option<HashMap<(usize, usize), char>> {
    let mut changed = false;

    let mut state_1 = HashMap::new();
    for x in 0..max_x+1 {
        for y in 0..max_y+1 {
            if initial_state[&(x,y)] == '>' {
                let n_pos = 
                    if x == max_x {
                        (0,y)
                    } else {
                        (x+1,y)
                    };
                if initial_state[&n_pos] == '.' {
                    changed = true;
                    state_1.insert((x,y), '.');
                    state_1.insert(n_pos, '>');
                    continue;
                }
            }
            
            if !state_1.contains_key(&(x,y)) {
                state_1.insert((x,y), initial_state[&(x,y)]);
            }
        }
    }

    let mut state_2 = HashMap::new();
    for x in 0..max_x+1 {
        for y in 0..max_y+1 {
            if state_1[&(x,y)] == 'v' {
                let n_pos = 
                    if y == max_y {
                        (x,0)
                    } else {
                        (x,y+1)
                    };
                if state_1[&n_pos] == '.' {
                    changed = true;
                    state_2.insert((x,y), '.');
                    state_2.insert(n_pos, 'v');
                    continue;
                }
            }
            
            if !state_2.contains_key(&(x,y)) {
                state_2.insert((x,y), state_1[&(x,y)]);
            }
        }
    }

    if changed {
        Some(state_2)
    }
    else {
        None
    }
}

pub fn run() {
    let start = Instant::now();
    
    let mut state: HashMap<(usize, usize), char> = read_lines("input/day25.txt")
        .enumerate()
        .flat_map(|(y,str)| str.chars().into_iter().enumerate().map(|(x, c)| ((x,y),c)).collect::<Vec<((usize,usize),char)>>())
        .collect::<HashMap<(usize,usize), char>>();

    let max_x = (&state).into_iter().map(|((x,_y),_c)| *x).max().unwrap();
    let max_y = (&state).into_iter().map(|((_x,y),_c)| *y).max().unwrap();
    
    let mut i = 1;
    while let Some(n_state) = step(&state, (max_x, max_y)) {
        state = n_state;
        i += 1;
    }
        
    println!("{} in {:?}", i, start.elapsed());
}
