use std::collections::HashMap;
use std::fs;

fn main() {
    let input = read_input("input.txt");
    let safe_count = count_safe(&input);
    let safe_count_tolerate_single_bad = count_safe_tolerate_single_bad(&input);
    println!("Number of safe resports are {:?}", safe_count);
    println!(
        "Number of safe resports when tolerating single bad {:?}",
        safe_count_tolerate_single_bad
    );
}

// totally not optimized
fn count_safe_tolerate_single_bad(input: &Vec<Vec<i32>>) -> i32 {
    let mut vec_map = HashMap::new();
    for (_, report) in input.iter().enumerate() {
        let hash_key: String = report
            .into_iter()
            .map(|num| num.to_string())
            .collect::<String>();

        let mut hash_vec = Vec::new();
        hash_vec.push(report.clone());

        for (j, _) in report.iter().enumerate() {
            let mut possibility_vec: Vec<i32> = Vec::new();
            for (x, _) in report.iter().enumerate() {
                if x != j {
                    possibility_vec.push(report[x]);
                }
            }
            hash_vec.push(possibility_vec);
        }

        _ = vec_map.insert(hash_key, hash_vec)
    }

    vec_map
        .into_iter()
        .map(|(_, val)| if count_safe(&val) > 0 { 1 } else { 0 })
        .sum()
}

fn count_safe(input: &Vec<Vec<i32>>) -> i32 {
    input
        .iter()
        .map(|report| {
            let mut safe = true;
            let mut increasing = true;
            for (i, ele) in report.iter().enumerate() {
                if i == 0 {
                    if ele - report[i + 1] > 0 {
                        increasing = false;
                    } else {
                        increasing = true;
                    }
                    continue;
                }

                let prev = report[i - 1];

                if (ele - prev).abs() > 3 || (ele - prev).abs() == 0 {
                    safe = false;
                    break;
                }

                if increasing && prev - ele > 0 {
                    safe = false;
                    break;
                }
                if !increasing && ele - prev > 0 {
                    safe = false;
                    break;
                }
            }

            if safe {
                1
            } else {
                0
            }
        })
        .sum()
}

fn read_input(input_file: &str) -> Vec<Vec<i32>> {
    let mut vec2d = Vec::new();
    fs::read_to_string(input_file)
        .expect("Couldn't parse input file")
        .lines()
        .for_each(|line| {
            vec2d.push(
                line.split_whitespace()
                    .map(|num| num.parse().unwrap())
                    .collect(),
            )
        });
    vec2d
}
