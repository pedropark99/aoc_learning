use itertools::Itertools;
use std::fs;

fn read_input() -> Vec<Vec<i32>> {
    const PATH: &str = "ex2-2024/src/input.txt";
    let input_text = fs::read_to_string(PATH).expect("File not found!");
    let it = input_text.split('\n');
    let lines: Vec<&str> = it.clone().collect();
    let n_lines = it.clone().count() - 1;
    let mut levels: Vec<Vec<i32>> = Vec::with_capacity(n_lines);
    for _index in 0..n_lines {
        levels.push(Vec::new());
    }

    for index in 0..n_lines {
        let levels_it = lines[index].split(' ');
        let levels_as_str: Vec<&str> = levels_it.collect();

        for level in levels_as_str {
            let level_as_int = level.parse::<i32>().unwrap();
            levels[index].push(level_as_int);
        }
    }

    return levels;
}

enum Direction {
    Increasing,
    Decreasing,
}

fn is_good(levels: &Vec<i32>) -> bool {
    let mut direction = Direction::Increasing;
    if levels[0] > levels[levels.len() - 1] {
        direction = Direction::Decreasing;
    }
    for (v1, v2) in levels.iter().tuple_windows() {
        let diff: i32 = (v1 - v2).abs();
        if (diff < 1) | (diff > 3) {
            return false;
        }
        match direction {
            Direction::Increasing => {
                if v1 > v2 {
                    return false;
                }
            }
            Direction::Decreasing => {
                if v1 < v2 {
                    return false;
                }
            }
        }
    }

    return true;
}

fn main() {
    let input = read_input();
    let mut safe_count: usize = 0;
    for levels in input {
        if is_good(&levels) {
            safe_count += 1;
        }
    }

    println!("Safe count: {}", safe_count);
}
