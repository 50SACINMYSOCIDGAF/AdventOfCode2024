// src/solutions/s01.rs
use std::collections::HashMap;

pub fn solve(input: &str) -> i64 {
    // We'll solve both parts and return part 2
    let (lists, similarity) = solve_both(input);
    similarity
}

pub fn solve_both(input: &str) -> (i64, i64) {
    let mut left_numbers: Vec<i64> = Vec::new();
    let mut right_numbers: Vec<i64> = Vec::new();

    // Parse input into two vectors
    for line in input.lines() {
        let numbers: Vec<i64> = line
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        if numbers.len() == 2 {
            left_numbers.push(numbers[0]);
            right_numbers.push(numbers[1]);
        }
    }

    // Part 1: Calculate total distance
    let mut left_sorted = left_numbers.clone();
    let mut right_sorted = right_numbers.clone();
    left_sorted.sort_unstable();
    right_sorted.sort_unstable();

    let total_distance: i64 = left_sorted.iter()
        .zip(right_sorted.iter())
        .map(|(left, right)| (left - right).abs())
        .sum();

    // Part 2: Calculate similarity score
    let mut right_counts: HashMap<i64, i64> = HashMap::new();
    for num in &right_numbers {
        *right_counts.entry(*num).or_insert(0) += 1;
    }

    let similarity_score: i64 = left_numbers.iter()
        .map(|num| num * right_counts.get(num).unwrap_or(&0))
        .sum();

    (total_distance, similarity_score)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "3   4\n4   3\n2   5\n1   3\n3   9\n3   3\n";
        let (distance, similarity) = solve_both(input);
        assert_eq!(distance, 11);
        assert_eq!(similarity, 31);
    }
}