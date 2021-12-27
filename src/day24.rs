// use crate::aoc_util::{read_lines};
use std::time::{Instant};
use std::collections::{HashMap};

fn read_operand(mem: &HashMap<String, i64>, operand: &String) -> i64 {
    if let Some(v) = mem.get(operand) {
        return *v;
    }

    if let Ok(v) = i64::from_str_radix(operand, 10) {
        return v;
    }

    return 0;
}

pub fn run_instructions(instructions: &Vec<(String, Vec<String>)>, input: &mut Vec<i64>) -> HashMap<String, i64> {

    let mut mem = HashMap::new();

    
    for (instruction, operands) in instructions {
        match instruction.as_str() {
            "inp" => mem.insert(operands[0].clone(), input.pop().unwrap()),
            "add" => mem.insert(operands[0].clone(), read_operand(&mem, &operands[0]) + read_operand(&mem, &operands[1])),
            "mul" => mem.insert(operands[0].clone(), read_operand(&mem, &operands[0]) * read_operand(&mem, &operands[1])),
            "div" => mem.insert(operands[0].clone(), read_operand(&mem, &operands[0]) / read_operand(&mem, &operands[1])),
            "mod" => mem.insert(operands[0].clone(), read_operand(&mem, &operands[0]) % read_operand(&mem, &operands[1])),
            "eql" => mem.insert(operands[0].clone(), if read_operand(&mem, &operands[0]) == read_operand(&mem, &operands[1]) { 1 } else { 0 }),
            _ => panic!("Instruction fail")
        };
        // println!("{} {:15}-> {:?}",instruction, format!("{:?}",operands), mem);
    }

    return mem;
}

fn first(w: i32) -> i32 {
    w + 7
}

fn second(z: i32, w: i32) -> i32 {
    26*z + w + 8
}

fn third(z: i32, w: i32) -> i32 {
    26 * z + w + 10
}

fn fourth(z: i32, w: i32) -> Option<i32> {
    if (z % 26) - 2 != w {
        return None;
    }

    Some(z / 26)
}

fn fifth(z: i32, w: i32) -> Option<i32> {
    if (z % 26) - 10 != w {
        return None;
    }

    Some(z / 26)
}

fn sixth(z: i32, w: i32) -> i32 {
    26 * z + w + 6
}

fn seventh(z: i32, w: i32) -> Option<i32> {
    if (z % 26) - 14 != w {
        return None;
    }

    Some(z / 26)
}

fn eigth(z: i32, w: i32) -> Option<i32> {
    if (z % 26) - 5 != w {
        return None;
    }

    Some(z / 26)
}

fn ninth(z: i32, w: i32) -> i32 {
    26 * z + w + 1
}

fn tenth(z: i32, w: i32) -> i32 {
    26 * z + w + 8
}

fn eleventh(z: i32, w: i32) -> Option<i32> {
    if (z % 26) - 14 != w {
        return None;
    }

    Some(z / 26)
}

fn twelfth(z: i32, w: i32) -> i32 {
    26 * z + w + 13
}

fn thirteenth(z: i32, w: i32) -> Option<i32> {
    if (z % 26) - 14 != w {
        return None;
    }

    Some(z / 26)
}

fn fourteenth(z: i32, w: i32) -> Option<i32> {
    if (z % 26) - 5 != w {
        return None;
    }

    Some(z / 26)
}

pub fn run() {
    let start = Instant::now();
    // let instructions = read_lines("input/day24.txt")
    //     .map(|str| {
    //         let mut split = str.split(" ").map(|x| String::from(x));
    //         (split.next().unwrap(), split.collect::<Vec<String>>())
    //     })
    //     .collect::<Vec<(String, Vec<String>)>>();

        
    for i1 in [9,8,7,6,5,4,3,2,1] {
        let z1 = first(i1);
        for i2 in [9,8,7,6,5,4,3,2,1] {
            let z2 = second(z1, i2);
            for i3 in [9,8,7,6,5,4,3,2,1] {
                let z3 = third(z2, i3);
                for i4 in [9,8,7,6,5,4,3,2,1] {
                    let z4 = fourth(z3, i4);
                    if z4.is_none() {
                        continue;
                    }
                    for i5 in [9,8,7,6,5,4,3,2,1] {
                        let z5 = fifth(z4.unwrap(), i5);
                        if z5.is_none() {
                            continue;
                        }
                        for i6 in [9,8,7,6,5,4,3,2,1] {
                            let z6 = sixth(z5.unwrap(), i6);
                            for i7 in [9,8,7,6,5,4,3,2,1] {
                                let z7 = seventh(z6, i7);
                                if z7.is_none() {
                                    continue;
                                }
                                for i8 in [9,8,7,6,5,4,3,2,1] {
                                    let z8 = eigth(z7.unwrap(), i8);
                                    if z8.is_none() {
                                        continue;
                                    }
                                    for i9 in [9,8,7,6,5,4,3,2,1] {
                                        let z9 = ninth(z8.unwrap(), i9);
                                        for i10 in [9,8,7,6,5,4,3,2,1] {
                                            let z10 = tenth(z9, i10);
                                            for i11 in [9,8,7,6,5,4,3,2,1] {
                                                let z11 = eleventh(z10, i11);
                                                if z11.is_none() {
                                                    continue;
                                                }
                                                for i12 in [9,8,7,6,5,4,3,2,1] {
                                                    let z12 = twelfth(z11.unwrap(), i12);
                                                    for i13 in [9,8,7,6,5,4,3,2,1] {
                                                        let z13 = thirteenth(z12, i13);
                                                        if z13.is_none() {
                                                            continue;
                                                        }
                                                        for i14 in [9,8,7,6,5,4,3,2,1] {
                                                            let z14 = fourteenth(z13.unwrap(), i14);
                                                            if z14.is_none() {
                                                                continue;
                                                            }
                                                            if z14.unwrap() == 0 {
                                                                println!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}",i1,i2,i3,i4,i5,i6,i7,i8,i9,i10,i11,i12,i13,i14);
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
        
    println!(" in {:?}", start.elapsed());
}
