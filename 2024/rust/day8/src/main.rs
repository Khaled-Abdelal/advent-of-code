use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn main() {
    let (input, width) = read_input("test.txt");
    let count = count_antinode(&input, width);
    println!("The count of unique antinode is {}", count);
}

fn count_antinode(input: &HashMap<char, Vec<(i32, i32)>>, width: i32) -> i32 {
    let mut result = HashSet::new();
    for (key, v) in input.iter() {
        for antena_point in v.iter() {
            for second_antena_point in v.iter() {
                if antena_point == second_antena_point {
                    continue;
                }
                let antinodes = calculate_antinodes(antena_point, second_antena_point, width);
                if let Some(antinodes) = antinodes {
                    antinodes.into_iter().for_each(|antinode| {
                        result.insert((key, antinode));
                    });
                }
            }
        }
    }
    result.len() as i32
}

fn calculate_antinodes(
    point: &(i32, i32),
    second_point: &(i32, i32),
    width: i32,
) -> Option<Vec<(i32, i32)>> {
    let row_diff = point.1 - second_point.1;
    let col_diff = point.0 - second_point.0;
    let mut result = Vec::new();

    let mut antinode_one = (0, 0);
    let mut antinode_two = (0, 0);

    match row_diff.signum() {
        0 => {
            antinode_one.1 = point.1;
            antinode_two.1 = point.1;
        }
        1 => {
            antinode_one.1 = point.1 + row_diff.abs();
            antinode_two.1 = second_point.1 - row_diff.abs();
        }
        -1 => {
            antinode_one.1 = point.1 - row_diff.abs();
            antinode_two.1 = second_point.1 + row_diff.abs();
        }
        _ => {
            println!("Why am I here");
        }
    }
    match col_diff.signum() {
        0 => {
            antinode_one.0 = point.0;
            antinode_two.0 = point.0;
        }
        1 => {
            antinode_one.0 = point.0 + col_diff.abs();
            antinode_two.0 = second_point.0 - col_diff.abs();
        }
        -1 => {
            antinode_one.0 = point.0 - col_diff.abs();
            antinode_two.0 = second_point.0 + col_diff.abs();
        }
        _ => {
            println!("Why am I here");
        }
    }
    if antinode_one.0 > 0 && antinode_one.0 < width && antinode_one.1 > 0 && antinode_one.1 < width
    {
        result.push(antinode_one);
    }
    if antinode_two.0 > 0 && antinode_two.0 < width && antinode_two.1 > 0 && antinode_two.1 < width
    {
        result.push(antinode_two);
    }
    if result.len() > 0 {
        return Some(result);
    }
    None
}

fn read_input(file_path: &str) -> (HashMap<char, Vec<(i32, i32)>>, i32) {
    let mut result = HashMap::new();
    let mut width = 0;
    fs::read_to_string(file_path)
        .expect("failed to parse input")
        .lines()
        .enumerate()
        .for_each(|(row, line)| {
            width = row as i32;
            line.char_indices().for_each(|(col, c)| {
                if c != '.' {
                    let v: &mut Vec<(i32, i32)> = result.entry(c).or_default();
                    v.push((col as i32, row as i32));
                }
            });
        });
    (result, width)
}
