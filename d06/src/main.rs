use std::collections::HashSet;
use std::fs;

fn main() {
    let file_path = String::from("sample_input.txt");
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let message: Vec<&str> = contents.split("").collect();

    println!("Marker start: {}", get_marker_idx(&message, 4));
    println!("Message start: {}", get_marker_idx(&message, 14));
}

fn get_marker_idx(message: &Vec<&str>, marker_size: usize) -> i32 {
    for idx in 1..(message.len() - marker_size - 1) {
        let pattern = &message[idx..idx + marker_size];

        let mut set = HashSet::new();

        for letter in pattern {
            set.insert(letter);
        }

        if set.len() == pattern.len() {
            return (idx + marker_size - 1).try_into().unwrap();
        }
    }
    return -1;
}
