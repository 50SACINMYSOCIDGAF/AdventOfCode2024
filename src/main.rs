// src/main.rs
use std::fs;

mod solutions {
    pub mod s01;
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Please provide the day number as an argument (1-25)");
        return;
    }

    let day = args[1].parse::<u8>().unwrap_or(0);
    match day {
        1 => run_day(1),
        _ => println!("Day {} not implemented yet", day),
    }
}

fn run_day(day: u8) {
    let input = fs::read_to_string(format!("input/day{:02}.txt", day))
        .expect("Failed to read input file");

    let result = match day {
        1 => {
            let (part1, part2) = solutions::s01::solve_both(&input);
            println!("Part 1: {}", part1);
            part2
        },
        _ => panic!("Day not implemented")
    };

    println!("Part 2: {}", result);
}