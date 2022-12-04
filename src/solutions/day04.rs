fn solve(input: &String, check: fn(u32, u32, u32, u32) -> u32) -> u32 {
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
        .map(|xs| check(xs[0][0], xs[0][1], xs[1][0], xs[1][1]))
        .sum()
}
pub fn solution(input: &String) {
    println!("part1 {:?}", solve(input, |l1, l2, r1, r2| {
        if l1 >= r1 && l2 <= r2 || l1 <= r2 && l2 >= r2 {
            1
        } else {
            0
        }
    }));

    println!("part2 {:?}", solve(input, |l1, l2, r1, r2| {
        if l2 >= r1 && l1 <= r2 || r2 >= l1 && r1 <= l2 {
            1
        } else {
            0
        }
    }));
}
