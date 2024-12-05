use std::{cmp::Ordering, collections::HashMap, fs};

fn main() {
    let (rules, reports) = read_input("input.txt");
    let count_valid = find_sum_valid_reports_middle(&rules, &reports);
    let count_invalid = find_sum_invalid_reports_middle(&rules, &reports);
    println!(
        "The sum of middle values in valid reports is {}",
        count_valid
    );
    println!(
        "The sum of middle values in non valid reports after ordering is {}",
        count_invalid
    );
}

#[derive(Default, Debug)]
struct PageRuleHolder {
    // those numbers must be after the number for example 75|47 => number 75 will have 47 in must_be_after
    must_be_after: Vec<i32>,
    must_be_before: Vec<i32>,
}

fn find_sum_invalid_reports_middle(rules: &Vec<(i32, i32)>, reports: &Vec<Vec<i32>>) -> i32 {
    let mut rules_as_map: HashMap<&i32, PageRuleHolder> = HashMap::new();
    rules.iter().for_each(|(before, after)| {
        let holder_before = rules_as_map.entry(before).or_default();
        holder_before.must_be_after.push(*after);

        let holder_after = rules_as_map.entry(after).or_default();
        holder_after.must_be_before.push(*before);
    });

    reports
        .iter()
        .filter(|report| {
            for (i, item) in report.iter().enumerate() {
                let elements_before = report[..i].to_vec();
                let elements_after = report[i + 1..].to_vec();
                let element_rules = rules_as_map.get(item);
                if element_rules.is_none() {
                    continue;
                }
                if !check_element_valid(&element_rules.unwrap(), &elements_before, &elements_after)
                {
                    return true;
                }
            }
            return false;
        })
        .map(|report| {
            let mut sorted_report = report.clone();
            sorted_report.sort_by(|a, b| {
                if rules_as_map.get(a).unwrap().must_be_after.contains(b) {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            });
            sorted_report
        })
        .map(|report| report[report.len() / 2])
        .sum()
}

fn find_sum_valid_reports_middle(rules: &Vec<(i32, i32)>, reports: &Vec<Vec<i32>>) -> i32 {
    let mut rules_as_map: HashMap<&i32, PageRuleHolder> = HashMap::new();
    rules.iter().for_each(|(before, after)| {
        let holder_before = rules_as_map.entry(before).or_default();
        holder_before.must_be_after.push(*after);

        let holder_after = rules_as_map.entry(after).or_default();
        holder_after.must_be_before.push(*before);
    });

    reports
        .iter()
        .filter(|report| {
            for (i, item) in report.iter().enumerate() {
                let elements_before = report[..i].to_vec();
                let elements_after = report[i + 1..].to_vec();
                let element_rules = rules_as_map.get(item);
                if element_rules.is_none() {
                    continue;
                }
                if !check_element_valid(&element_rules.unwrap(), &elements_before, &elements_after)
                {
                    return false;
                }
            }
            return true;
        })
        .map(|report| report[report.len() / 2])
        .sum()
}

fn check_element_valid(rules: &PageRuleHolder, before: &Vec<i32>, after: &Vec<i32>) -> bool {
    after
        .iter()
        .all(|elem| !rules.must_be_before.contains(elem))
        && before
            .iter()
            .all(|elem| !rules.must_be_after.contains(elem))
}

fn read_input(file_path: &str) -> (Vec<(i32, i32)>, Vec<Vec<i32>>) {
    let input = fs::read_to_string(file_path).expect("failed to parse input file");
    let mut rules = Vec::new();
    let mut reports = Vec::new();
    input.lines().for_each(|line| {
        if line.contains("|") {
            let parts: Vec<i32> = line.split("|").map(|num| num.parse().unwrap()).collect();
            rules.push((parts[0], parts[1]));
        }
        if line.contains(",") {
            // assumeing no reports with one value
            let report = line
                .split(",")
                .map(|num| num.parse().unwrap())
                .collect::<Vec<i32>>();
            reports.push(report);
        }
    });
    (rules, reports)
}
