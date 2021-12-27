use std::time::{Instant};
use std::collections::HashMap;

fn is_done(room: &[char;4], expected_char: char) -> bool {
    for c in room {
        if *c != expected_char {
            return false;
        }
    }

    return true;
}

fn solved (rooms:&[[char;4];4]) -> bool {
    is_done(&rooms[0], 'A') &&
    is_done(&rooms[1], 'B') &&
    is_done(&rooms[2], 'C') &&
    is_done(&rooms[3], 'D')
}

fn get_room_target(i: usize) -> char {
    match i {
        0 => 'A',
        1 => 'B',
        2 => 'C',
        3 => 'D',
        _ => panic!("no room target for {}", i)
    }
}

fn get_weight(i: char) -> u64 {
    match i {
        'A' => 1,
        'B' => 10,
        'C' => 100,
        'D' => 1000,
        _ => panic!("no weight for {}",i)
    }
}

fn get_target_room(i: char) -> usize {
    match i {
        'A' => 0,
        'B' => 1,
        'C' => 2,
        'D' => 3,
        _ => panic!("no target room for {}", i)
    }
}

fn get_room_hallway(room: usize) -> usize {
    match room {
        0 => 2,
        1 => 4,
        2 => 6,
        3 => 8,
        _ => panic!("no room hallway for {}", room)
    }
}

fn to_room( hallway: &[char;11], hallway_pos: usize, room: usize) -> Option<u64> {
    let target_hallway = get_room_hallway(room);

    let mut count = 0;
    if hallway_pos < target_hallway {
        for pos in hallway_pos+1..target_hallway+1 {
            if hallway[pos] != '.' {
                return None;
            }
            count+=1;
        }
    }
    else {
        for pos in target_hallway..hallway_pos {
            if hallway[pos] != '.' {
                return None;
            }
            count+=1;
        }
    }

    return Some(count);
}

fn unblocked_hallways(room: usize, hallway: &[char;11]) -> Vec<(usize, u64)> {
    let mut v = Vec::new();
    for hallway_pos in [0,1,3,5,7,9,10] {
        if hallway[hallway_pos] != '.' {
            continue;
        }

        if let Some(w) = to_room(hallway, hallway_pos, room) {
            v.push((hallway_pos,w));
        }
    }
    return v;
}

fn print(rooms: &[[char;4];4], hallway: &[char;11]) {
    println!("#############");
    print!("#");
    for hallway_p in hallway {
        print!("{}", hallway_p);
    }
    println!("#");
    println!("###{}#{}#{}#{}###", rooms[0][0],rooms[1][0],rooms[2][0],rooms[3][0]);
    println!("  #{}#{}#{}#{}#  ", rooms[0][1],rooms[1][1],rooms[2][1],rooms[3][1]);
    println!("  #{}#{}#{}#{}#  ", rooms[0][2],rooms[1][2],rooms[2][2],rooms[3][2]);
    println!("  #{}#{}#{}#{}#  ", rooms[0][3],rooms[1][3],rooms[2][3],rooms[3][3]);
    println!("  #########  ");
    println!("");
}

fn get_possible_moves (rooms: &[[char;4];4], hallway: &[char;11]) -> Vec<([[char;4];4], [char;11], u64)> {
    let mut moves = Vec::new();

    for i in 0..hallway.len() {
        let c = hallway[i];
        if c == '.' {
            continue;
        }
        
        let target_room = get_target_room(c);
        let mut target_room_pos = None;
        for j in [3,2,1,0] {
            if rooms[target_room][j] == '.' {
                target_room_pos = Some(j);
                break;
            } else if rooms[target_room][j] != c {
                break;
            } 
        };

        if target_room_pos.is_none() {
            continue;
        }

        if let Some(move_count) = to_room(hallway, i, target_room) {
            let mut roomcopy = rooms.clone();
            let mut hallwaycopy = hallway.clone();
            let target_room_pos = target_room_pos.unwrap();
            roomcopy[target_room][target_room_pos] = c;
            hallwaycopy[i] = '.';

            moves.push((roomcopy, hallwaycopy, (move_count + (target_room_pos as u64) + 1) * get_weight(c)));
        }
    }

    for i in 0..rooms.len() {
        let target = get_room_target(i);

        let mut is_done = true;
        for c in rooms[i] {
            if c != target && c != '.' {
                is_done = false;
            }
        }
        if is_done {
            continue;
        }

        let mut from_room_pos = None;
        for j in [0,1,2,3] {
            if rooms[i][j] == '.' {
                continue;
            } else {
                from_room_pos = Some(j);
                break;
            } 
        };

        if let Some(from_room_pos) = from_room_pos {
            let c = rooms[i][from_room_pos];
            for (hallway_pos, move_count) in unblocked_hallways(i, hallway) {
                let mut roomcopy = rooms.clone();
                let mut hallwaycopy = hallway.clone();
                roomcopy[i][from_room_pos] = '.';
                hallwaycopy[hallway_pos] = c;
    
                moves.push((roomcopy, hallwaycopy, (move_count + (from_room_pos as u64) + 1) * get_weight(c)));
            }
        }
    }

    return moves;
}

fn solve (rooms: [[char;4];4], hallway: [char;11], weight: u64, cache: &mut HashMap<([[char;4];4],[char;11],u64), Option<u64>>) -> Option<u64> {
    let cache_key = (rooms, hallway, weight);

    // print(&rooms, &hallway);

    if solved(&rooms) {
        return Some(weight);
    }
    else if cache.contains_key(&cache_key) {
        return cache[&cache_key];
    }

    let possible_moves = get_possible_moves(&rooms, &hallway);

    if possible_moves.is_empty() {
        // print(&rooms, &hallway);
        return None;
    }

    let best = possible_moves
        .iter()
        .filter_map(|mv| solve(mv.0,mv.1,mv.2+weight, cache))
        .min();

    cache.insert(cache_key, best);

    return best;
}

pub fn run() {
    let start = Instant::now();
    // let lines = read_lines("input/day23.txt");
    
    let hallway = ['.';11];
    // let rooms = [['B','A'],['C','D'],['B','C'],['D','A']];
    // let rooms = [['C','B'],['B','C'],['A','D'],['D','A']];
    // let rooms = [['B','D','D','A'],['C','C','B','D'],['B','B','A','C'],['D','A','C','A']];
    let rooms = [['C','D','D','B'],['B','C','B','C'],['A','B','A','D'],['D','A','C','A']];

    let min = solve(rooms, hallway, 0, &mut HashMap::new());
    
    println!("{:?} in {:?}", min, start.elapsed());
}
