use std::collections::HashMap;
use std::fs;

fn main() {
    let (list_one, list_two) = read_input("input.txt");
    let distance = find_total_distance(&list_one, &list_two);
    let similarity = find_similarity(&list_one, &list_two);
    println!("The total distance is: {}", distance);
    println!("The similarity is: {}", similarity);
}

fn find_total_distance(list_one: &[i32], list_two: &[i32]) -> i32 {
    let mut sorted_one = list_one.to_vec();
    let mut sorted_two = list_two.to_vec();
    sorted_one.sort();
    sorted_two.sort();

    sorted_one
        .iter()
        .zip(&sorted_two)
        .map(|(x, y)| (x - y).abs())
        .sum()
}

fn find_similarity(list_one: &Vec<i32>, list_two: &Vec<i32>) -> i32 {
    let mut counter_of_two = HashMap::new();
    for num in list_two {
        let count = counter_of_two.entry(num).or_insert(0);
        *count += 1;
    }
    list_one
        .iter()
        .map(|num| counter_of_two.get(num).copied().unwrap_or(0) * num)
        .sum()
}

fn read_input(file_path: &str) -> (Vec<i32>, Vec<i32>) {
    let input = fs::read_to_string(file_path).expect("Unable to read the input");
    let mut list_one: Vec<i32> = Vec::new();
    let mut list_two: Vec<i32> = Vec::new();

    for line in input.lines() {
        let input: Vec<i32> = line
            .split_whitespace()
            .map(|num| num.parse().expect("Invalid input number"))
            .collect();

        if input.len() != 2 {
            panic!("Invalid input format");
        }
        list_one.push(input[0]);
        list_two.push(input[1]);
    }

    (list_one, list_two)
}
