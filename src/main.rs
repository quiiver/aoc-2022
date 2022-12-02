use std::env;
use std::fs;
use std::usize;

mod solutions;

pub fn read_file(day: u8) -> String {
    let cwd = env::current_dir().unwrap();

    let filepath = cwd
        .join("inputs")
        .join(format!("day{:02}.txt", day));

    let f = fs::read_to_string(filepath);
    f.expect("could not open input file")
}

static SOLUTIONS: &[fn(&String)] = &[
    solutions::day01::solution,
    solutions::day02::solution,
];

fn main() {
    let day_arg = env::args().nth(1).map(|x| x.parse::<usize>()).unwrap();
    match day_arg {
        Ok(day) => {
            if day <= SOLUTIONS.len() {
                SOLUTIONS[(day-1)](&read_file(day as u8))
            } else {
                println!("Day argument out of bounds")
            }
        },
        Err(_) => println!("Not a valid day arg"),
    }
}
