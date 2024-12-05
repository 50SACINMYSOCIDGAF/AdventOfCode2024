// src/main.rs
use std::fs;

mod solutions {
    pub mod s01;
    pub mod s02;
    pub mod s03;
    pub mod s04;
    pub mod s05;
    // Future days will be added here
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Please provide the day number as an argument (1-25)");
        return;
    }

    let day = match args[1].parse::<u8>() {
        Ok(n) if (1..=25).contains(&n) => n,
        _ => {
            println!("Please provide a valid day number between 1 and 25");
            return;
        }
    };

    match run_day(day) {
        Ok(()) => (),
        Err(e) => println!("{}", e),
    }
}

fn run_day(day: u8) -> Result<(), String> {
    // Try to read the input file
    let input = fs::read_to_string(format!("src/input/day{:02}.txt", day))
        .map_err(|e| format!("Error reading input file for day {}: {}\nMake sure src/input/day{:02}.txt exists!", day, e, day))?;

    // Try to run the solution for the given day
    match day {
        1 => {
            let (part1, part2) = solutions::s01::solve_both(&input);
            println!("Part 1: {}", part1);
            println!("Part 2: {}", part2);
        },
        2 => {
            let (part1, part2) = solutions::s02::solve_both(&input);
            println!("Part 1: {}", part1);
            println!("Part 2: {}", part2);
        },
        3 => {
            let (part1, part2) = solutions::s03::solve_both(&input);
            println!("Part 1: {}", part1);
            println!("Part 2: {}", part2);
        },
        4 => {
            let (part1, part2) = solutions::s04::solve_both(&input);
            println!("Part 1: {}", part1);
            println!("Part 2: {}", part2);
        },
        5 => {
            let (part1, part2) = solutions::s05::solve_both(&input);
            println!("Part 1: {}", part1);
            println!("Part 2: {}", part2);
        },
        d => return Err(format!("Day {} not implemented yet", d)),
    }

    Ok(())
}