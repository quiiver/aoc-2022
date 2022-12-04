static CHAR_MAP: &str = ".abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn part1(input: &String) -> u32 {
    input
        .lines()
        .map(|s| {
            let (l, r) = s.split_at(s.len()/2);
            match l.chars().find(|&x| r.contains(x)) {
                Some(c) => CHAR_MAP.find(c).unwrap() as u32,
                None => 0
            }
        })
        .sum()
}

fn part2(input: &String) -> u32 {
    input
        .lines()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|chunk| {
            match chunk[0].chars().find(|&x| chunk[1].contains(x) && chunk[2].contains(x)) {
                Some(c) => CHAR_MAP.find(c).unwrap() as u32,
                None => 0
            }
        })
        .sum()
}

pub fn solution(input: &String) {
    println!("part1 {:?}", part1(input));

    println!("part2 {:?}", part2(input));
}
