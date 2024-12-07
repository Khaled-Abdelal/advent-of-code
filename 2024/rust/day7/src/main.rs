use std::fs;

fn main() {
    let input = read_input("input.txt");
    let count = calculate_calibration_result(&input);
    let count_with_new_operator = calculate_calibration_result_with_new_operator(&input);
    println!("The sum of results is {:?}", count);
    println!(
        "The sum of results with new operator is {:?}",
        count_with_new_operator
    );
}

fn calculate_calibration_result_with_new_operator(input: &Vec<(i64, Vec<i64>)>) -> i64 {
    input
        .iter()
        .filter_map(|(sum, vals)| {
            if check_valid_with_new_operator(*sum, vals, 0, 0) {
                return Some(sum);
            }
            None
        })
        .sum()
}

fn check_valid_with_new_operator(
    target: i64,
    vals: &Vec<i64>,
    current_sum: i64,
    current_val_index: usize,
) -> bool {
    if current_sum == target && current_val_index == vals.len() {
        return true;
    }
    if current_sum > target || current_val_index == vals.len() {
        return false;
    }

    check_valid_with_new_operator(
        target,
        vals,
        current_sum + vals[current_val_index],
        current_val_index + 1,
    ) || check_valid_with_new_operator(
        target,
        vals,
        if current_sum == 0 {
            vals[current_val_index]
        } else {
            current_sum * vals[current_val_index]
        },
        current_val_index + 1,
    ) || check_valid_with_new_operator(
        target,
        vals,
        if current_sum == 0 {
            vals[current_val_index]
        } else {
            [current_sum.to_string(), vals[current_val_index].to_string()]
                .join("")
                .parse()
                .unwrap()
        },
        current_val_index + 1,
    )
}

fn calculate_calibration_result(input: &Vec<(i64, Vec<i64>)>) -> i64 {
    input
        .iter()
        .filter_map(|(sum, vals)| {
            if check_valid(*sum, vals, 0, 0) {
                return Some(sum);
            }
            None
        })
        .sum()
}

fn check_valid(target: i64, vals: &Vec<i64>, current_sum: i64, current_val_index: usize) -> bool {
    if current_sum == target && current_val_index == vals.len() {
        return true;
    }
    if current_sum > target || current_val_index == vals.len() {
        return false;
    }

    check_valid(
        target,
        vals,
        current_sum + vals[current_val_index],
        current_val_index + 1,
    ) || check_valid(
        target,
        vals,
        if current_sum == 0 {
            vals[current_val_index]
        } else {
            current_sum * vals[current_val_index]
        },
        current_val_index + 1,
    )
}

fn read_input(file_path: &str) -> Vec<(i64, Vec<i64>)> {
    fs::read_to_string(file_path)
        .expect("failed to read input")
        .lines()
        .enumerate()
        .map(|(_, line)| {
            let sum_value = line.split(':').collect::<Vec<_>>()[0].parse().unwrap();
            let nums = line.split(':').collect::<Vec<_>>()[1]
                .split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect::<Vec<_>>();

            (sum_value, nums)
        })
        .collect()
}
