use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn main() {
    let (input, width) = read_input("input.txt");
    let count = count_antinode(&input, width);
    let count_with_harmonics = count_antinode_with_harmonics(&input, width);
    println!("The count of unique antinode is {}", count);
    println!(
        "The count of unique antinode with harmonics is {}",
        count_with_harmonics
    );
}

fn count_antinode_with_harmonics(input: &HashMap<char, Vec<(i32, i32)>>, width: i32) -> i32 {
    let mut result = HashSet::new();
    for (_, v) in input.iter() {
        for antena_point in v.iter() {
            for second_antena_point in v.iter() {
                if antena_point == second_antena_point {
                    continue;
                }
                let antinodes =
                    calculate_antinodes_with_harmonics(antena_point, second_antena_point, width);
                if let Some(antinodes) = antinodes {
                    antinodes.into_iter().for_each(|antinode| {
                        result.insert(antinode);
                    });
                }
            }
        }
    }
    result.len() as i32
}

fn calculate_antinodes_with_harmonics(
    point: &(i32, i32),
    second_point: &(i32, i32),
    width: i32,
) -> Option<Vec<(i32, i32)>> {
    let row_diff = point.1 - second_point.1;
    let col_diff = point.0 - second_point.0;
    let mut result = Vec::new();

    for i in 0..width {
        let new_point_one = (point.0 - i * col_diff, point.1 - row_diff * i);
        if is_in_bound(&new_point_one, width) {
            result.push(new_point_one);
        }
        let new_point_two = (second_point.0 + col_diff * i, second_point.1 + row_diff * i);
        if is_in_bound(&new_point_two, width) {
            result.push(new_point_two);
        }
    }

    if result.len() > 0 {
        return Some(result);
    }
    None
}

fn is_in_bound(node: &(i32, i32), width: i32) -> bool {
    if node.0 >= 0 && node.0 < width && node.1 >= 0 && node.1 < width {
        true
    } else {
        false
    }
}

fn count_antinode(input: &HashMap<char, Vec<(i32, i32)>>, width: i32) -> i32 {
    let mut result = HashSet::new();
    for (_, v) in input.iter() {
        for antena_point in v.iter() {
            for second_antena_point in v.iter() {
                if antena_point == second_antena_point {
                    continue;
                }
                let antinodes = calculate_antinodes(antena_point, second_antena_point, width);
                if let Some(antinodes) = antinodes {
                    antinodes.into_iter().for_each(|antinode| {
                        result.insert(antinode);
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

    let antinode_one = (point.0 + col_diff, point.1 + row_diff);
    let antinode_two = (second_point.0 - col_diff, second_point.1 - row_diff);

    if is_in_bound(&&antinode_one, width) {
        result.push(antinode_one);
    }
    if is_in_bound(&&antinode_two, width) {
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
            width = (row + 1) as i32;
            line.char_indices().for_each(|(col, c)| {
                if c != '.' {
                    let v: &mut Vec<(i32, i32)> = result.entry(c).or_default();
                    v.push((col as i32, row as i32));
                }
            });
        });
    (result, width)
}
