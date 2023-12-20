use std::fs;

fn find_increases(values: &[u32]) -> u32 {
    let mut increases: u32 = 0;
    let mut previous: u32 = values[0];
    for &depth in values.iter().skip(1) {
        if depth > previous {
            increases += 1
        }
        previous = depth;
    }
    increases
}

pub fn solve() {
    let contents: String = fs::read_to_string("inputs/day_01.txt").expect("Unable to read file!");
    let depths: Vec<u32> = contents
        .split_whitespace()
        .map(|depth| depth.parse().unwrap())
        .collect();

    println!("🦀 Day 1 🦀");
    println!("🦀 Ex 1: {} 🦀", find_increases(&depths)); // 1527

    let grouped_depths: Vec<u32> = depths
        .windows(3)
        .map(|window| window.iter().sum())
        .collect();

    println!("🦀 Ex 2: {} 🦀", find_increases(&grouped_depths)); // 1575
}
