use crate::aoc_util::read_lines;
use std::collections::HashSet;

fn size_basin(lines: &Vec<Vec<u32>>, (i,j,n): (usize, usize, u32), counted: &mut HashSet<(usize,usize)>) -> u64 {
    if counted.contains(&(i,j)) {
        return 0;
    } else {
        counted.insert((i,j));
    }
    
    let mut size = 1;
    if i != (lines.len() - 1) {
        let n_i = i+1;
        let n_j = j;
        let n_n = lines[n_i][n_j];
        if n_n > n && n_n != 9 {
            size += size_basin(lines, (n_i, n_j, n_n), counted)
        }
    } 

    if i != 0 {
        let n_i = i-1;
        let n_j = j;
        let n_n = lines[n_i][n_j];
        if n_n > n && n_n != 9 {
            size += size_basin(lines, (n_i, n_j, n_n), counted)
        }
    }

    if j != (&lines[i].len() - 1) {
        let n_i = i;
        let n_j = j+1;
        let n_n = lines[n_i][n_j];
        if n_n > n && n_n != 9 {
            size += size_basin(lines, (n_i, n_j, n_n), counted)
        }
    } 

    if j != 0 {
        let n_i = i;
        let n_j = j-1;
        let n_n = lines[n_i][n_j];
        if n_n > n && n_n != 9 {
            size += size_basin(lines, (n_i, n_j, n_n), counted)
        }
    }

    return size;
}

fn part1(lines: &Vec<Vec<u32>>) {
    let mut low_points: Vec<(usize, usize, u32)> = Vec::new();
    for i in 0..lines.len() {
        let inner_lines = &lines[i];
        for j in 0..inner_lines.len() {
            let n = lines[i][j];

            if i != (lines.len() - 1) && n >= lines[i+1][j] {
                continue;
            } 

            if i != 0 && n >= lines[i-1][j] {
                continue;
            }

            if j != (inner_lines.len() - 1) && n >= lines[i][j+1] {
                continue;
            } 

            if j != 0 && n >= lines[i][j-1] {
                continue;
            }
            
            low_points.push((i,j,n))
        }
    }
    
    let risk_level: u64 = (&low_points).into_iter().map(|(_x,_y,z)| (z + 1) as u64).sum();
    println!("{}", risk_level);

    let mut basins: Vec<u64> = (&low_points).into_iter()
        .map(|(i,j,n)| {
            let size = size_basin(&lines, (*i, *j, *n), &mut HashSet::new());
            println!("{}", size);
            return size;
        })
        .collect();

    basins.sort();

    println!("{}, {}, {}", basins[basins.len()-1], basins[basins.len()-2], basins[basins.len()-3]);
    println!("{}", basins[basins.len()-1] * basins[basins.len()-2] * basins[basins.len()-3]);
}

pub fn run() {
    let lines: Vec<Vec<u32>> = read_lines("input/day9.txt")
        .map(|s| s.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    part1(&lines);
    
}