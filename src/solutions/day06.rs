use std::collections::{HashSet, VecDeque};

fn solve(input: &String, packet_size: usize) -> u32 {
    let mut packet = VecDeque::new();
    input.chars()
        .take_while(|c| {
            packet.push_back(c.to_owned());
            if packet.len() > packet_size {
                packet.pop_front();
            }
            let set = HashSet::<_>::from_iter(&packet);
            !(packet.len() == packet_size && packet.len() == set.len())
        })
        // take_while stops before the last char so just add 1 here.
        .count() as u32 + 1
}

pub fn solution(input: &String) {
    println!("part1 {:?}", solve(input, 4));
    println!("part2 {:?}", solve(input, 14));
}

