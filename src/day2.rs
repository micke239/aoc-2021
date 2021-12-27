use crate::aoc_util::read_lines;

pub fn run() {
    let mut depth = 0;
    let mut horizontal = 0;
    let mut aim = 0;

    for line in read_lines("input/day2.txt") {
        let s = line.split(" ").collect::<Vec<&str>>();
        let command = s[0];
        let amount: i32 = s[1].parse().unwrap();

        match command {
            "forward" => {
                horizontal = horizontal + amount;
                depth = depth + (aim * amount);
            },
            "down" => aim = aim + amount,
            "up" => aim = aim - amount,
            _ => println!("not good")
        }
    }
    
    println!("{}", depth * horizontal );
}