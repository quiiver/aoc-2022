fn solve(input: &String, check: fn((u32, u32), (u32, u32)) -> bool) -> usize {
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
        .filter(|xs| check((xs[0][0], xs[0][1]), (xs[1][0], xs[1][1])))
        .count()
}
pub fn solution(input: &String) {
    println!("part1 {:?}", solve(input, |l, r| {
        l.0 >= r.0 && l.1 <= r.1 || l.0 <= r.1 && l.1 >= r.1
    }));

    println!("part2 {:?}", solve(input, |l, r| {
        l.1 >= r.0 && l.0 <= r.1 || r.1 >= l.0 && r.0 <= l.1
    }));
}
