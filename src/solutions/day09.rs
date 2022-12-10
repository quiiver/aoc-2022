use std::env;
use std::collections::HashSet;
use std::ops::{Add, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

// directions
static UP: Point = Point { x: 0, y: 1};
static DOWN: Point = Point { x: 0, y: -1};
static RIGHT: Point = Point { x: 1, y: 0};
static LEFT: Point = Point { x: -1, y: 0};

impl Point {

    fn new() -> Self {
        Point::from(0, 0)
    }

    fn from(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    fn is_touching(self, point: &Point) -> bool {
        (point.x - self.x == 0 && point.y - self.y == 0) // overlap
        || ((point.x - self.x).abs() == 1 && (point.y - self.y == 0))
        || ((point.y - self.y).abs() == 1 && (point.x - self.x == 0))
        || ((point.x - self.x).abs() == 1 && (point.y - self.y).abs() == 1)
    }

    fn get_move(self, other: Self) -> Self {
        let delta = self - other;
        if delta.x.abs() == 1 && delta.y.abs() == 1 {
            Point::from(0, 0)
        } else {
            Point {x: delta.x.signum(), y: delta.y.signum()}
        }
    }

    fn min(self, other: &Self) -> Self {
        Point{ x: self.x.min(other.x), y: self.y.min(other.y) }
    }

    fn max(self, other: &Self) -> Self {
        Point { x: self.x.max(other.x), y: self.y.max(other.y) }
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, point: Self) -> Self {
        Point {
            x: self.x + point.x,
            y: self.y + point.y,
        }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

fn draw_visits(visited: &HashSet<Point>) {
    let min = visited.iter().fold(Point::new(), |acc, p| acc.min(p));
    let max = visited.iter().fold(Point::new(), |acc, p| acc.max(p));

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
    let mut knots: Vec<Point> = (0..size).map(|_| Point::from(0, 0)).collect();
    let mut visited: HashSet<Point> = HashSet::new();
    visited.insert(knots[size-1]);

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
            knots[0] = knots[0] + direction;
            for idx in 1..size {
                if !knots[idx-1].is_touching(&knots[idx]) {
                    knots[idx] = knots[idx] + knots[idx-1].get_move(knots[idx]);
                    if idx+1 == size {
                        visited.insert(knots[idx]);
                    }
                }
            }
        }
    }

    if draw {
        draw_visits(&visited);
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
