// src/solutions/s02.rs
pub fn solve(input: &str) -> i64 {
    count_safe_reports(input)
}

fn is_safe_sequence(nums: &[i64]) -> bool {
    if nums.len() <= 1 {
        return true;
    }

    let differences: Vec<i64> = nums.windows(2)
        .map(|w| w[1] - w[0])
        .collect();

    let should_increase = differences[0] > 0;

    for &diff in &differences {
        // Check if direction matches what it should be
        if (should_increase && diff <= 0) || (!should_increase && diff >= 0) {
            return false;
        }

        // Check if difference is between 1 and 3 (absolute value)
        if diff.abs() < 1 || diff.abs() > 3 {
            return false;
        }
    }

    true
}

fn is_safe_with_dampener(line: &str) -> bool {
    // Convert string numbers to integers
    let nums: Vec<i64> = line.split_whitespace()
        .filter_map(|x| x.parse().ok())
        .collect();

    // First check if safe without removing any level
    if is_safe_sequence(&nums) {
        return true;
    }

    // Try removing each level one at a time
    for i in 0..nums.len() {
        let mut dampened_nums = nums.clone();
        dampened_nums.remove(i);
        if is_safe_sequence(&dampened_nums) {
            return true;
        }
    }

    false
}

fn count_safe_reports(input: &str) -> i64 {
    input.lines()
        .filter(|line| !line.trim().is_empty())
        .filter(|line| is_safe_with_dampener(line))
        .count() as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let test_data = "7 6 4 2 1\n\
                        1 2 7 8 9\n\
                        9 7 6 2 1\n\
                        1 3 2 4 5\n\
                        8 6 4 4 1\n\
                        1 3 6 7 9";
        assert_eq!(count_safe_reports(test_data), 4);
    }
}