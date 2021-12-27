use crate::aoc_util::read_lines;
use std::time::{Instant};
use std::collections::{HashSet, HashMap, VecDeque};
 
fn transform(point: (i32,i32,i32), sub: (i32,i32,i32), rots: usize, shift: usize) -> (i32,i32,i32) {
    let bvec = vec![
        (1,1,1),
        (1,1,-1),
        (1,-1,1),
        (1,-1,-1),
        (-1,1,1),
        (-1,1,-1),
        (-1,-1,1),
        (-1,-1,-1)
    ];
    let mut n_point = point;
    
    if shift == 1 {
        n_point = (point.2, point.0, point.1);
    }
    else if shift == 2 {
        n_point = (point.1, point.2, point.0);
    }
    else if shift == 3 {
        n_point = (point.0, point.2, point.1);
    }
    else if shift == 4 {
        n_point = (point.2, point.1, point.0);
    }
    else if shift == 5 {
        n_point = (point.1, point.0, point.2);
    }

    let (mut_x,mut_y,mut_z) = bvec[rots];

    n_point = (n_point.0 * mut_x, n_point.1 * mut_y, n_point.2 * mut_z);

    return (n_point.0 - sub.0, n_point.1 - sub.1, n_point.2 - sub.2);
}

pub fn run() {
    let start = Instant::now();
    let lines = read_lines("input/day19.txt");

    let bvec = vec![
        (1,1,1),
        (1,1,-1),
        (1,-1,1),
        (1,-1,-1),
        (-1,1,1),
        (-1,1,-1),
        (-1,-1,1),
        (-1,-1,-1)
    ];

    let mut scanners: Vec<HashSet<(i32,i32,i32)>> = Vec::new();
    for line in lines {
        if line.starts_with("--") {
            scanners.push(HashSet::new());
        } else if line.is_empty() {
        } else {
            let split = line.split(",").collect::<Vec<&str>>();
            let x = split[0].parse().unwrap();
            let y = split[1].parse().unwrap();
            let z = split[2].parse().unwrap();

            let idx = scanners.len()-1;
            scanners[idx].insert((x,y,z));
        }
    }

    let mut lengths: Vec<HashMap<(i32,i32,i32), HashMap<i32, (i32,i32,i32)>>> = Vec::new();
    for scanner in &scanners {
        let mut l: HashMap<(i32,i32,i32), HashMap<i32, (i32,i32,i32)>> = HashMap::new(); 
        for (b_x,b_y,b_z) in scanner {
            let mut l2: HashMap<i32, (i32,i32,i32)> = HashMap::new(); 
            for (b2_x,b2_y,b2_z) in scanner {
                if (b_x,b_y,b_z) == (b2_x,b2_y,b2_z) {
                    continue;
                }
                l2.insert((b2_x - b_x).abs() + (b2_y - b_y).abs() + (b2_z - b_z).abs(), (b2_x.clone(),b2_y.clone(),b2_z.clone()));
            }
            l.insert((b_x.clone(),b_y.clone(),b_z.clone()), l2);
        }
        lengths.push(l);
    }
    let mut queue: VecDeque<usize> = VecDeque::new();
    queue.push_back(0); 
    let mut to_zero: HashMap<usize, ((i32,i32,i32), usize, usize, usize)> = HashMap::new();
    to_zero.insert(0, ((0,0,0), 0, 0, 0));
    while let Some(i) = queue.pop_front() {
        for j in 0..lengths.len() {
            if to_zero.contains_key(&j) {
                continue;
            }
            let scanner1 = &lengths[i];
            let scanner2 = &lengths[j];
            let mut b88= false;
            for (k, b) in scanner1 {
                let l1 = b.keys().into_iter().collect::<HashSet<&i32>>();
                for (k1, b2) in scanner2 {
                    let l2 = b2.keys().into_iter().collect::<HashSet<&i32>>();
                    let inter = l1.intersection(&l2).into_iter().map(|x| x.clone().clone()).collect::<HashSet<i32>>();
                    if inter.len() >= 11 {
                        let len2 = inter.into_iter().next().unwrap();
                        let k_2 = b[&len2];
                        let k1_2 = b2[&len2];
                        
                        let mut rots = 0;
                        let mut shifts = 0;
                        let mut m = (0,0,0);
                        let mut i_j = 0;
                        loop {
                            let (x1, y1, z1) = k;
                            let (mut x2, mut y2, mut z2) = k1;
                            let (x3, y3, z3) = k_2;
                            let (mut x4, mut y4, mut z4) = k1_2;
                            
                            if shifts == 1 {
                                x2 = k1.2;
                                y2 = k1.0;
                                z2 = k1.1;

                                x4 = k1_2.2;
                                y4 = k1_2.0;
                                z4 = k1_2.1;
                            }
                            else if shifts == 2 {
                                x2 = k1.1;
                                y2 = k1.2;
                                z2 = k1.0;

                                x4 = k1_2.1;
                                y4 = k1_2.2;
                                z4 = k1_2.0;
                            }
                            else if shifts == 3 {
                                x2 = k1.0;
                                y2 = k1.2;
                                z2 = k1.1;

                                x4 = k1_2.0;
                                y4 = k1_2.2;
                                z4 = k1_2.1;
                            }
                            else if shifts == 4 {
                                x2 = k1.2;
                                y2 = k1.1;
                                z2 = k1.0;

                                x4 = k1_2.2;
                                y4 = k1_2.1;
                                z4 = k1_2.0;
                            }
                            else if shifts == 5 {
                                x2 = k1.1;
                                y2 = k1.0;
                                z2 = k1.2;

                                x4 = k1_2.1;
                                y4 = k1_2.0;
                                z4 = k1_2.2;
                            }

                            let (mut_x,mut_y,mut_z) = bvec[rots];

                            x2 = x2 * mut_x;
                            y2 = y2 * mut_y;
                            z2 = z2 * mut_z;
    
                            x4 = x4 * mut_x;
                            y4 = y4 * mut_y;
                            z4 = z4 * mut_z;

                            let d_x = x2 - x1;
                            let d_y = y2 - y1;
                            let d_z = z2 - z1;
    
                            if x4 - d_x == x3 && y4 - d_y == y3 && z4 - d_z == z3 {
                                m = (d_x,d_y,d_z);
                                break;
                            }

                            rots += 1;

                            if rots == bvec.len() {
                                rots = 0;
                                shifts += 1;
                            }
                            i_j += 1;
                            if shifts == 6 {
                                println!("Fail {}", i_j);
                                break;
                            }
                        }

                        to_zero.insert(j, (m, rots, shifts, i));
                        queue.push_back(j);
                        b88 = true;
                        break;
                    }
                }
                if b88 {
                    break;
                }
            }
        }
    }

    let mut points : HashSet<(i32,i32,i32)> = HashSet::new();
    for i in 0..scanners.len() {
        for beacon in &(scanners)[i] {
            if i == 0 {
                points.insert(beacon.clone());
            }
            let mut blehe = Some(i);
            let mut t_beacon = beacon.clone();
            while let Some(con) = blehe {
                let (add, rot, shift, next) = to_zero[&con];

                t_beacon = transform(t_beacon, add, rot, shift);
                
                if next != 0 {
                    blehe = Some(next);
                } else {
                    points.insert(t_beacon);
                    blehe = None;
                }
            }
        }
    }
    
    let mut scanner_pos: Vec<(i32,i32,i32)> = Vec::new();
    scanner_pos.push((0,0,0));
    for i in 1..scanners.len() {
        let mut blehe = Some(i);
        let mut t_beacon = (0,0,0);
        while let Some(con) = blehe {
            let (add, rot, shift, next) = to_zero[&con];

            t_beacon = transform(t_beacon, add, rot, shift);
            
            if next != 0 {
                blehe = Some(next);
            } else {
                scanner_pos.push(t_beacon);
                blehe = None;
            }
        }
    }

    let mut max = 0;
    for i in 0..scanner_pos.len() {
        for j in i..scanner_pos.len() {
            let s1 = scanner_pos[i];
            let s2 = scanner_pos[j];

            let dist = (s2.0 - s1.0).abs() + (s2.1 - s1.1).abs() + (s2.2 - s1.2).abs();

            if dist > max {
                max = dist;
            }
        }
    }
    
    println!("{} {} in {:?}", max, points.len(), start.elapsed())
}
