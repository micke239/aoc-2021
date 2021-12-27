use crate::aoc_util::read_lines;

pub fn run() {
    
    let lines = read_lines("input/day7.txt").collect::<Vec<String>>();
    let fishy: Vec<i64> = lines[0].split(",").map(|s| s.parse().unwrap()).collect();

    let mut last_y: i64 = 100000000000;
    let mut i = 0;
    loop {
        let mut fuel_sum: i64 = 0;
        for j in &fishy {
            let n = (j - i).abs();

            fuel_sum += n*(n+1)/2;
        }
        if fuel_sum < last_y {
            last_y = fuel_sum;
            i = i+1;
        } 
        else {
            break;
        }

    }

    println!("{}, {}", i - 1, last_y)
}