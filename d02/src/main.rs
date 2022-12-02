use std::fs;

fn main() {
    // Read in the file lines to a vector
    let file_path = String::from("input.txt");
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let games: Vec<&str> = contents.split("\n").collect();

    let mut total_score_first_guess = 0;
    let mut total_score_second_guess = 0;
    for cur_game in games {
        let moves: Vec<&str> = cur_game.split(" ").collect();

        // If we have an empty line just keep going
        if moves == [""] {
            continue;
        }   

        // Map moves to rock, paper and scissors
        let opponent_move = match moves.get(0).unwrap() {
            &"A" => "R",
            &"B" => "P",
            &"C" => "S",
            &&_ => todo!(),
        };

        // Calculate the score when using the first guess of what the code is
        let ego_move_first_guess = match moves.get(1).unwrap() {
            &"X" => "R",
            &"Y" => "P",
            &"Z" => "S",
            &&_ => todo!(),
        };
        total_score_first_guess +=
            choice_score(ego_move_first_guess) + match_score(opponent_move, ego_move_first_guess);

        // Get the move we need to make for the second guess of the code
        let ego_move_instruction = moves.get(1).unwrap();
        let ego_move_second_guess = match_move_for_outcome(opponent_move, ego_move_instruction);
        // Calculate the score for the second guess
        total_score_second_guess += choice_score(&ego_move_second_guess)
            + match_score(opponent_move, &ego_move_second_guess);
    }

    // Print results
    println!(
        "Total for the first code guess: {}",
        total_score_first_guess
    );
    println!(
        "Total for the second code guess: {}",
        total_score_second_guess
    );
}

/// The score due to the choice of move with 1 for Rock, 2 for Paper, and 3 for Scissors where
fn choice_score(choice: &str) -> i32 {
    return match choice {
        "R" => 1,
        "P" => 2,
        "S" => 3,
        &_ => todo!(),
    };
}

/// The score due to the RPS match where if you lost, 3 if the round was a draw, and 6 if you won
fn match_score(opponent_choice: &str, ego_choice: &str) -> i32 {
    // Draw
    if opponent_choice == ego_choice {
        return 3;
    }

    // We win
    if ((opponent_choice == "R") && (ego_choice == "P"))
        || ((opponent_choice == "P") && (ego_choice == "S"))
        || ((opponent_choice == "S") && (ego_choice == "R"))
    {
        return 6;
    }

    // We lose
    return 0;
}

/// Find the move the ego needs to perform to get the desired outcome where X means you need to
/// lose, Y means you need to end the round in a draw, and Z means you need to win
fn match_move_for_outcome(opponent_choice: &str, ego_instruction: &str) -> String {
    // Draw
    if ego_instruction == "Y" {
        return opponent_choice.to_string();
    }

    // We lose
    if ego_instruction == "X" {
        return match opponent_choice {
            "R" => "S".to_string(),
            "P" => "R".to_string(),
            "S" => "P".to_string(),
            &_ => todo!(),
        };
    }

    // We win
    if ego_instruction == "Z" {
        return match opponent_choice {
            "R" => "P".to_string(),
            "P" => "S".to_string(),
            "S" => "R".to_string(),
            &_ => todo!(),
        };
    }

    todo!()
}
