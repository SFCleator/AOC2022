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

        //let first = &backpack[..length/2].chars();
        //let second= &backpack[length/2..].chars();

        let mut common_items: Vec<String> = Vec::new();

        for first_char in backpack[0..length_half].chars() {
            'inner: for second_char in backpack[length_half..length_half*2].chars() {
                if common_items.contains(&first_char.to_string()) {
                    break 'inner;
                }
                if first_char == second_char {
                    common_items.push(first_char.to_string());
                    break 'inner;
                }
            }
        }

        // Cheating...
        let alphabet: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();


        for cur_char in &common_items {
            for (idx, cur_alphabet) in alphabet.iter().enumerate(){
                if cur_alphabet.to_string() == *cur_char {
                    value += idx + 1;
                    break
                }
            }
        }

        println!("{} {:?} {}",  backpack, common_items, value);
    }

    println!("Result: {}", value);
}
