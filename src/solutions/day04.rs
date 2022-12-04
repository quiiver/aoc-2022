fn solve(input: &String, check: fn((u32, u32), (u32, u32)) -> u32) -> u32 {
    input
        .lines()
        .map(|x| x.split(',')
            .take(2)
            .map(|r| r.split('-')
                .map(|i| i.parse().unwrap())
                .take(2)
                .collect::<Vec<u32>>()
            ).collect::<Vec<_>>()
        )
        .map(|xs| check((xs[0][0], xs[0][1]), (xs[1][0], xs[1][1])))
        .sum()
}
pub fn solution(input: &String) {
    println!("part1 {:?}", solve(input, |l, r| {
        if l.0 >= r.0 && l.1 <= r.1 || l.0 <= r.1 && l.1 >= r.1 {
            1
        } else {
            0
        }
    }));

    println!("part2 {:?}", solve(input, |l, r| {
        if l.1 >= r.0 && l.0 <= r.1 || r.1 >= l.0 && r.0 <= l.1 {
            1
        } else {
            0
        }
    }));
}
