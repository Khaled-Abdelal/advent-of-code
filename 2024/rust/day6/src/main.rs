use std::fs;

fn main() {
    let (start_position, width, blockers) = read_input("input.txt");
    let visited_positions = count_distinct_visisted_positions(&start_position, width, &blockers);
    println!("Count of visited_positions is {:?}", visited_positions);
}

fn count_distinct_visisted_positions(
    start_position: &(i32, i32),
    width: i32,
    blockers: &Vec<(i32, i32)>,
) -> i32 {
    let directions = [
        (0, -1), // up
        (1, 0),  // right
        (0, 1),  // down
        (-1, 0), // left
    ];
    let mut current_direction_index = 0;
    let mut already_visited = Vec::new();

    let (mut col, mut row) = start_position.clone();
    let mut steps = 0;
    loop {
        let current_direction = directions[current_direction_index % 4];
        let front_step = (col + current_direction.0, row + current_direction.1);
        if blockers.contains(&&front_step) {
            current_direction_index += 1;
            continue;
        }
        col = front_step.0;
        row = front_step.1;
        if !already_visited.contains(&(col, row)) {
            already_visited.push((col, row));
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
