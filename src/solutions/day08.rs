
fn check_visible((y, x): (usize, usize), grid: &Vec<Vec<u32>>) -> bool {
    let (height, width) = (grid.len(), grid[0].len());
    let point = Some(grid[y][x]);
    let top = grid[0..y].iter().map(|xs| xs[x]).max();
    let bottom = grid[y+1..height].iter().map(|xs| xs[x]).max();
    let left = Some(*grid[y][0..x].iter().max().unwrap());
    let right = Some(*grid[y][x+1..width].iter().max().unwrap());

    point > top || point > bottom || point > left || point > right
}

fn solve(input: &String) -> u32 {
    let grid: Vec<Vec<u32>> = input.lines()
        .map(|line| line
            .chars()
            .flat_map(|s| s.to_string().parse() )
            .collect()
        )
        .collect();
    let (height, width) = (grid.len(), grid[0].len());
    let mut visible = 0;
    for y in 1..(height-1) {
        for x in 1..(width-1) {
            let is_vis = check_visible((y, x), &grid);
            visible += is_vis as usize;
        }
    }
    (width * 2 + (height - 2) * 2 + visible) as u32
}

fn count_trees(height: u32, trees: Vec<u32>) -> u32 {
    let mut count = 0;
    for tree in trees {
        if tree < height {
            count += 1;
        } else if tree >= height {
            return count + 1
        }
    }
    count
}

fn check_score((y, x): (usize, usize), grid: &Vec<Vec<u32>>) -> u32 {
    let (height, width) = (grid.len(), grid[0].len());
    let point = grid[y][x];
    let top = grid[0..y].iter().rev().map(|xs| xs[x]);
    let bottom = grid[y+1..height].iter().map(|xs| xs[x]);
    let left = grid[y][0..x].iter().rev().map(|x| *x);
    let right = grid[y][x+1..width].iter().map(|x| *x);

    count_trees(point, top.collect::<Vec<u32>>())
        * count_trees(point, bottom.collect::<Vec<u32>>())
        * count_trees(point, left.collect::<Vec<u32>>())
        * count_trees(point, right.collect::<Vec<u32>>())
}

fn solve2(input: &String) -> u32 {
    let grid: Vec<Vec<u32>> = input.lines()
        .map(|line| line
            .chars()
            .flat_map(|s| s.to_string().parse() )
            .collect()
        )
        .collect();
    let (height, width) = (grid.len(), grid[0].len());
    let mut scores: Vec<u32> = Vec::new();
    for y in 1..height-1 {
        for x in 1..width-1 {
            let score = check_score((y, x), &grid);
            scores.push(score);
        }
    }

    *scores.iter().max().unwrap()
}


pub fn solution(input: &String) {
    println!("part 1: {:?}", solve(input));
    println!("part 2: {:?}", solve2(input));
}
