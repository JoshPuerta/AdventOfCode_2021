use std::{
    collections::HashSet,
    env,
    fs::File,
    io::{self, BufRead, BufReader, Lines},
    iter::Peekable,
    path::Path,
    time::Instant,
};

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Debug)]
struct RegisterLog {
    pos: Vec<u32>,
    entries: u32,
    regsize: usize,
    rmap: Vec<Vec<u8>>,
}

impl RegisterLog {
    fn count(&mut self, reg: &str) {
        let mut v: Vec<u8> = vec![0; self.regsize];
        reg.char_indices()
            .map(|q| match q.1 {
                '1' => {
                    self.pos[q.0] += 1;
                    v[q.0] += 1;
                }
                _ => {}
            })
            .count();
        self.entries += 1;
        self.rmap.push(v);
    }

    fn calc_power_cons(&self) -> u32 {
        let gamma: u32 = self
            .pos
            .iter()
            .enumerate()
            .map(|x| {
                if x.1 >= &(self.entries / 2) {
                    1 << (self.regsize - 1 - x.0)
                } else {
                    0
                }
            })
            .sum();

        let epsilon: u32 = gamma ^ ((2 << (self.regsize - 1)) - 1);
        gamma * epsilon
    }

    fn calc_oxy_rate(&self) -> u32 {
        let v: Vec<u8> = self
            .pos
            .iter()
            .map(|x| if x >= &(self.entries / 2) { 1 } else { 0 })
            .collect();

        let mut set: HashSet<usize> = (0..self.entries as usize).collect();
        for i in 0..self.regsize {
            for s in &set {
                if self.rmap[*s][i] != v[i] {
                    set.remove(&&s);
                }
            }
        }

        0
    }
}

fn peek_reg_len(pk: &mut Peekable<Lines<BufReader<File>>>) -> usize {
    let fl = pk.peek();
    if let Some(lo) = fl {
        if let Ok(lr) = lo {
            lr.len()
        } else {
            0 as usize
        }
    } else {
        0 as usize
    }
}

fn part_1(file: &str) -> Option<u32> {
    if let Ok(lines) = read_lines(file) {
        // Check len of each reg
        let mut plines = lines.peekable();
        let size: usize = peek_reg_len(&mut plines);

        let mut pc: RegisterLog = RegisterLog {
            pos: vec![0; size],
            entries: 0,
            regsize: size,
            rmap: vec![vec![0; size]],
        };

        plines
            .map(|reg| match reg {
                Ok(vreg) => pc.count(vreg.as_str()),
                Err(_) => {}
            })
            .count();

        Some(pc.calc_power_cons())
    } else {
        None
    }
}

fn part_2(file: &str) -> Option<u32> {
    if let Ok(lines) = read_lines(file) {
        // Check len of each reg
        let mut plines = lines.peekable();
        let size: usize = peek_reg_len(&mut plines);

        let mut rl: RegisterLog = RegisterLog {
            pos: vec![0; size],
            entries: 0,
            regsize: size,
            rmap: Vec::new(),
        };

        plines
            .map(|reg| match reg {
                Ok(vreg) => rl.count(vreg.as_str()),
                Err(_) => {}
            })
            .count();

        println!("Register log: {:?}", rl);
        Some(rl.calc_power_cons())
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
            println!("Power consumption: {}. Elapsed: {} [us]", res, delta)
        }
        None => eprintln!("Something failed hard :("),
    }

    let t0 = Instant::now();
    match part_2(input) {
        Some(res) => {
            let delta = t0.elapsed().as_micros();
            println!("Life Support rating: {}. Elapsed: {} [us]", res, delta)
        }
        None => eprintln!("Something failed hard :("),
    }
}
