// src/solutions/s03.rs

#[derive(Debug)]
struct Multiplication {
    x: i64,
    y: i64,
}

impl Multiplication {
    fn result(&self) -> i64 {
        self.x * self.y
    }
}

pub fn solve(input: &str) -> i64 {
    solve_both(input).1
}

pub fn solve_both(input: &str) -> (i64, i64) {
    let part1 = process_memory(input, false);
    let part2 = process_memory(input, true);
    (part1, part2)
}

fn process_memory(input: &str, handle_conditionals: bool) -> i64 {
    let mut total = 0;
    let mut enabled = true;
    let mut i = 0;
    let chars: Vec<char> = input.chars().collect();

    while i < chars.len() {
        if handle_conditionals {
            if let Some(new_i) = check_conditional(&chars[i..], &mut enabled) {
                i += new_i;
                continue;
            }
        }

        if let Some((mult, new_i)) = parse_multiplication(&chars[i..]) {
            if !handle_conditionals || enabled {
                total += mult.result();
            }
            i += new_i;
        } else {
            i += 1;
        }
    }

    total
}

fn check_conditional(chars: &[char], enabled: &mut bool) -> Option<usize> {
    let content: String = chars.iter().collect();

    if content.starts_with("do()") {
        *enabled = true;
        Some(4)
    } else if content.starts_with("don't()") {
        *enabled = false;
        Some(7)
    } else {
        None
    }
}

fn parse_multiplication(chars: &[char]) -> Option<(Multiplication, usize)> {
    let content: String = chars.iter().collect();

    // Check if it starts with "mul("
    if !content.starts_with("mul(") {
        return None;
    }

    let mut i = 4; // Skip "mul("
    let mut num_str = String::new();
    let mut x = None;
    let mut y = None;

    while i < chars.len() {
        match chars[i] {
            '0'..='9' => num_str.push(chars[i]),
            ',' => {
                if let Ok(num) = num_str.parse() {
                    x = Some(num);
                    num_str.clear();
                } else {
                    return None;
                }
            }
            ')' => {
                if let Ok(num) = num_str.parse() {
                    y = Some(num);
                    if let (Some(x_val), Some(y_val)) = (x, y) {
                        return Some((
                            Multiplication { x: x_val, y: y_val },
                            i + 1
                        ));
                    }
                }
                return None;
            }
            _ => return None,
        }
        i += 1;
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(process_memory(input, false), 161);
    }

    #[test]
    fn test_part2() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)do()?mul(8,5))";
        assert_eq!(process_memory(input, true), 48);
    }
}