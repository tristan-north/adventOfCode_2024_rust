fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let input = std::fs::read_to_string("input.txt").expect("Couldn't read input.");

    let input_lines = input.lines().collect::<Vec<&str>>();

    let mut solution_accum = 0;

    // Horizontal
    for line in input_lines.iter() {
        solution_accum += get_num_xmas_occurances(&line);
    }

    // Vertically
    let verticals = create_verticals(&input_lines);
    for line in &verticals {
        solution_accum += get_num_xmas_occurances(line.as_str());
    }

    // First diagonal
    let diagonal = create_diagonals(&input_lines, false);
    for line in &diagonal {
        solution_accum += get_num_xmas_occurances(line.as_str());
    }

    // Second diagonal
    let diagonal = create_diagonals(&input_lines, true);
    for line in &diagonal {
        solution_accum += get_num_xmas_occurances(line.as_str());
    }

    println!("Part One solution: {solution_accum}");
}

fn create_verticals(input_lines: &[&str]) -> Vec<String> {
    let mut result = vec![];

    for i in 0..input_lines.len() {
        let mut line = vec![];
        for j in 0..input_lines.len() {
            let x = input_lines[j]
                .chars()
                .nth(i)
                .expect("Trying to get char out of bounds");

            line.push(x);
        }
        result.push(line.iter().collect());
    }

    result
}

fn create_diagonals(input_lines: &[&str], other_diagonal: bool) -> Vec<String> {
    let grid_width = input_lines.len() as i32;
    let mut result = vec![];

    for i in -grid_width + 1..grid_width {
        let mut result_line = vec![];

        let first_coord = (std::cmp::max(0, i), std::cmp::max(0, -i));

        let num_values_in_diag_line = grid_width - i.abs();
        for k in 0..num_values_in_diag_line {
            let x = first_coord.0 + k;
            let mut y = first_coord.1 + k;

            if other_diagonal {
                y = grid_width - y - 1;
            }

            let element = input_lines[y as usize]
                .chars()
                .nth(x as usize)
                .expect("Trying to get char out of bounds");
            result_line.push(element);
        }

        result.push(result_line.iter().collect());
    }

    result
}

fn get_num_xmas_occurances(line: &str) -> u32 {
    let mut accum = 0;
    accum += line.matches("XMAS").count();
    accum += line.matches("SAMX").count();

    accum as u32
}

// ---- Part Two ----

fn part_two() {
    let input = std::fs::read_to_string("input.txt").expect("Couldn't read input.");

    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut solution_accum = 0;
    for y in 0..grid.len() - 2 {
        for x in 0..grid.len() - 2 {
            solution_accum += check_for_match(&grid, (x, y));
        }
    }

    println!("Part Two solution: {solution_accum}");
}

fn check_for_match(grid: &Vec<Vec<char>>, coord: (usize, usize)) -> u32 {
    let config_a = [['M', '.', 'M'], ['.', 'A', '.'], ['S', '.', 'S']];
    let config_b = [['S', '.', 'S'], ['.', 'A', '.'], ['M', '.', 'M']];
    let config_c = [['M', '.', 'S'], ['.', 'A', '.'], ['M', '.', 'S']];
    let config_d = [['S', '.', 'M'], ['.', 'A', '.'], ['S', '.', 'M']];
    let configs = [config_a, config_b, config_c, config_d];

    for config in configs {
        let mut is_match = true;
        for x in 0..3 {
            for y in 0..3 {
                let config_val = config[y][x];
                if config_val == '.' {
                    continue;
                }

                let grid_val = grid[(coord.1 + x) as usize][(coord.0 + y) as usize];
                if grid_val != config_val {
                    is_match = false;
                    break;
                }
            }
        }
        if is_match {
            return 1;
        }
    }

    0
}
