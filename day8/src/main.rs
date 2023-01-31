use std::{fs, env};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    
    let mut tree_heights = [[0i32; 99]; 99];

    for (row, line) in contents.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            tree_heights[col][row] = c.to_digit(10).unwrap() as i32;
        }
    }

    let mut visible_trees = [[false; 99]; 99];

    for x in 0..99 {
        let mut heighest = -1;
        for y in 0..99 {
            if tree_heights[x][y] > heighest {
                heighest = tree_heights[x][y];
                visible_trees[x][y] = true;
            }
        }
        let mut heighest = -1;
        for y in (0..99).rev() {
            if tree_heights[x][y] > heighest {
                heighest = tree_heights[x][y];
                visible_trees[x][y] = true;
            }
        }
    }

    for y in 0..99 {
        let mut heighest = -1;
        for x in 0..99 {
            if tree_heights[x][y] > heighest {
                heighest = tree_heights[x][y];
                visible_trees[x][y] = true;
            }
        }
        let mut heighest = -1;
        for x in (0..99).rev() {
            if tree_heights[x][y] > heighest {
                heighest = tree_heights[x][y];
                visible_trees[x][y] = true;
            }
        }
    }

    let mut total_visible = 0;
    for x in 0..99 {
        for y in 0..99 {
            if visible_trees[x][y] {
                total_visible += 1;
            }
        }
    }

    dbg!(total_visible);
}