use std::{collections::VecDeque, str::Lines};

fn parse_columns(row: &str) -> Vec<String> {
    row.chars()
        .collect::<Vec<char>>()
        .chunks(4)
        .map(|c| String::from_iter(c).to_owned())
        .map(|s| match s.matches(char::is_alphabetic).nth(0) {
            Some(m) => String::from(m),
            None => String::new(),
        })
        .collect::<Vec<String>>()
}

fn parse_stacks(rows: Vec<&str>) -> Vec<VecDeque<String>> {
    let cols: Vec<u32> = rows.last()
        .map(|x| 
            x.matches(char::is_numeric)
             .map(|x| x.parse().unwrap())
             .collect::<Vec<u32>>())
        .unwrap();

    let mut stacks: Vec<VecDeque<String>> = Vec::with_capacity(cols.len());
    for _ in 0..cols.len() {
        stacks.push(VecDeque::new())
    }

    for row in rows.iter().take_while(|s| s.starts_with('[')) {
        for (i, col) in parse_columns(row).iter().enumerate() {
            if col != "" {
                stacks[i].push_back(col.to_owned())
            }
        }
    }

    stacks
}

#[derive(Debug)]
struct Move {
    from: usize,
    to: usize,
    count: u32,
}

impl Move {
    fn from(line: &str) -> Option<Move> {
        if line.starts_with("move") {
            let m: Vec<u32> = line
                .split_whitespace()
                .filter(|s| s.matches(char::is_numeric).count() > 0 )
                .map(|s| s.parse().unwrap())
                .collect();
            Some(Move { from: (m[1] -1) as usize, to: (m[2] -1) as usize, count: m[0] })
        } else {
            None
        }
    }
}

fn parse_moves(lines: Lines) -> Vec<Move> {
    lines.map(Move::from)
        .flatten() 
        .collect()
}

fn get_stack_tops(stacks: &Vec<VecDeque<String>>) -> String {
    let tops = stacks.iter()
        .flat_map(|q| q.front())
        .map(String::as_str)
        .collect::<Vec<&str>>();

    tops[..].join("")
}

fn part1(input: &String) -> String {
    let mut stacks = parse_stacks(
        input.lines().take_while(|s| s.starts_with('[') || s.starts_with(" 1")).collect()
    );
    let moves = parse_moves(input.lines());
    for movement in moves {
        for _i in 0..movement.count {
            let from_item = stacks[movement.from].pop_front();
            stacks[movement.to].push_front(from_item.unwrap())
        }
    }

    get_stack_tops(&stacks)

}

fn part2(input: &String) -> String {
    let mut stacks = parse_stacks(
        input.lines().take_while(|s| s.starts_with('[') || s.starts_with(" 1")).collect()
    );
    let moves = parse_moves(input.lines());
    for movement in moves {
        let drained = stacks[movement.from]
            .drain(..(movement.count as usize))
            .rev()
            .collect::<VecDeque<String>>();

        for item in &drained {
            stacks[movement.to].push_front(item.to_owned())

        }
    }

    get_stack_tops(&stacks)

}

pub fn solution(input: &String) {
    println!("part1 {:?}", part1(input));
    println!("part2 {:?}", part2(input));
}
