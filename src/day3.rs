use std::isize;

use crate::aoc_util::read_lines;

struct Count {
    one_count: i32,
    zero_count: i32
}

fn get_counts<T>(lines : &[T]) -> Vec<Count>
    where T : AsRef<str>
{
    let mut x = Vec::<Count>::new();
    for line in lines {
        for (index, c) in line.as_ref().chars().enumerate() {
            if x.len() < (index + 1) {
                x.push(Count {
                    one_count: 0,
                    zero_count: 0
                });
            }

            let mut count = x.get_mut(index).unwrap();

            match c {
                '0' => count.zero_count += 1,
                '1' => count.one_count += 1,
                _ => println!("hello")
            }
        }
    }
    x
}

pub fn run() {
    
    let lines = read_lines("input/day3.txt").collect::<Vec<String>>();
    let counts = get_counts(&lines);
    
    let mut gamma_rate_b = String::from("");
    let mut epsilon_rate_b = String::from("");

    for count in &counts {
        if count.zero_count > count.one_count {
            gamma_rate_b.push_str("0");
            epsilon_rate_b.push_str("1");
        } else {
            gamma_rate_b.push_str("1");
            epsilon_rate_b.push_str("0");
        }
    }

    let gamma_rate = isize::from_str_radix(&gamma_rate_b, 2).unwrap();
    let epsilon_rate = isize::from_str_radix(&epsilon_rate_b, 2).unwrap();

    let mut index = 0;
    let mut possible_oxygens = (&lines).into_iter().collect::<Vec<&String>>();
    while possible_oxygens.len() != 1 {
        let ncounts = get_counts(&possible_oxygens);
        if ncounts[index].one_count >= ncounts[index].zero_count {
            possible_oxygens = possible_oxygens.into_iter()
                .filter(|l| &l[index..(index+1)] == "1")
                .collect::<Vec<&String>>();
        } else {
            possible_oxygens = possible_oxygens.into_iter()
            .filter(|l| &l[index..(index+1)] == "0")
            .collect::<Vec<&String>>();
        }
        index += 1;
    }

    index = 0;
    let mut possible_co2 = (&lines).into_iter().collect::<Vec<&String>>();
    while possible_co2.len() != 1 {
        let ncounts = get_counts(&possible_co2);
        if ncounts[index].one_count < ncounts[index].zero_count {
            possible_co2 = possible_co2.into_iter()
                .filter(|l| &l[index..(index+1)] == "1")
                .collect::<Vec<&String>>();
        } else {
            possible_co2 = possible_co2.into_iter()
            .filter(|l| &l[index..(index+1)] == "0")
            .collect::<Vec<&String>>();
        }
        index += 1;
    }

    let oxygen_generator_rating = isize::from_str_radix(&possible_oxygens[0], 2).unwrap();
    let co2_generator_rating = isize::from_str_radix(&possible_co2[0], 2).unwrap();

    println!("{}, {}, {}, {}", gamma_rate, epsilon_rate, (gamma_rate * epsilon_rate), (oxygen_generator_rating * co2_generator_rating))
    
}
