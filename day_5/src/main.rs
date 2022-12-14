use std::{io::{self, BufRead, StdinLock}, collections::VecDeque};

fn import_state(mut lines: std::io::Lines<StdinLock>) -> (Option<Vec<Vec<char>>>, std::io::Lines<StdinLock>){
    let mut input_vec: VecDeque<_> = VecDeque::new();
    while let Some(line) = lines.next() {
        let input_line = match line {
            Ok(val) => val,
            Err(e) => {println!("Error: {}", e); continue;},
        };
        if input_line.chars().collect::<Vec<char>>()[1] == '1' {
            let input_cols:Vec<&str> = input_line.split("").collect();
            let start_state = parse_input(input_vec, input_cols);
            return (Some(start_state), lines);

        } else {
            input_vec.push_front(String::from(input_line.to_string()));
        }
    }

    (None, lines)
}

fn parse_input(buffer: VecDeque<String>, input_cols: Vec<&str>) -> Vec<Vec<char>> {
    let mut parse_rows: Vec<usize> = Vec::new();
    let mut state: Vec<Vec<char>> = Vec::new();
    for i in 0..(input_cols.len()-2) {
        if input_cols[i+1] != " " {
            state.push(Vec::new());
            parse_rows.push(i);
        }
    }

    for row in buffer {
        let row_chars: Vec<char> = row.chars().collect();
        for (trans_col, col) in parse_rows.clone().into_iter().enumerate() {
            if row_chars[col].is_alphabetic() {
                state[trans_col].push(row_chars[col])
            }
        }
    }

    state
}

fn parse_direction(direction: &str) -> (usize, usize, usize) {
    let parsed = direction.split(" ").collect::<Vec<&str>>();
    (parsed[1].parse::<usize>().unwrap(), parsed[3].parse::<usize>().unwrap() - 1, parsed[5].parse::<usize>().unwrap() - 1)

}

fn pt1(mut lines: std::io::Lines<StdinLock>, mut state: Vec<Vec<char>>) -> Vec<Vec<char>>{
    while let Some(line) = lines.next() {
        let input_line = match line {
            Ok(val) => val,
            Err(e) => {println!("Error: {}", e); continue;},
        };

        if input_line == "" {
            continue;
        }

        let (num, src, dest) = parse_direction(&input_line);

        for _i in 0..num {
            let temp = match state[src].pop() {Some(val) => val, None => continue,};
            state[dest].push(temp);
        }
    }

    state
}

fn pt2(mut lines: std::io::Lines<StdinLock>, mut state: Vec<Vec<char>>) -> Vec<Vec<char>>{
    while let Some(line) = lines.next() {
        let input_line = match line {
            Ok(val) => val,
            Err(e) => {println!("Error: {}", e); continue;},
        };

        if input_line == "" {
            continue;
        }

        let (num, src, dest) = parse_direction(&input_line);

        let mut temp: Vec<char> = Vec::new();
        for _i in 0..num {
            temp.push(match state[src].pop() {Some(val) => val, None => continue,});
        }
        for _i in 0..num {
            state[dest].push(match temp.pop() {Some(val) => val, None => continue,});
        }
    }

    state
}


fn main() {
    let lines = io::stdin().lock().lines();
    let (state, lines) = import_state(lines);
    let state = match state {
        Some(val) => val,
        None => return,
    };

    //let state = pt1(lines, state);
    let state = pt2(lines, state);

    for col in state {
        if col.len() <= 0 {
            continue;
        }
        print!("{}", col[col.len()-1]);
    }
    println!("");
}
