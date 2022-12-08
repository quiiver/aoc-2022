use std::{collections::HashMap, path::{PathBuf, Path}};
#[derive(Debug)]
enum CommandName {
    CD,
    LS,
}

#[derive(Debug)]
struct Command {
    name: CommandName,
    argument: String,
}

fn parse_command(line: &str) -> Command {
    let cd_prefix = "$ cd ";
    if line.starts_with(cd_prefix) {
        let argument = line.split_at(cd_prefix.len()).1.to_string();
        Command { name: CommandName::CD, argument}
    } else {
        Command { name: CommandName::LS, argument: "".to_string()}
    }
}

fn parse_file_size(line: &str) -> u32 {
    if line.starts_with(char::is_numeric) {
        line.split_whitespace()
            .nth(0).unwrap()
            .parse::<u32>().unwrap()
    } else {
        0
    }
}

fn parse_lines(input: &String) -> HashMap<PathBuf, u32> {
    let mut lines = input.lines();
    let mut cwd = PathBuf::new();
    let mut fsmap: HashMap<PathBuf, u32> = HashMap::new();
    let mut dir_size: u32 = 0;
    while let Some(line) = lines.next() {
        let cmd = parse_command(line);
        match cmd.name {
            CommandName::CD => {
                if cmd.argument == ".." {
                    cwd.pop();
                } else {
                    cwd.push(cmd.argument);
                }
            },
            _ => {
                dir_size += parse_file_size(line);
                if dir_size > 0 {
                    let ancestors = cwd.ancestors(); 
                    for parent in ancestors {
                        *fsmap.entry(parent.to_owned()).or_insert(0) += dir_size;
                    }
                    dir_size = 0;
                }
            }
        }
    }

    fsmap
}

fn solve(input: &String) -> u32 {
    let fsmap = parse_lines(input);
    let max = 100000;
    fsmap.values().filter(|v| **v < max).sum::<u32>()
}

fn solve2(input: &String) -> u32 {
    let fsmap = parse_lines(input);
    let total = 70_000_000;
    let free = 30_000_000;
    let used = fsmap.get(&Path::new("/").to_path_buf()).unwrap();
    *fsmap.values().filter(|v| **v > (free - (total - used))).min().unwrap()
}


pub fn solution(input: &String) {
    println!("part 1: {:?}", solve(input));
    println!("part 2: {:?}", solve2(input));
}


