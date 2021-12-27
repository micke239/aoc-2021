use crate::aoc_util::{read_lines};
use std::time::{Instant};

struct Game {
    player1: (usize,usize),
    player2: (usize,usize),
    turn: usize,
    throw: usize,
    value: u64
}

pub fn run() {
    let start = Instant::now();
    let lines = read_lines("input/day21.txt");
    let mut players = Vec::new();

    for line in lines {
        let sp = usize::from_str_radix(&line[line.len()-1..], 10).unwrap();
        players.push((sp, 0))
    }

    let mut wins = [0 as u64,0 as u64];

    let mut games: Vec<Game> = Vec::new();

    let mut throw_count = [0; 10];
    for i in 1..4 {
        for j in 1..4 {
            for k in 1..4 {
                throw_count[i+j+k] += 1;
            }
        }
    }

    for i in 3..10 {
        games.push(
            Game {
                player1: players[0],
                player2: players[1],
                turn: 0,
                throw: i,
                value: throw_count[i]
            }
        );
    }

    while let Some(game) = games.pop() {
        
        let turn = game.turn;
        let player = match turn {
            0 => game.player1,
            1 => game.player2,
            _ => panic!()
        };

        let mut pos = player.0 + game.throw;
        
        if pos > 10 {
            pos = pos % 10;
        }

        let score = player.1 + pos;

        if score > 20 {
            wins[turn] += game.value;
            continue;
        } 

        let (player1, player2, turn) = match turn {
            0 => ((pos, score), game.player2, 1),
            1 => (game.player1, (pos, score), 0),
            _ => panic!()
        };

        for i in 3..10 {
            games.push(
                Game {
                    player1,
                    player2,
                    turn,
                    throw: i,
                    value: game.value * throw_count[i]
                }
            );
        }
    }  

    println!("{}, {} in {:?}",wins[0], wins[1], start.elapsed())
}
