use crate::aoc_util::read_lines;

pub fn run() {
    let numbers = read_lines("input/day1.txt")
        .map(|x| x.parse().unwrap())
        .collect::<Vec<i32>>();
        
    day1_part1(&numbers);
    day1_part2(&numbers);
}

fn day1_part1(numbers: &Vec<i32>) {
    let mut last: Option<&i32> = None;
    let mut count = 0;

    for number in numbers {
        if last.is_some() && number > last.unwrap() {
            count = count + 1;
        }
        
        last = Some(number);
    }

    println!("{}", count)
}

fn day1_part2(numbers: &Vec<i32>) {
    let mut summed_numbers = Vec::<i32>::new();

    for n in 0..(numbers.len()-2) {
        summed_numbers.push(numbers[n] + numbers[n+1] + numbers[n+2] )
    }

    day1_part1(&summed_numbers);
}