use std::env;
use std::collections::HashSet;

#[derive(Debug, Copy, PartialEq, PartialOrd, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn from(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    fn add(&self, point: &Point) -> Point {
        Point {
            x: self.x + point.x,
            y: self.y + point.y,
        }
    }

    fn is_touching(&self, point: &Point) -> bool {
        (point.x - self.x == 0 && point.y - self.y == 0) // overlap
        || ((point.x - self.x).abs() == 1 && (point.y - self.y == 0))
        || ((point.y - self.y).abs() == 1 && (point.x - self.x == 0))
        || ((point.x - self.x).abs() == 1 && (point.y - self.y).abs() == 1)
    }

    fn get_move(&self, other: Self) -> Self {
        if self.x - other.x == 0 {
            match self.y > other.y {
                true => UP,
                false => DOWN,
            }
        } else if self.y - other.y == 0 {
            match self.x > other.x {
                true => RIGHT,
                false => LEFT,
            }
        } else {
            // right
            if self.x > other.x {
                // above
                if self.y > other.y {
                    RIGHT.add(&UP)
                } else {
                    RIGHT.add(&DOWN)
                }
            // left
            } else {
                // above
                if self.y > other.y {
                    LEFT.add(&UP)
                } else {
                    LEFT.add(&DOWN)
                }
            }
        }
    }
}


impl Ord for Point {
    fn cmp(&self, _: &Self) -> std::cmp::Ordering {
        std::cmp::Ordering::Equal
    }

    fn min(self, other: Self) -> Self
        where
            Self: Sized, {
        Point{ x: self.x.min(other.x), y: self.y.min(other.y) }
    }

    fn max(self, other: Self) -> Self
        where
            Self: Sized, {
        Point { x: self.x.max(other.x), y: self.y.max(other.y) }
    }
}

impl Clone for Point {
    fn clone(&self) -> Point {
        Point { x: self.x, y: self.y }
    }
}

static UP: Point = Point { x: 0, y: 1};
static DOWN: Point = Point { x: 0, y: -1};
static RIGHT: Point = Point { x: 1, y: 0};
static LEFT: Point = Point { x: -1, y: 0};

fn draw_visits(visited: &HashSet<Point>, min: Point, max: Point) {
    println!("{}", ["-"; 80].join(""));
    for y in ((min.y-1)..(max.y+1)).rev() {
        for x in min.x-1..max.x+1 {
            let char = if (x, y) == (0, 0) { 's' } else if visited.contains(&Point::from(x, y)) { '#' } else { '.' };
            print!("{}", char);
        }
        print!("\n");
    }
    println!("{}", ["-"; 80].join(""));

}

fn solve(input: &String, size: usize, draw: bool) -> usize {
    let mut max = Point::from(0, 0);
    let mut min = Point::from(0, 0);
    let mut knots: Vec<Point> = (0..size).map(|_| Point::from(0, 0)).collect();
    let mut visited: HashSet<Point> = HashSet::new();
    visited.insert(knots[size-1].clone());

    for line in input.lines() {
        let split: Vec<&str> = line.split_whitespace().collect();
        let direction = match split[0] {
            "U" => UP,
            "D" => DOWN,
            "R" => RIGHT,
            "L" => LEFT,
            _ => Point::from(0, 0),
        };

        let count = match split[1].parse() {
            Ok(c) => c,
            _ => 0,
        };


        for _ in 0..count {
            knots[0] = knots[0].add(&direction);
            max = max.max(knots[0]);
            min = min.min(knots[0]);
            for idx in 1..size {
                if !knots[idx-1].is_touching(&knots[idx]) {
                    knots[idx] = knots[idx].add(&knots[idx-1].get_move(knots[idx]));
                    if idx+1 == size {
                        visited.insert(knots[idx].clone());
                    }
                }
            }
        }
    }

    if draw {
        draw_visits(&visited, min, max);
    }

    visited.len()
}

pub fn solution(input: &String) {
    let draw_arg = match env::args().nth(2).filter(|x| x.starts_with("draw")) {
        Some(_) => true,
        _ => false,
    };

    println!("part 1: {:?}", solve(input, 2, draw_arg));

    println!("part 2: {:?}", solve(input, 10, draw_arg));
}
