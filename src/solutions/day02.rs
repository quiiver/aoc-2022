
fn solve(input: &String, scores: &[u32]) -> u32 {
    input
        .lines()
        .map(|s| {
            s.split(' ')
                .flat_map(|x| x.chars())
                .collect::<Vec<char>>()
        })
        .map(|xs: Vec<char>| match xs[..] {
            [l, r] => 3*(l as u32 - 'A' as u32) + r as u32 - 'X' as u32,
            _ => 10,
        })
        .map(|idx| scores[idx as usize])
        .sum()
}

pub fn solution(input: &String) {
    println!("part1 {:?}", solve(input, &[4, 8, 3, 1, 5, 9, 7, 2, 6]));

    println!("part2 {:?}", solve(input, &[3, 4, 8, 1, 5, 9, 2, 6, 7]));
}
