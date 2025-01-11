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

fn is_sorted_even(vec: &Vec<i32>) -> bool {
    let mut index = 0;
    if vec[0] < vec[index] {
        while index < vec.len() {
            if vec[index] > vec[index + 1] {
                return false;
            }
            index += 2;
        }
    } else {
        while index < vec.len() {
            if vec[index] < vec[index + 1] {
                return false;
            }
            index += 2;
        }
    }
    return true;
}
fn is_sorted_odd(vec: &Vec<i32>) -> bool {
    let mut index: usize = 0;
    if vec[0] < vec[index] {
        while index < vec.len() {
            if index == vec.len() - 1 {
                if vec[index - 1] > vec[index] {
                    return false;
                }
                break;
            }
            if vec[index] > vec[index + 1] {
                return false;
            }
            index += 2;
        }
    } else {
        while index < vec.len() {
            if index == vec.len() - 1 {
                if vec[index - 1] < vec[index] {
                    return false;
                }
                break;
            }
            if vec[index] < vec[index + 1] {
                return false;
            }
            index += 2;
        }
    }
    return true;
}

fn is_sorted(vec: &Vec<i32>) -> bool {
    let len = vec.len();
    if len - 1 <= 1 {
        return true;
    }

    if len % 2 == 0 {
        return is_sorted_even(&vec);
    } else {
        return is_sorted_odd(&vec);
    }
}

fn check_diff(levels: &Vec<i32>) -> bool {
    if levels.len() % 2 == 0 {
        return check_diff_even(&levels);
    } else {
        return check_diff_odd(&levels);
    }
}

fn check_diff_even(levels: &Vec<i32>) -> bool {
    let mut index = 0;
    while index < levels.len() - 1 {
        let diff = (levels[index] - levels[index + 1]).abs();
        if (diff < 1) || (diff > 3) {
            return false;
        }
        index += 2;
    }
    return true;
}

fn check_diff_odd(levels: &Vec<i32>) -> bool {
    let mut v1: i32 = 0;
    let mut v2: i32 = 0;
    let mut index = 0;
    while index < levels.len() - 1 {
        if index == levels.len() {
            v1 = levels[index - 1];
            v2 = levels[index];
        } else {
            v1 = levels[index];
            v2 = levels[index + 1];
        }
        let diff = (v1 - v2).abs();
        if (diff < 1) || (diff > 3) {
            return false;
        }
        index += 2;
    }
    return true;
}

fn main() {
    let input = read_input();
    let mut safe_count: usize = 0;
    for levels in input {
        if !is_sorted(&levels) | !check_diff(&levels) {
            continue;
        }
        safe_count += 1;
    }

    println!("Safe count: {}", safe_count);
}
