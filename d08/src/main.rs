use std::fs;

fn main() {
    let file_path = String::from("input.txt");
    let contents = fs::read_to_string(file_path).expect("Cannie read");
    let rows: Vec<&str> = contents.split("\n").collect();

    let height = rows.len() - 1;
    let width = rows[0].len();

    let mut tree_grid = vec![vec![0; width]; height];

    let mut row_idx = 0;
    let mut column_idx = 0;
    for row in rows {
        if row == "" {
            continue;
        }

        let columns: Vec<&str> = row.split("").collect();
        for value in columns {
            if value == "" {
                continue;
            }

            tree_grid[row_idx][column_idx] = (*value).to_string().parse().unwrap();
            column_idx += 1;
        }

        column_idx = 0;
        row_idx += 1;
    }
    
    let from_top= check_vis(&tree_grid,&|x,y|(x,y)); 
    let from_bot= check_vis(&tree_grid,&|x,y|(height-x-1,y)); 
    let from_left= check_vis(&tree_grid,&|x,y|(y,x)); 
    let from_right= check_vis(&tree_grid,&|x,y|(y,height-x-1)); 

    let mut comb_vis = create_vis_grid(width, height);

    for r in 0..height-1 {
        for c in 0..width-1 {
            if from_top[r][c] + from_bot[r][c] + from_left[r][c] + from_right[r][c] > 0 {
                comb_vis[r][c] = 1;
            }
        }
    }

    let mut total_vis =0;
    for r in 0..height {
        for c in 0..width {
            total_vis += comb_vis[r][c];
        }
    }

    print_grid(&comb_vis);
    println!("Total visible trees: {}", total_vis);
}

fn print_grid(grid: &Vec<Vec<i32>>) {
    for row in grid.iter() {
        for val in row.iter() {
            print!("{}", val);
        }
        println!("");
    }
}

fn create_vis_grid(width: usize, height: usize) -> Vec<Vec<i32>> {
    let mut vis_grid = vec![vec![0; width]; height];

    // All the trees on the outside are visible
    for vis_row_idx in 0..height {
        for vis_col_idx in 0..width {
            if vis_row_idx == 0
                || vis_row_idx == height - 1
                || vis_col_idx == 0
                || vis_col_idx == width - 1
            {
                vis_grid[vis_row_idx][vis_col_idx] = 1;
            }
        }
    }

    return vis_grid;
}

fn check_vis(tree_grid: &Vec<Vec<i32>>, rotate: &dyn Fn(usize, usize) -> (usize, usize) ) -> Vec<Vec<i32>> {
    let height = tree_grid.len();
    let width = tree_grid[0].len();

    let mut from_top_vis_grid = create_vis_grid(width, height);

    //TODO: This only works if width==height? 
    //let (x_max, y_max) = rotate(height-1, width-1);
    let (x_max, y_max) = (height-1,width-1);

    for vis_col_idx in 1..y_max {

        let (start_x, start_y) = rotate(0, vis_col_idx);

        let mut max_in_line = tree_grid[start_x][start_y];
        for vis_row_idx in 1..x_max {
            let (x,y) = rotate(vis_row_idx, vis_col_idx);

            if tree_grid[x][y] > max_in_line {
                from_top_vis_grid[x][y] = 1;
                max_in_line = tree_grid[x][y];
            }
        }
    }

    return from_top_vis_grid;
}
