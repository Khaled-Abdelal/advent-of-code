use std::{char, fs, i32, usize};

fn main() {
    let input = read_input("input.txt");
    let count_xmax = xmas_count(&input);
    let count_x_max = x_mas_count(&input);
    println!("The count of xmax {}", count_xmax);
    println!("The count of x-max {}", count_x_max);
}

fn x_mas_count(input: &str) -> i32 {
    let input_as_grid: Vec<_> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect();

    let a_locations = find_char_locations(&input_as_grid, 'A');
    a_locations
        .iter()
        .map(|&location| check_complete_mas(location, &input_as_grid))
        .filter(|&val| val)
        .count()
        .try_into()
        .unwrap()
}

fn check_complete_mas(center_location: (i32, i32), grid: &Vec<Vec<char>>) -> bool {
    let row = center_location.1 as usize;
    let col = center_location.0 as usize;

    if row == 0 || row + 1 >= grid.len() || col == 0 || col + 1 >= grid[0].len() {
        return false;
    }

    let upper_left = grid[row - 1][col - 1];
    let upper_right = grid[row - 1][col + 1];
    let lower_left = grid[row + 1][col - 1];
    let lower_right = grid[row + 1][col + 1];

    let mas_pattern = |m: char, s: char| {
        (upper_left == m && upper_right == m && lower_left == s && lower_right == s)
            || (upper_left == s && upper_right == s && lower_left == m && lower_right == m)
            || (upper_left == s && lower_left == s && lower_right == m && upper_right == m)
            || (upper_right == s && lower_right == s && upper_left == m && lower_left == m)
    };
    mas_pattern('M', 'S') || mas_pattern('S', 'M')
}

fn xmas_count(input: &str) -> i32 {
    const WORD: &str = "XMAS";
    let input_as_grid: Vec<_> = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect();

    let x_locations = find_char_locations(&input_as_grid, 'X');
    let directions = [
        (1, 0),   // Down
        (0, 1),   // Right
        (-1, 0),  // Up
        (0, -1),  // Left
        (-1, -1), // Up-Left
        (1, 1),   // Down-Right
        (-1, 1),  // Up-Right
        (1, -1),  // Down-Left
    ];
    directions
        .iter()
        .map(|direction| {
            x_locations
                .iter()
                .map(|x_location| {
                    search_full_xmax_from(
                        (x_location.0 as i32, x_location.1 as i32),
                        &input_as_grid,
                        &WORD,
                        0 as i32,
                        &(direction.0 as i32, direction.1 as i32),
                    )
                })
                .sum::<i32>()
        })
        .sum()
}

fn search_full_xmax_from(
    location: (i32, i32),
    grid: &Vec<Vec<char>>,
    word: &str,
    current_char_in_word: i32,
    direction: &(i32, i32),
) -> i32 {
    if location.0 >= grid[0].len() as i32
        || location.1 >= grid.len() as i32
        || location.0 < 0
        || location.1 < 0
    {
        return 0;
    }
    if current_char_in_word >= word.len() as i32 {
        return 0;
    }
    let grid_char = grid[location.1 as usize][location.0 as usize];
    let word_char = word.chars().nth((current_char_in_word) as usize).unwrap();

    if grid_char != word_char {
        return 0;
    }

    if current_char_in_word == (word.len() - 1) as i32 && word_char == grid_char {
        return 1;
    }

    search_full_xmax_from(
        (location.0 + direction.0, location.1 + direction.1),
        &grid,
        &word,
        current_char_in_word + 1,
        direction,
    )
}

fn find_char_locations(grid: &Vec<Vec<char>>, target: char) -> Vec<(i32, i32)> {
    let mut result = Vec::new();
    for (i, line) in grid.iter().enumerate() {
        for (j, &c) in line.iter().enumerate() {
            if c == target {
                result.push((j as i32, i as i32));
            }
        }
    }
    result
}

fn read_input(file_path: &str) -> String {
    fs::read_to_string(file_path).expect("couldn't parse input file")
}
