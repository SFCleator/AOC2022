use std::collections::HashSet;
use std::fs;

fn main() {
    // Read in the file lines to a vector
    let file_path = String::from("input.txt");
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let backpacks: Vec<&str> = contents.split("\n").collect();

    let mut value = 0;
    for backpack in &backpacks {
        let length_half: usize = (backpack.chars().count() / 2).try_into().unwrap();

        // If we have an empty line just keep going
        if length_half == 0 {
            continue;
        }

        let in_backpack_common_items =
            dual_common(backpack, 0, length_half, length_half, length_half * 2);

        value += score_items(&in_backpack_common_items);
    }
    println!("Wrong backpack result: {value}");

    let mut team_score = 0;
    for idx in 0..backpacks.len() {
        let start_idx = idx * 3;

        // Ignore if we're starting with an empty string
        match backpacks.get(start_idx) {
            Some(x) => {
                if *x == "" {
                    continue;
                }
            }
            None => continue,
        }

        // Get all the sets of interest
        let first_set: HashSet<char> = backpacks
            .get(start_idx)
            .unwrap()
            .to_string()
            .chars()
            .collect();
        let second_set: HashSet<char> = backpacks
            .get(start_idx + 1)
            .unwrap()
            .to_string()
            .chars()
            .collect();
        let third_set: HashSet<char> = backpacks
            .get(start_idx + 2)
            .unwrap()
            .to_string()
            .chars()
            .collect();

        // Find their intersection
        let first_intersection: HashSet<char> =
            first_set.intersection(&second_set).cloned().collect();
        let second_intersection: HashSet<char> = first_intersection
            .intersection(&third_set)
            .cloned()
            .collect();

        let alphabet: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
            .chars()
            .collect();
        for (idx, cur_alphabet) in alphabet.iter().enumerate() {
            if second_intersection.contains(cur_alphabet) {
                team_score += idx + 1;
                break;
            }
        }
    }

    println!("Team score: {team_score}");
}

fn dual_common(
    items: &str,
    first_start: usize,
    first_end: usize,
    second_start: usize,
    second_end: usize,
) -> Vec<String> {
    let mut common: Vec<String> = Vec::new();

    for first_char in items[first_start..first_end].chars() {
        'inner: for second_char in items[second_start..second_end].chars() {
            if common.contains(&first_char.to_string()) {
                break 'inner;
            }
            if first_char == second_char {
                common.push(first_char.to_string());
                break 'inner;
            }
        }
    }

    return common;
}

fn score_items(items: &Vec<String>) -> usize {
    let mut score = 0;
    let alphabet: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .chars()
        .collect();
    for cur_char in items {
        for (idx, cur_alphabet) in alphabet.iter().enumerate() {
            if cur_alphabet.to_string() == *cur_char {
                score += idx + 1;
                break;
            }
        }
    }

    return score;
}
