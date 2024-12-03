use regex::Regex;
use std::{fs, i32};

fn main() {
    let input = read_input("input.txt");
    let count = correct_and_count(&input);
    let count_respect_commands = correct_and_count_respecting_commands(&input);
    println!("Output of valid mul is: {:?}", count);
    println!(
        "Output of valid mul respecting commands is: {:?}",
        count_respect_commands
    );
}

fn correct_and_count_respecting_commands(s: &str) -> i32 {
    let pattern = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut do_match: Vec<_> = s.match_indices("do()").map(|(pos, _)| pos).collect();
    let donot_match: Vec<_> = s.match_indices("don't()").map(|(pos, _)| pos).collect();

    do_match.insert(0, 0);

    pattern
        .captures_iter(s)
        .map(|cap| {
            let match_start = cap.get(0).unwrap().start();

            let nearest_do = do_match
                .iter()
                .rev()
                .find(|&&pos| pos <= match_start)
                .copied()
                .unwrap_or(0);

            let nearest_donot = donot_match
                .iter()
                .rev()
                .find(|&&pos| pos <= match_start)
                .copied()
                .unwrap_or(0);

            if nearest_do < nearest_donot {
                return 0;
            }

            let fist_number: i32 = cap.get(1).unwrap().as_str().parse().unwrap();
            let second_number: i32 = cap.get(2).unwrap().as_str().parse().unwrap();
            fist_number * second_number
        })
        .sum()
}

fn correct_and_count(s: &str) -> i32 {
    let pattern = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    pattern
        .captures_iter(s)
        .map(|cap| {
            let fist_number: i32 = cap.get(1).unwrap().as_str().parse().unwrap();
            let second_number: i32 = cap.get(2).unwrap().as_str().parse().unwrap();
            fist_number * second_number
        })
        .sum()
}

fn read_input(file_path: &str) -> String {
    fs::read_to_string(file_path)
        .expect("could't read input")
        .trim()
        .to_string()
}
