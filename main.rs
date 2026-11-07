// •	From project root: cargo run -- <n> <total limit> <number of print columns>
//•	Example: cargo run -- 50 100 4 

use std::collections::{HashMap, HashSet};
use std::env;

fn all_unordered_nonneg_four_squares(n: u64) -> Vec<[u64; 4]> {
    let m = (n as f64).sqrt() as u64;
    let mut map: HashMap<u64, Vec<(u64, u64)>> = HashMap::new();

    for i in 0..=m {
        let si = i * i;
        for j in i..=m {
            let s = si + j * j;
            if s <= n {
                map.entry(s).or_default().push((i, j));
            } else {
                break;
            }
        }
    }

    let mut uniq: HashSet<[u64; 4]> = HashSet::new();
    for (&s, pairs1) in &map {
        let t = n.saturating_sub(s);
        if let Some(pairs2) = map.get(&t) {
            for &(a, b) in pairs1 {
                for &(c, d) in pairs2 {
                    let mut arr = [a, b, c, d];
                    arr.sort_unstable();
                    uniq.insert(arr);
                }
            }
        }
    }

    let mut res: Vec<[u64; 4]> = uniq.into_iter().collect();
    res.sort_unstable();
    res
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        eprintln!("Usage: {} <n>", args[0]);
        std::process::exit(1);
    }

    let n: u64 = match args[1].parse() {
        Ok(v) => v,
        Err(_) => {
            eprintln!("Invalid integer: {}", args[1]);
            std::process::exit(1);
        }
    };

    let limit: usize = match args[2].parse() {
        Ok(v) => v,
        Err(_) => {
            eprintln!("Invalid integer: {}", args[2]);
            std::process::exit(1);
        }
    };

    let columns: usize = match args[3].parse() {
        Ok(v) => v,
        Err(_) => {
            eprintln!("Invalid integer: {}", args[3]);
            std::process::exit(1);
        }
    };

    let sols = all_unordered_nonneg_four_squares(n);

    if sols.len() > limit {println!("n = {}, unique unordered nonnegative solutions: {}, the limit is {}", n, sols.len(), limit);
               std::process::exit(0) };

    println!("n = {}, unique unordered nonnegative solutions: {}", n, sols.len());
    let mut i:usize = 0;
    for q in sols {
        if i%columns != columns - 1 {print!("{:?}, ", q); i+=1} else {println!("{:?},", q); i = 0};
    }
}