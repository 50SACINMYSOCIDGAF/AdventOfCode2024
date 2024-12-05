use std::collections::{HashMap, BinaryHeap};

pub fn solve_both(input: &str) -> (String, String) {
    // Split input into rules and updates sections
    let parts: Vec<&str> = input.split("\n\n").collect();
    if parts.len() < 2 {
        panic!("Invalid input format");
    }
    let rules_section = parts[0];
    let updates_section = parts[1];

    // Parse rules
    let mut rules: HashMap<u32, Vec<u32>> = HashMap::new();
    for rule in rules_section.lines() {
        let parts: Vec<&str> = rule.split('|').collect();
        if parts.len() != 2 {
            panic!("Invalid rule format");
        }
        let x: u32 = parts[0].parse().expect("Invalid page number");
        let y: u32 = parts[1].parse().expect("Invalid page number");
        rules.entry(x).or_insert(Vec::new()).push(y);
    }

    // Parse updates
    let updates: Vec<Vec<u32>> = updates_section
        .lines()
        .map(|line| {
            line.split(',')
                .map(|page| page.parse().expect("Invalid page number"))
                .collect()
        })
        .collect();

    // Initialize sums
    let mut sum_part1 = 0;
    let mut sum_part2 = 0;

    // Process each update
    for update in &updates {
        if update.len() % 2 == 0 {
            continue; // Skip even-length updates
        }

        if is_correctly_ordered(update, &rules) {
            // Correctly ordered, add to part one sum
            let middle_index = update.len() / 2;
            sum_part1 += update[middle_index];
        } else {
            // Not correctly ordered, reorder using topological sort
            match topological_sort(update, &rules) {
                Ok(sorted_update) => {
                    let middle_index = sorted_update.len() / 2;
                    sum_part2 += sorted_update[middle_index];
                }
                Err(_) => {
                    // Cycle detected, skip this update
                }
            }
        }
    }

    (sum_part1.to_string(), sum_part2.to_string())
}

fn is_correctly_ordered(update: &[u32], rules: &HashMap<u32, Vec<u32>>) -> bool {
    // Create position map
    let position_map: HashMap<u32, usize> = update.iter().enumerate().map(|(i, &p)| (p, i)).collect();

    // Check all relevant rules
    for &page in update {
        if let Some(after_pages) = rules.get(&page) {
            for &after_page in after_pages {
                if position_map.contains_key(&after_page) {
                    if position_map[&after_page] <= position_map[&page] {
                        return false;
                    }
                }
            }
        }
    }
    true
}

fn topological_sort(pages: &[u32], rules: &HashMap<u32, Vec<u32>>) -> Result<Vec<u32>, String> {
    // Build adjacency list and in-degree map
    let mut adj_list: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut in_degree: HashMap<u32, usize> = HashMap::new();

    for &page in pages {
        adj_list.insert(page, Vec::new());
        in_degree.insert(page, 0);
    }

    for &page in pages {
        if let Some(dependents) = rules.get(&page) {
            for &dep in dependents {
                if pages.contains(&dep) {
                    adj_list.get_mut(&page).unwrap().push(dep);
                    *in_degree.get_mut(&dep).unwrap() += 1;
                }
            }
        }
    }

    // Initialize heap with pages having zero in-degree
    let mut heap: BinaryHeap<u32> = pages.iter()
        .filter(|&&p| in_degree[&p] == 0)
        .cloned()
        .collect();

    let mut sorted: Vec<u32> = Vec::new();

    while let Some(node) = heap.pop() {
        sorted.push(node);
        for &neighbor in adj_list.get(&node).unwrap() {
            *in_degree.get_mut(&neighbor).unwrap() -= 1;
            if in_degree[&neighbor] == 0 {
                heap.push(neighbor);
            }
        }
    }

    // Check for cycles
    if sorted.len() != pages.len() {
        Err("Cycle detected in dependencies".to_string())
    } else {
        Ok(sorted)
    }
}