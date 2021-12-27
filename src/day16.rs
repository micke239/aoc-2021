use crate::aoc_util::read_lines;
use std::time::{Instant};
use std::cmp;

fn aggregate(vals: Vec<i64>, t: i32) -> i64 {
    match t {
        0 => vals.into_iter().sum::<i64>(),
        1 => vals.into_iter().fold(1 as i64, |agg, val| agg * val),
        2 => vals.into_iter().reduce(|v1, v2| cmp::min(v1, v2)).unwrap(),
        3 => vals.into_iter().reduce(|v1, v2| cmp::max(v1, v2)).unwrap(),
        5 => if vals[0] > vals[1] { 1 } else { 0 },
        6 => if vals[0] < vals[1] { 1 } else { 0 },
        7 => if vals[0] == vals[1] { 1 } else { 0 },
        _ => panic!()
    }
}

fn parse_one_bit_packet_content(packet_content: &str, t: i32) -> (i32, usize, i64) {
    let packets = usize::from_str_radix(&packet_content[0..11], 2).unwrap();
    let mut v_sum: i32 = 0;
    let mut last_index: usize = 11;
    let mut vals: Vec<i64> = Vec::new();
    for _i in 0..packets {
        let (i_v_sum, i_last_index, v) = parse_packet(&packet_content[last_index..]);
        vals.push(v);
        v_sum += i_v_sum;
        last_index += i_last_index;
    }

    return (v_sum, last_index, aggregate(vals, t));
}

fn parse_zero_bit_packet_content(packet_content: &str, t: i32) -> (i32, usize, i64) {
    let length = usize::from_str_radix(&packet_content[0..15], 2).unwrap();
    let mut i = 15;
    let mut v_sum: i32 = 0;
    let mut vals: Vec<i64> = Vec::new();
    while i < packet_content.len() && i - 15 < length {
        let (i_v_sum, i_last_index, v) = parse_packet(&packet_content[i..]);
        vals.push(v);
        v_sum += i_v_sum;
        i += i_last_index;

    }
    (v_sum, i, aggregate(vals, t))
}

fn parse_literal_packet_content(packet_content: &str) -> (i32, usize, i64) {
    let mut i = 5;
    let mut value = String::new();
    while i < packet_content.len() {
        let sub = &packet_content[(i-5)..i];

        value.push_str(&sub[1..]);

        if &sub[0..1] == "0" {
            break;
        }
        i += 5;
    }

    return (0, i, i64::from_str_radix(value.as_str(), 2).unwrap());
}

fn parse_packet(packet: &str) -> (i32, usize, i64) {
    if packet.len() < 11 {
        return (0, packet.len(), 0)
    }
    
    let v = i32::from_str_radix(&packet[0..3], 2).unwrap();
    let t = i32::from_str_radix(&packet[3..6], 2).unwrap();
    
    let (i_v_sum, i_last_index, val) = match t {
        4 => parse_literal_packet_content(&packet[6..]),
        _ => {
            let i = &packet[6..7];
            if i == "0" {
                parse_zero_bit_packet_content(&packet[7..], t)
            }
            else {
                parse_one_bit_packet_content(&packet[7..], t)
            }
        }
    };

    return (i_v_sum + v, i_last_index + (if t == 4 { 6 } else { 7 }), val);
}

pub fn run() {
    let start = Instant::now();
    let mut lines = read_lines("input/day16.txt");

    let mut hex: String = String::new();

    for char in lines.next().unwrap().chars() {
        let s = match char {
            '0' => "0000",
            '1' => "0001",
            '2' => "0010",
            '3' => "0011",
            '4' => "0100",
            '5' => "0101",
            '6' => "0110",
            '7' => "0111",
            '8' => "1000",
            '9' => "1001",
            'A' => "1010",
            'B' => "1011",
            'C' => "1100",
            'D' => "1101",
            'E' => "1110",
            'F' => "1111",
             _ => panic!() 
        };

        hex.push_str(s);
    }

    let (v, i, val) = parse_packet(hex.as_str());
    
    println!("{},{},{} in {:?}", v,i,val, start.elapsed());
}
