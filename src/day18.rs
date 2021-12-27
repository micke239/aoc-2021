use crate::aoc_util::read_lines;
use std::time::{Instant};

struct Node {
    v1: Option<i32>,
    v2: Option<i32>,
    n1: Option<i32>,
    n2: Option<i32>

}

pub fn reduce(vec: &mut Vec<(i32,i32)>) {
    if vec.is_empty() {
        return;
    }
    for i in 0..(vec.len()-1) {
        let (n1, d1 ) = vec[i];
        let (n2, d2)= vec[i+1];
        let is_number_pair = d1 == d2;

        if !is_number_pair {
            continue;
        }

        if d1 > 4 {

            // println!("explode {} {}", n1, d1);
            if i > 0 {
                let (n3, d3) = vec[i-1];
                vec[i-1] = (n3 + n1, d3);
            }
            if i < vec.len()-2 {
                let (n3, d3) = vec[i+2];
                vec[i+2] = (n3 + n2, d3);
            }
    
            vec[i] = (0, d1-1);
            vec.remove(i+1);

            reduce(vec);

            break;
        }
    }
    
    for i in 0..vec.len() {
        let (n, d ) = vec[i];
        
        if n > 9 {
            let f1 = n as f64 / 2.; 
            let n1 = f1.floor() as i32;
            let n2 = f1.ceil() as i32;
            // println!("split {} {}", n, d);

            vec[i] = (n1, d+1);
            vec.insert(i+1, (n2, d+1));

            reduce(vec);

            break;
        }
    }
}

pub fn pri(vec: &Vec<(i32,i32)>) {
    let mut curr_depth: i32 = -1;
    let mut print_depth = 0;
    for (n, d) in vec {
        while curr_depth < *d {
            print!("[");
            curr_depth+=1;
        }
        while curr_depth > *d {
            print!("]");
            curr_depth-=1;
        }
        
        if print_depth == 0 {
            print!("{}", n);
        }
        else {
            print!(",{}", n);
        }
        if print_depth == 1 {
            print!("]");
            curr_depth -= 1;
            print_depth = 0;
        }
        else {
            print_depth += 1;
        }
    }
    while curr_depth > 0 {
        print!("]");
        curr_depth-=1;
    }
    println!("");
}

pub fn mag(nums: Vec<String>) -> i32 {
    let lines = nums.into_iter().map(|l| {
        let mut depth: i32 = 0;
        let mut vec: Vec<(i32,i32)> = Vec::new();
        for c in l.chars() {
            if c == '[' {
                depth += 1;
            }
            else if c == ']' {
                depth -= 1;
            }
            else if c == ',' {
                continue;        
            }
            else {
                let d = c.to_digit(10);
                if d.is_none() {
                    println!("{}", c)
                }
                vec.push((d.unwrap() as i32, depth));
            }
        }
        vec
    })
    .fold(Vec::<(i32,i32)>::new(), |acc, line| {
        let mut n_acc = Vec::<(i32,i32)>::new();
        for (n, d) in &acc {
            n_acc.push((*n,d+1));
        }

        for (n, d) in line {
            if acc.is_empty() {
                n_acc.push((n, d));
            } else {
                n_acc.push((n, d + 1));
            }
        }

        reduce(&mut n_acc);

        //  pri(&n_acc);

        n_acc
    });


    let mut m_d = (&lines).iter().map(|(_n,d)| d).max();
    let mut n_l: Vec<(i32,i32)> = (&lines).into_iter().map(|x| *x).collect();
    while let Some(m) = m_d {
        if *m == 0 {
            break;
        }
        let mut i_n_l: Vec<(i32,i32)> = Vec::new();
        let mut i = 0;
        while i < n_l.len() - 1 {
            let (n1, d1 ) = n_l[i];
            let (n2, d2)= n_l[i+1];

            let is_number_pair = d1 == d2;

            if !is_number_pair || d1 != *m {
                i_n_l.push(n_l[i]);
                i+=1;

                continue;
            }

            if d1 == *m {
                i_n_l.push((3*n1 + 2* n2, d1 - 1));
                i += 2;
            }
        }

        if i == n_l.len() - 1 {
            i_n_l.push(n_l[i]);
        }

        n_l = i_n_l;
        m_d = (&n_l).iter().map(|(_n,d)| d).max();
    }

    let sum = (&n_l).iter().map(|(n,_d)| n).sum::<i32>();

    return sum;
}
 
pub fn run() {
    let start = Instant::now();
    let lines = read_lines("input/day18.txt").collect::<Vec<String>>();
    let mut max = 0;
    let c = lines.len() * lines.len();
    let mut i_c = 0;
    for l1 in &lines {
        for l2 in &lines {
            if l1 == l2 || i_c % 101 == 0 {
                i_c += 1;
                println!("continue");
                continue;   
            }
           let mag = mag(vec![String::from(l1), String::from(l2)]);
           if mag > max {
               max = mag;
               println!("New max: {}", max)
           }
           i_c += 1;
           println!("{}/{}", i_c, c);
        }
    } 
    
    println!("{} in {:?}", max, start.elapsed())
}
