use crate::aoc_util::{read_lines};
use std::time::{Instant};

fn except(r1: (i64,i64),r2:(i64,i64)) -> Vec<(i64,i64)> {
    if r1.0 > r2.1 || r2.0 > r1.1 {
        return vec![r1];
    }
    
    let mut v = Vec::new();
    if r2.0 > r1.0 {
        v.push((r1.0, r2.0-1));
    }

    if r2.1 < r1.1 {
        v.push((r2.1+1, r1.1));
    }
    
    return v;
}

pub fn run() {
    let start = Instant::now();
    let lines = read_lines("input/day22.txt");
    let mut add: Vec<((i64, i64), (i64, i64), (i64, i64))> = Vec::new();

    for line in lines {
        let split = line.split(",").collect::<Vec<&str>>();
        let x_split = split[0].split("..").collect::<Vec<&str>>();
        let y_split = split[1].split("..").collect::<Vec<&str>>();
        let z_split = split[2].split("..").collect::<Vec<&str>>();

        let on = line.starts_with("on");

        let x_from: i64 = if on { &x_split[0][5..] } else { &x_split[0][6..] }.parse().unwrap();
        let x_to: i64 = x_split[1].parse().unwrap();

        let y_from: i64 = y_split[0][2..].parse().unwrap();
        let y_to: i64 = y_split[1].parse().unwrap();

        let z_from: i64 = z_split[0][2..].parse().unwrap();
        let z_to: i64 = z_split[1].parse().unwrap();

        let mut n_add = Vec::new();
        for (c_x,c_y,c_z) in &add {
            let mut x_is = Vec::new();
            for x_i in except(*c_x, (x_from, x_to)) {
                n_add.push((x_i,*c_y,*c_z));
                x_is.push(x_i);
            }
            
            let xx = x_is.iter().fold(Some(*c_x), |acc, y_i| if acc.is_some() { except(acc.unwrap(), *y_i).get(0).map(|x| *x) } else { None });
            if let Some(xxx) = xx {
                let mut y_is = Vec::new();
                for y_i in except(*c_y, (y_from, y_to)) {
                    n_add.push((xxx,y_i,*c_z));
                    y_is.push(y_i);
                }
    
                let yy = y_is.iter().fold(Some(*c_y), |acc, y_i| if acc.is_some() { except(acc.unwrap(), *y_i).get(0).map(|x| *x) } else { None });
                if let Some(yyy) = yy {
                    for z_i in except(*c_z, (z_from, z_to)) {
                        n_add.push((xxx,yyy,z_i));
                    }
                }
            }
        }
        
        if on {
            n_add.push(((x_from, x_to),(y_from, y_to),(z_from, z_to)));
        }

        add = n_add;
    }
    
    let add_i: i64 = (&add).into_iter().map(|(xr,yr,zr)| ((xr.1-xr.0).abs()+1)*((yr.1-yr.0).abs()+1)*((zr.1-zr.0).abs()+1)).sum();
    println!("{} in {:?}", add_i, start.elapsed())
}
