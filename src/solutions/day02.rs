
fn part1(input: &String) -> u32 {
    input
        .lines()
        .map(|s| {
            s.split(' ').collect()
        })
        .map(|xs: Vec<&str>| match xs[..] {
            [opponent, "X"] => (match opponent {
                "A"=> 3,
                "B" => 0,
                "C" => 6,
                _ => 0,
            }) + 1,
            [opponent, "Y"] => (match opponent {
                "A"=> 6,
                "B" => 3,
                "C" => 0,
                _ => 0,
            }) + 2,
            [opponent, "Z"] => (match opponent {
                "A"=> 0,
                "B" => 6,
                "C" => 3,
                _ => 0,
            }) + 3,
            _ => 0
        })
        .sum()
}

fn part2(input: &String) -> u32 {
    input
        .lines()
        .map(|s| {
            s.split(' ').collect()
        })
        .map(|xs: Vec<&str>| match xs[..] {
            // rock
            ["A", play] => (match play {
                "X"=> 0 + 3,
                "Y" => 3 + 1,
                "Z" => 6 + 2,
                _ => 0,
            }),
            ["B", play] => (match play {
                "X"=> 0 + 1,
                "Y" => 3 + 2,
                "Z" => 6 + 3,
                _ => 0,
            }),
            ["C", play] => (match play {
                "X"=> 0 + 2,
                "Y" => 3 + 3,
                "Z" => 6 + 1,
                _ => 0,
            }),
            _ => 0
        })
        .sum()
}

pub fn solution(input: &String) {
    println!("part1 {:?}", part1(input));

    println!("part2 {:?}", part2(input));
}
