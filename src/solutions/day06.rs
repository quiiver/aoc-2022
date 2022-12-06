use std::collections::{HashSet, VecDeque};

fn solve(input: &String, packet_size: usize) -> usize {
    let mut packet = VecDeque::new();
    input.chars()
        .position(|c| {
            packet.push_back(c.to_owned());
            if packet.len() > packet_size {
                packet.pop_front();
            }
            let set = HashSet::<_>::from_iter(&packet);
            packet.len() == packet_size && packet.len() == set.len()
        }).unwrap() + 1 // account for 0 offset
}

pub fn solution(input: &String) {
    println!("part1 {:?}", solve(input, 4));
    println!("part2 {:?}", solve(input, 14));
}

