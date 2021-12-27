use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn read_lines<P>(filename: P) -> impl Iterator<Item = String>
where P: AsRef<Path>, {
    let file = File::open(filename).unwrap();

    let x = BufReader::new(file)
        .lines()
        .map(Result::unwrap);

    return x;
}

pub fn neighbours_2d((x, y): (i64,i64)) -> Vec<(i64,i64)> {
    return vec![
        (x+1,y+1),
        (x+1,y),
        (x+1,y-1),
        (x,y+1),
        (x,y-1),
        (x-1,y+1),
        (x-1,y),
        (x-1,y-1),
    ];
}

pub fn neighbours_2d_bounds((x, y): (i64,i64), (max_x, max_y): (i64, i64)) -> Vec<(i64,i64)> {
    let mut neighbours: Vec<(i64,i64)> = Vec::new();

    for i in -1..1 {
        let n_x = x + i;
        
        if n_x < 0 && n_x > max_x {
            continue;
        }

        for j in -1..1 {
           if i == 0 && j == 0 {
                continue;
            }

            let n_y = y + j;

            if n_y < 0 && n_y > max_y {
                continue;
            }

            neighbours.push((n_x, n_y));
        }
    }

    return neighbours;
}

pub fn neighbours_2d_bounds_no_diag((x, y): (i64,i64), (max_x, max_y): (i64, i64)) -> Vec<(i64,i64)> {
    let mut neighbours: Vec<(i64,i64)> = Vec::new();

    for i in -1..2 {
        let n_x = x + i;
        
        if n_x < 0 || n_x > max_x {
            continue;
        }
        
        for j in -1..2 {
            if i == 0 && j == 0 {
                continue;
            }
            if i != 0 && j != 0 {
                continue;
            }
            
            let n_y = y + j;
            
            if n_y < 0 || n_y > max_y {
                continue;
            }
            // println!("{},{},{},{},{},{}",i,j,n_x,n_y,max_x,max_y);

            neighbours.push((n_x, n_y));
        }
    }

    return neighbours;
}

pub fn neighbours_3d((x,y,z): (i64, i64, i64)) -> Vec<(i64,i64,i64)> {
    let mut neighbours: Vec<(i64,i64,i64)> = Vec::new();

    for i in -1..2 {
        let n_x = x + i;
        for j in -1..2 {
            let n_y = y + j;
            for k in -1..2 {
                if i == 0 && j == 0 && k == 0 {
                    continue;
                }
                let n_z = z + k;

                neighbours.push((n_x, n_y, n_z));
            }
        }
    }

    return neighbours;
}

pub fn neighbours_3d_bounds((x,y,z): (i64, i64, i64), (max_x,max_y,max_z): (i64,i64,i64)) -> Vec<(i64,i64,i64)> {
    let mut neighbours: Vec<(i64,i64,i64)> = Vec::new();

    for i in -1..1 {
        let n_x = x + i;
        if n_x < 0 && n_x > max_x {
            continue;
        }

        for j in -1..1 {
            let n_y = y + j;
            if n_y < 0 && n_y > max_y {
                continue;
            }
            for k in -1..1 {
                if i == 0 && j == 0 && k == 0 {
                    continue;
                }
                let n_z = z + k;
                if n_z < 0 && n_z > max_z {
                    continue;
                }

                neighbours.push((n_x, n_y, n_z));
            }
        }
    }

    return neighbours;
}