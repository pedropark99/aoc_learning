// https://adventofcode.com/2024/day/1
use std::fs;

fn main() {
    const PATH: &str = "ex1-2024/src/input.txt";
    let input_text = fs::read_to_string(PATH).expect("File not found!");
    let it = input_text.split('\n');
    let n_lines = it.clone().count() - 1;
    let mut first_list: Vec<i32> = Vec::with_capacity(n_lines);
    let mut second_list: Vec<i32> = Vec::with_capacity(n_lines);

    for line in it {
        if line == "" {
            break;
        }
        let components: Vec<&str> = line.split("   ").collect();
        let first_num = components[0].parse::<i32>().unwrap();
        let second_num = components[1].parse::<i32>().unwrap();
        first_list.push(first_num);
        second_list.push(second_num);
    }

    first_list.sort();
    second_list.sort();
    let mut total_distance: u32 = 0;
    for index in 0..n_lines - 1 {
        total_distance += (first_list[index] - second_list[index]).abs() as u32;
    }

    println!("Total distance: {}", total_distance);
}
