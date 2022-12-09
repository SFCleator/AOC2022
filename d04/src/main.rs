use std::fs;

fn main() {
    // Read in the file lines to a vector
    let file_path = String::from("input.txt");
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let assignment_pairs: Vec<&str> = contents.split("\n").collect();

    let mut inner_lapping_value = 0;
    let mut over_lapping_value = 0;
    for assignment_pair in assignment_pairs {
        let length: usize = (assignment_pair.chars().count()).try_into().unwrap();
        if length == 0 {
            continue;
        }

        let assignment: Vec<&str> = assignment_pair.split(",").collect();

        let first_assignment: Vec<&str> = assignment.get(0).unwrap().split("-").collect();
        let first_begin: i32 = first_assignment.get(0).unwrap().parse().unwrap();
        let first_end: i32 = first_assignment.get(1).unwrap().parse().unwrap();

        let second_assignment: Vec<&str> = assignment.get(1).unwrap().split("-").collect();
        let second_begin: i32 = second_assignment.get(0).unwrap().parse().unwrap();
        let second_end: i32 = second_assignment.get(1).unwrap().parse().unwrap();

        let first_check = check_forward_internal(first_begin, first_end, second_begin, second_end);
        let second_check = check_forward_internal(second_begin, second_end, first_begin, first_end);
        if first_check || second_check {
            inner_lapping_value += 1;
        }

        let overlap_check = check_overlap(first_begin, first_end, second_begin, second_end);
        if overlap_check {
            over_lapping_value += 1;
        }
    }

    println!("Inner-lapping result: {inner_lapping_value}");
    println!("Over-lapping result: {over_lapping_value}");
}

fn check_forward_internal(
    first_begin: i32,
    first_end: i32,
    second_begin: i32,
    second_end: i32,
) -> bool {
    return (first_begin >= second_begin) && (first_end <= second_end);
}

fn check_overlap(first_begin: i32, first_end: i32, second_begin: i32, second_end: i32) -> bool {
    return (first_begin <= second_end) && (first_end >= second_begin);
}
