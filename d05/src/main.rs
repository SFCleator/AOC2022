use std::fs;


/// An order to move {amount} crates from the {from} stack to the {to} stack
#[derive(Debug)]
struct MoveOrder {
    amount: i32,
    from: i32,
    to: i32,
}

impl MoveOrder {
    pub fn new(order: &str) -> Self {
        let words: Vec<&str> = order.split(" ").collect();
        MoveOrder {
            amount: (*words.get(1).unwrap()).parse().unwrap(),
            from: (*words.get(3).unwrap()).parse().unwrap(),
            to: (*words.get(5).unwrap()).parse().unwrap(),
        }
    }
}

/// A depot containing all the stacks
#[derive(Debug)]
struct Depot {
    stacks: [Vec<String>; 9],
}

impl Depot {
    pub fn new(initial: Vec<&str>) -> Self {
        const VAL: Vec<String> = Vec::<String>::new();
        let mut stacks = [VAL; 9];

        for row in initial.into_iter().rev() {
            for column_idx in 0..9 {
                let cur_char = row.chars().nth(1 + 4 * column_idx).unwrap();
                if cur_char != ' ' {
                    stacks[column_idx].push(cur_char.to_string());
                }
            }
        }

        Depot { stacks: stacks }
    }

    pub fn move_crates_single(&mut self, order: MoveOrder) {
        let mut crane_stack = Vec::<String>::new();

        for _ in 0..order.amount {
            crane_stack.push(self.stacks[(order.from - 1) as usize].pop().unwrap());
            self.stacks[(order.to - 1) as usize].push(crane_stack.pop().unwrap());
        }
    }

    pub fn move_crates_multiple(&mut self, order: MoveOrder) {
        let mut crane_stack = Vec::<String>::new();

        for _ in 0..order.amount {
            crane_stack.push(self.stacks[(order.from - 1) as usize].pop().unwrap());
        }
        for _ in 0..order.amount {
            self.stacks[(order.to - 1) as usize].push(crane_stack.pop().unwrap());
        }
    }

    pub fn print_top(&self) {
        for stack_idx in 0..9 {
            print!(
                "{}",
                self.stacks[stack_idx]
                    .get(self.stacks[stack_idx].len() - 1)
                    .unwrap()
            );
        }
        println!("");
    }

    pub fn print(&self) {
        for stack_idx in 0..9 {
            if self.stacks[stack_idx].len() > 0 {
                print!("{}:", stack_idx + 1);
                for cur_box in &self.stacks[stack_idx] {
                    print!("{}", cur_box);
                }
                println!("");
            }
        }
    }
}

fn main() {
    // Read in the file lines to a vector
    let file_path = String::from("input.txt");
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let box_moves: Vec<&str> = contents.split("\n").collect();

    // Set up both depots of stacks
    let mut depot_9000 = Depot::new(box_moves[..8].to_vec());
    let mut depot_9001 = Depot::new(box_moves[..8].to_vec());

    // Move the crates for both depots 
    for box_move in &box_moves[9..] {
        let length: usize = (box_move.chars().count()).try_into().unwrap();
        if length == 0 {
            continue;
        }

        depot_9000.move_crates_single(MoveOrder::new(box_move));
        depot_9001.move_crates_multiple(MoveOrder::new(box_move));
    }

    // Print the final stacks
    println!("Crane 9000");
    depot_9000.print_top();
    depot_9000.print();
    println!("");

    println!("Crane 9001");
    depot_9001.print_top();
    depot_9001.print();
}
