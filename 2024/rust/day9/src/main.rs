use std::fs;

fn main() {
    let input = read_input("input.txt");
    let check_sum = calculate_check_sum(&input);
    println!("The check sum is {:?}", check_sum);
}

fn calculate_check_sum(input: &Vec<Option<i64>>) -> i64 {
    let mut input_mut = input.clone();
    let mut left_pointer = 0;
    let mut right_pointer = input_mut.len() - 1;
    loop {
        if left_pointer >= right_pointer {
            break;
        }
        if input_mut[right_pointer].is_none() {
            right_pointer = right_pointer - 1;
            continue;
        }
        if input_mut[left_pointer].is_some() {
            left_pointer += 1;
            continue;
        }
        if input_mut[left_pointer].is_none() && input_mut[right_pointer].is_some() {
            input_mut[left_pointer] = input_mut[right_pointer];
            input_mut[right_pointer] = None;
        }
    }
    input_mut
        .iter()
        .enumerate()
        .filter_map(|(index, val)| val.map(|v| index as i64 * v))
        .sum()
}

fn read_input(file_path: &str) -> Vec<Option<i64>> {
    let mut result = Vec::new();
    let input = fs::read_to_string(file_path).expect("failed to read input");
    input.chars().enumerate().for_each(|(index, c)| {
        let c_as_int = c.to_digit(10);
        if let Some(c_as_int) = c_as_int {
            for _ in 0..c_as_int {
                if index % 2 == 0 {
                    result.push(Some((index / 2) as i64));
                } else {
                    result.push(None);
                }
            }
        }
    });
    result
}
