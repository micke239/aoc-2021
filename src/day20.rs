use crate::aoc_util::{read_lines};
use std::time::{Instant};
use std::collections::{HashMap};
use std::fs::File;
use std::io::prelude::*;

fn get_bounds(image: &HashMap<(i64,i64), u32>) -> ((i64,i64),(i64,i64)) {
    let mut max_y = 0;
    let mut min_y = 0;
    let mut max_x = 0;
    let mut min_x = 0;
    for (x,y) in image.keys() {
        let v = image[&(*x,*y)];

        if v == 0 {
            continue;
        }

        if *y > max_y {
            max_y = y.clone();
        } else if *y < min_y {
            min_y = y.clone();
        }

        if *x > max_x {
            max_x = x.clone();
        } else if *x < min_x {
            min_x = x.clone();
        }
    }

    return ((min_x, max_x), (min_y, max_y));
}

fn neighbours((x, y): (i64,i64)) -> Vec<(i64,i64)> {
    return vec![
        (x-1,y-1),
        (x,y-1),
        (x+1,y-1),
        (x-1,y),
        (x,y),
        (x+1,y),
        (x-1,y+1),
        (x,y+1),
        (x+1,y+1)
    ];
}

fn print_image(image: &HashMap<(i64,i64), u32>) {
    let ((min_x,max_x),(min_y,max_y)) = get_bounds(image); 

    for y in (min_y)..(max_y+1) {
        for x in (min_x)..(max_x+1) {
            print!("{}", if let Some(n) = image.get(&(x,y)) { if *n == 1 {'#'} else {'.'}} else {'.'});
        }
        println!("");
    }
}

fn print_image_file(image: &HashMap<(i64,i64), u32>) {
    let ((min_x,max_x),(min_y,max_y)) = get_bounds(image); 

    let mut s = String::new();
    for y in (min_y)..(max_y+1) {
        for x in (min_x)..(max_x+1) {
            if let Some(n) = image.get(&(x,y)) { if *n == 1 {s.push('#')} else {s.push('.')}} else {s.push('.')};
        }
        s += "\n";
    }
    let mut file = File::create("foo.txt").unwrap();
    file.write_all(s.as_bytes()).unwrap();
}

fn enhance_image(input_image: &HashMap<(i64,i64), u32>, enhancement_algorithm: &Vec<u32>, overflow_white: bool) -> HashMap<(i64,i64), u32> {
    let mut image = HashMap::new();

    let ((min_x, max_x), (min_y, max_y)) = get_bounds(&input_image);

    for x in (min_x-1)..(max_x+2) {
        for y in (min_y-1)..(max_y+2) {
            let neighbours = neighbours((x,y));
            let mut counter = 8;
            let mut index:usize = 0;
            for n in neighbours {
                let v = match input_image.get(&n) {
                    Some(n) => *n as usize,
                    None => if overflow_white {1} else {0}
                };
                if v == 1 {
                    index |= v << counter;
                }
                counter-=1;
            }

            let n_v = enhancement_algorithm[index];
            image.insert((x,y), n_v);
        }
    }

    return image;
}
 
pub fn run() {
    let start = Instant::now();
    let mut lines = read_lines("input/day20.txt");

    let enhancement_algorithm = lines.next().unwrap()
        .chars()
        .map(|c| match c {
            '.' => 0,
            '#' => 1,
            _ => panic!()
        })
        .collect();
    lines.next();

    let mut image: HashMap<(i64,i64), u32> = HashMap::new();
    let mut y = 0;
    for line in lines {
        let mut x = 0;
        for c in line.chars() {
            let b = match c {
                '.' => 0,
                '#' => 1,
                _ => panic!()
            };

            image.insert((x,y), b);

            x += 1
        }
        y += 1
    }

    let mut enhanced_image = image;
    for i in 0..50 {
        enhanced_image = enhance_image(&enhanced_image, &enhancement_algorithm, i % 2 == 1);
        // if i % 2 == 1 {
        //     // print_image(&enhanced_image);
        //     let ((min_x, max_x), (min_y, max_y)) = get_bounds(&enhanced_image);
        //     enhanced_image = enhanced_image.iter()
        //     .filter(|((x,y),_v)| *x <= max_x-4 && *x >= min_x+4 && *y <= max_y-4 && *y >= min_y+4)
        //     .map(|((x,y), v)| ((*x,*y),*v) )
        //     .collect();
        // }
        // print_image(&enhanced_image);
    }
    // print_image_file(&enhanced_image);
    
    let c = enhanced_image
        .values()
        .filter(|v| **v == 1)
        .count();
    
    println!("{} in {:?}", c, start.elapsed())
}
