use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
#[derive(Debug)]
struct Position {
    x: i32,
    y: i32,
    aim: i32,
}

impl Position {
    fn depth(&self) -> i64 {
        i64::from(&self.x * &self.y)
    }

    fn navigate(&mut self, cmd: Vec<String>) {
        let val: i32 = cmd.last().unwrap().parse().unwrap();

        match cmd.first().unwrap().as_str() {
            "forward" => self.x += val,
            "down" => self.y += val,
            "up" => self.y -= val,
            _ => {}
        };
    }

    fn navigate_with_aim(&mut self, cmd: Vec<String>) {
        let val: i32 = cmd.last().unwrap().parse().unwrap();

        match cmd.first().unwrap().as_str() {
            "forward" => {
                self.x += val;
                self.y += self.aim * val;
            }
            "down" => self.aim += val,

            "up" => self.aim -= val,

            _ => {}
        };
    }
}

fn part_1(file: &str) -> Option<i64> {
    let mut pos: Position = Position { x: 0, y: 0, aim: 0 };

    if let Ok(lines) = read_lines(file) {
        for line in lines {
            if let Ok(pload) = line {
                let command: Vec<String> =
                    pload.split_whitespace().map(|s| s.to_string()).collect();
                pos.navigate(command);
            }
        }
        Some(pos.depth())
    } else {
        None
    }
}

fn part_2(file: &str) -> Option<i64> {
    let mut pos: Position = Position { x: 0, y: 0, aim: 0 };

    if let Ok(lines) = read_lines(file) {
        for line in lines {
            if let Ok(pload) = line {
                let command: Vec<String> =
                    pload.split_whitespace().map(|s| s.to_string()).collect();
                pos.navigate_with_aim(command);
            }
        }
        Some(pos.depth())
    } else {
        None
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];

    let t0 = Instant::now();
    match part_1(input) {
        Some(res) => {
            let delta = t0.elapsed().as_micros();
            println!("Horiz * Depth: {}. Elapsed: {} [us]", res, delta)
        }
        None => eprintln!("Something failed hard :("),
    }

    let t0 = Instant::now();
    match part_2(input) {
        Some(res) => {
            let delta = t0.elapsed().as_micros();
            println!("Horiz * Depth (Aim): {}. Elapsed: {} [us]", res, delta)
        }
        None => eprintln!("Something failed hard :("),
    }
}
