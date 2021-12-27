use crate::aoc_util::read_lines;

struct Count {
    one_count: i32,
    zero_count: i32
}

fn parse(lines: &Vec<String>) -> (Vec<i32>, Vec<Vec<Vec<(i32, bool)>>>) {
    let mut input = Vec::<i32>::new();
    let mut boards: Vec<Vec<Vec<(i32, bool)>>> = Vec::<Vec<Vec<(i32, bool)>>>::new();
    let mut current_board = &mut Vec::<Vec<(i32,bool)>>::new();

    for (index, line) in (&lines).iter().enumerate() {
        if index == 0 {
            input = line.split(",").map(|s| s.parse().unwrap()).collect::<Vec<i32>>();
        } else if line == "" {
            boards.push(Vec::<Vec<(i32, bool)>>::new());
            current_board = boards.last_mut().unwrap();
        } else {
            current_board.push(
                line.split(" ")
                .filter_map(|s| s.parse().ok())
                .map(|x| (x, false))
                .collect::<Vec<(i32, bool)>>()
            )
        }
    }
    (input, boards)
}

fn has_bingo(board: &Vec<Vec<(i32, bool)>>) -> bool {

    let hori = board.iter().any(
        |r| r.into_iter().all(
            |(_n,m)| *m)
        );

    if hori {
        return true;
    }

    let rows = board[0].len();
    for i in 0..rows {
        let vert = board.iter().map(|b| b[i]).all(|(_n,m)| m);

        if vert {
            return true;
        }
    }

    false
}

pub fn run() {
    
    let lines = read_lines("input/day4.txt").collect::<Vec<String>>();
    let (input, mut boards) = parse(&lines);
    
    // let mut bingo_boards = Vec::<Vec<Vec<(i32, bool)>>>::new();
    for i in 0..input.len() {
        let inp = input[i];

        for j in 0..boards.len() {
            let board = &mut boards[j];
            for k in 0..board.len() {
                let row = &mut board[k];
                for l in 0..row.len() {
                    let (number, _marked) = row[l];
                    if number == inp {
                        row[l] = (number, true);
                    }
                }
            }
        }
        
        if boards.len() > 1 {
            boards = boards.into_iter()
                .filter(|b| !has_bingo(b))
                .collect::<Vec<Vec<Vec<(i32,bool)>>>>();
        }
        else if has_bingo(&boards[0]) {
            // println!("bingo! {}", bingo_boards.len());
            let score = (&boards[0]).into_iter()
                .flat_map(
                    |r| r.into_iter().filter_map(
                        |(n,m)| match m {
                            false => Some(n),
                            _ => None
                        }
                    ))
                .sum::<i32>();
                        
            println!("bingo! {}", (score * inp));
            break;
        }
    }
}