
pub fn solution(input: String) {
    let mut sum_calories: Vec<u32> = input
        .split("\n\n")
        .map(|x| x.split('\n')
            .filter(|x| !x.is_empty())
            .map(|n| n.parse::<u32>().unwrap())
            .sum()
        )
        .collect::<Vec<u32>>();

    sum_calories.sort();

    println!(
        "part 1: {:?}",
            sum_calories.last().unwrap()
        ,
    );

    println!(
        "part 2: {:?}",
            sum_calories.iter().rev().take(3).sum::<u32>()
        ,
    );
}

