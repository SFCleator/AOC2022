use std::fs;

fn main() {
    // Read in the file lines to a vector
    let file_path = String::from("input.txt");
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let calories: Vec<&str> = contents.split("\n").collect();

    let mut elf_calories = Vec::new();

    // Calculate the max of the blocks
    let mut cur_total = 0;
    let mut max_total = 0;
    for cur_calorie in calories {
        let calorie_result = cur_calorie.parse::<i32>();

        // If there was a problem reading the line it was likely an empty
        let calorie = match calorie_result {
            Ok(value) => value,
            Err(_error) => -1,
        };

        // Add more calories or research for a new elf
        if calorie >= 0 {
            cur_total += calorie;
        } else {
            elf_calories.push(cur_total);

            if cur_total > max_total {
                max_total = cur_total;
            }
            cur_total = 0;
        }
    }
    println!("");
    println!("Max Value: {}", max_total);

    // Get the largest 3 values
    elf_calories.sort();
    let vec_len = elf_calories.len();
    let largest_3 = elf_calories.get((vec_len - 3)..vec_len).unwrap();

    // Sum the largest three together
    let mut sum_largest_3 = 0;
    for elf_cal in largest_3 {
        sum_largest_3 += elf_cal;
    }
    println!("Sum of the largest three: {}", sum_largest_3);
}
