use crate::aoc_util::read_lines;
use std::time::{Instant};
use std::collections::HashSet;

pub fn shoot ((i_vel_x, i_vel_y): (i32, i32), min_x: i32, max_x: i32, min_y: i32, max_y: i32) -> (bool, i32) {
    let mut pos_x = 0;
    let mut pos_y = 0; 

    let mut vel_x = i_vel_x;
    let mut vel_y = i_vel_y;

    let mut max_height_reached = 0;
    let success: bool;

    loop {
        pos_x = pos_x + vel_x;
        pos_y = pos_y + vel_y;

        if max_height_reached < pos_y {
            max_height_reached = pos_y;
        }

        if pos_x > max_x || pos_y < min_y { // naiv
            success = false;
            break;
        }

        if pos_x >= min_x && pos_x <= max_x && pos_y >= min_y && pos_y <= max_y {
            success = true;
            break;
        } 

        if vel_x < 0 {
            vel_x = vel_x + 1;
        }
        else if vel_x > 0 {
            vel_x = vel_x - 1;
        }

        vel_y = vel_y - 1;
    }

    return (success, max_height_reached);
}

pub fn run() {
    let start = Instant::now();
    let mut lines = read_lines("input/day17.txt");
    let line = lines.next().unwrap();
    let split = line.split("..").collect::<Vec<&str>>();
    let split2 = split[1].split(", ").collect::<Vec<&str>>();

    let min_x: i32 = split[0].replace("target area: x=", "").parse().unwrap();
    let max_x: i32 = split2[0].parse().unwrap();
    let min_y: i32 = split2[1].replace("y=", "").parse().unwrap();
    let max_y: i32 = split[2].parse().unwrap();

    let mut set: HashSet<(i32,i32)> = HashSet::new();
    for i in -1000..1000 {
        for j in -1000..1000 {
            let (success, _max_height_reached) = shoot((i,j), min_x,max_x,min_y,max_y);

            if success {
                set.insert((i,j));
            }
        }
    }
    

    println!("{} in {:?}", set.len(), start.elapsed())
}
