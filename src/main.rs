use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;

fn read_input_data() -> io::Result<Vec<Vec<char>>>{
    let f = File::open("inputData.txt")?;
    let f = BufReader::new(f);

    let mut grid:Vec<Vec<char>> = Vec::new();

    for line in f.lines() {
        let mut symbol_line:Vec<char> = Vec::new();
        for c in line.unwrap().chars() {
            symbol_line.push(c);
        }
        grid.push(symbol_line);
    }

    Ok(grid)
}

fn simple_solution_part_1(slope: (usize, usize)) -> i32 {
    let grid = read_input_data().unwrap();
    let height = grid.len();
    let width = grid[0].len();
    let mut tree_encounters: i32 = 0;
    let mut pos: (usize, usize) = (0, 0);
    while pos.0 < height {
        if grid[pos.0][pos.1] == '#' {
            tree_encounters += 1;
        }
        pos.0 += slope.1;
        pos.1 = (pos.1 + slope.0) % width;
    }
    tree_encounters
}

fn simple_solution_part_2() -> i64 {
    let slopes:Vec<(usize, usize)> = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let mut product: i64 = 1;
    for slope in slopes {
        product *= simple_solution_part_1(slope) as i64;
    }
    product
}

fn main() {
    println!("{}", simple_solution_part_1((3,1)));
    println!("{}", simple_solution_part_2());
}
