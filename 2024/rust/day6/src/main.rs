use std::{collections::HashSet, fs};

fn main() {
    let (start_position, width, blockers) = read_input("input.txt");
    let visited_positions = count_distinct_visisted_positions(&start_position, width, &blockers);
    let blockers_infinite = count_blockers_loop(&start_position, width, &blockers);
    println!("Count of visited_positions is {:?}", visited_positions);
    println!(
        "Count of blocker to get infinite is  {:?}",
        blockers_infinite
    );
}

const DIRECTIONS: [(i32, i32); 4] = [
    (0, -1), // up
    (1, 0),  // right
    (0, 1),  // down
    (-1, 0), // left
];

// brute force slow solution
fn count_blockers_loop(start_position: &(i32, i32), width: i32, blockers: &Vec<(i32, i32)>) -> i32 {
    let mut current_direction_index = 0;
    let mut count_infinite = 0;

    let (mut col, mut row) = start_position.clone();
    let mut checked_nodes = HashSet::new();
    loop {
        let current_direction = DIRECTIONS[current_direction_index % 4];
        let front_step = (col + current_direction.0, row + current_direction.1);
        if blockers.contains(&&front_step) {
            current_direction_index += 1;
            continue;
        }
        // put a blocker at the frontend step and check if it causes an infinite loop
        if front_step != *start_position && !checked_nodes.contains(&front_step) {
            if check_infinite_loop(
                &(col, row),
                width,
                &blockers,
                front_step,
                current_direction_index,
            ) {
                count_infinite += 1;
            }

            checked_nodes.insert(front_step);
        }

        col = front_step.0;
        row = front_step.1;
        if col == width - 1 || row == width - 1 || col == 0 || row == 0 {
            return count_infinite;
        }
    }
}

fn check_infinite_loop(
    start_position: &(i32, i32),
    width: i32,
    blockers: &Vec<(i32, i32)>,
    new_blocker: (i32, i32),
    mut current_direction_index: usize,
) -> bool {
    let (mut col, mut row) = start_position.clone();
    let mut visited = HashSet::new();
    loop {
        let current_direction = DIRECTIONS[current_direction_index % 4];
        let front_step = (col + current_direction.0, row + current_direction.1);

        if visited.contains(&(front_step, current_direction)) {
            return true;
        }
        if blockers.contains(&&front_step) || front_step == new_blocker {
            current_direction_index += 1;
            continue;
        }
        visited.insert((front_step, current_direction));
        col = front_step.0;
        row = front_step.1;
        if col == width - 1 || row == width - 1 || row == 0 || col == 0 {
            return false;
        }
    }
}

fn count_distinct_visisted_positions(
    start_position: &(i32, i32),
    width: i32,
    blockers: &Vec<(i32, i32)>,
) -> i32 {
    let mut current_direction_index = 0;
    let mut already_visited = HashSet::new();

    let (mut col, mut row) = start_position.clone();
    let mut steps = 0;
    loop {
        let current_direction = DIRECTIONS[current_direction_index % 4];
        let front_step = (col + current_direction.0, row + current_direction.1);
        if blockers.contains(&&front_step) {
            current_direction_index += 1;
            continue;
        }
        col = front_step.0;
        row = front_step.1;
        if !already_visited.contains(&(col, row)) {
            already_visited.insert((col, row));
            steps += 1;
        }
        if col == width - 1 || row == width - 1 {
            return steps;
        }
    }
}

fn read_input(file_path: &str) -> ((i32, i32), i32, Vec<(i32, i32)>) {
    let mut start_position: (i32, i32) = (0, 0);
    let lines = fs::read_to_string(file_path).expect("failed to read input");
    let width = lines.lines().count() as i32;

    let block_locations = lines
        .lines()
        .enumerate()
        .flat_map(|(line_number, line)| {
            line.chars()
                .enumerate()
                .filter_map(|(char_index, char)| {
                    if char == '^' {
                        start_position = (char_index as i32, line_number as i32)
                    }
                    if char == '#' {
                        return Some((char_index as i32, line_number as i32));
                    }
                    None
                })
                .collect::<Vec<_>>()
        })
        .collect();
    (start_position, width, block_locations)
}
