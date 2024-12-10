use std::{collections::HashSet, fs, i32, usize};

fn main() {
    let input = read_input("input.txt");
    let sum = get_sum(&input);
    println!("The sum is {:?}", sum);
}

fn get_sum(grid: &Vec<Vec<i32>>) -> i32 {
    let start_points: Vec<(i32, i32)> = grid
        .iter()
        .enumerate()
        .flat_map(move |(row_index, line)| {
            line.iter()
                .enumerate()
                .filter_map(move |(col_index, &val)| {
                    if val == 0 {
                        Some((col_index as i32, row_index as i32))
                    } else {
                        None
                    }
                })
        })
        .collect();
    let mut sum = 0;
    for start_point in start_points {
        let mut local_unique_heads = HashSet::new();
        get_point_trails(grid, start_point, 0, &mut local_unique_heads);
        sum += local_unique_heads.len();
    }
    return sum as i32;
}

fn get_point_trails(
    grid: &Vec<Vec<i32>>,
    (col, row): (i32, i32),
    step: i32,
    unique_heads: &mut HashSet<(i32, i32)>,
) {
    if col < 0 || row < 0 || row as usize >= grid.len() || col as usize >= grid[0].len() {
        return;
    }
    if grid[row as usize][col as usize] != step {
        return;
    }
    if grid[row as usize][col as usize] as i32 == 9 && step == 9 {
        unique_heads.insert((col, row));
        return;
    }
    let sides = [(0, 1), (1, 0), (-1, 0), (0, -1)];
    for side in sides {
        get_point_trails(grid, (col + side.0, row + side.1), step + 1, unique_heads);
    }
}

fn read_input(file_path: &str) -> Vec<Vec<i32>> {
    let input = fs::read_to_string(file_path).expect("failed to parse input");
    // let mut result = Vec::new();
    input
        .lines()
        .enumerate()
        .map(|(_, line)| {
            line.chars()
                .enumerate()
                .map(|(_, c)| c.to_digit(10).unwrap() as i32)
                .collect()
        })
        .collect()
}
