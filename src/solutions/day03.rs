fn part1(input: &String) -> u32 {
    let char_map = ".abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    input
        .lines()
        .map(|s| {
            let (l, r) = s.split_at(s.len()/2);
            match l.chars().find(|&x| r.contains(x)) {
                Some(c) => char_map.find(c).unwrap() as u32,
                None => 0
            }
        })
        .sum()
}

fn part2(input: &String) -> u32 {
    let char_map = ".abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    input
        .lines()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|chunk| {
            match chunk[0].chars().find(|&x| chunk[1].contains(x) && chunk[2].contains(x)) {
                Some(c) => char_map.find(c).unwrap() as u32,
                None => 0
            }
        })
        .sum()
}

pub fn solution(input: &String) {
    println!("part1 {:?}", part1(input));

    println!("part2 {:?}", part2(input));
}
