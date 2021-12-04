use std::cmp::Ordering;
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

fn monotonic_depth_cnt(k0: i32, cache: &[i32]) -> Vec<i32> {
    let acc = cache[0];
    let k1 = cache[1];

    match k1.cmp(&k0) {
        Ordering::Less => vec![acc + 1, k0],
        Ordering::Greater | Ordering::Equal => vec![acc, k0],
    }
}

fn monotonic_depth_wind_cnt(k0: i32, c: &[i32]) -> Vec<i32> {
    let acc = c[0];

    let p: i32 = c[2..].iter().sum();
    let q: i32 = c[1..].iter().sum();

    match q.cmp(&(p + k0)) {
        Ordering::Less => vec![acc + 1, c[2], c[3], k0],
        Ordering::Greater | Ordering::Equal => vec![acc, c[2], c[3], k0],
    }
}

fn part_1(file: &str) -> Option<i32> {
    if let Ok(lines) = read_lines(file) {
        let mut cache = vec![-1, 0];

        for line in lines {
            if let Ok(depth) = line {
                let depth: i32 = depth.parse().unwrap();
                cache = monotonic_depth_cnt(depth, &cache[..]);
            }
        }
        Some(cache[0])
    } else {
        None
    }
}

fn part_2(file: &str) -> Option<i32> {
    if let Ok(lines) = read_lines(file) {
        let mut cache = vec![0, -1, -1, -1]; // acc, k3, k2, k1

        for line in lines {
            if let Ok(depth) = line {
                let depth = depth.parse().unwrap();
                if cache[1] == -1 {
                    cache[1] = depth;
                    continue;
                } else if cache[2] == -1 {
                    cache[2] = depth;
                    continue;
                } else if cache[3] == -1 {
                    cache[3] = depth;
                    continue;
                }
                cache = monotonic_depth_wind_cnt(depth, &cache[..]);
            }
        }
        Some(cache[0])
    } else {
        None
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = &args[1];

    let t0 = Instant::now();
    match part_1(input) {
        Some(n) => {
            let delta = t0.elapsed().as_micros();
            println!("Max depth counter: {}. Elapsed: {} [us]", n, delta)
        }
        None => eprintln!("Something failed hard :("),
    }
    let t0 = Instant::now();
    match part_2(input) {
        Some(n) => {
            let delta = t0.elapsed().as_micros();
            println!(
                "Max depth counter [3º window]: {}. Elapsed: {} [us]",
                n, delta
            )
        }
        None => eprintln!("Something failed hard :("),
    }
}
