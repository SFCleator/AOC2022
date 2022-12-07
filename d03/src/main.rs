use std::fs;

fn main() {
    // Read in the file lines to a vector
    let file_path = String::from("input.txt");
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let backpacks: Vec<&str> = contents.split("\n").collect();

    let mut value =0;
    for backpack in backpacks {
        let length_half: usize = (backpack.chars().count()/2).try_into().unwrap();

        // If we have an empty line just keep going
        if length_half ==0 {
            continue
        }

        let mut in_backpack_common_items = dual_common(backpack, 0, length_half,length_half, length_half*2) ;

        value += score_items(&in_backpack_common_items );

        //println!("{} {:?} {}",  backpack, in_backpack_common_items, value);
    }
    //println!("Result: {}", value);

    for idx in 0..backpacks.len() {
        let start_idx = idx*3;

        let  mut dual_backpack: Vec<String>= Vec::new();
        dual_backpack.push_str(&backpacks.get(start_idx).unwrap().to_string());
        dual_backpack.push_str(&backpacks.get(start_idx+1).unwrap().to_string());

        let first_string_size: usize = backpacks.get(start_idx).unwrap().len();

        let first_two_compare = dual_common(dual_backpack, 0, first_string_size,first_string_size, dual_backpack.len());

        println!("{} {:?}", dual_backpack, first_two_compare);

    }

}

fn dual_common(items: &str, first_start: usize, first_end: usize, second_start: usize, second_end:usize) -> Vec<String> {
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

fn score_items(items: &Vec<String>) -> usize{
    let mut score =0;
    let alphabet: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
    for cur_char in items{
        for (idx, cur_alphabet) in alphabet.iter().enumerate(){
            if cur_alphabet.to_string() == *cur_char {
                score += idx + 1;
                break
            }
        }
    }

    return score;
}
