#![allow(dead_code)]

use std::{fs, cmp};
use regex::Regex;

fn get_file_string(file_path: &str) -> String {
    let contents = fs::read_to_string(file_path).expect("Unable to read string");

    return contents;
}

fn part1() {
    let text = get_file_string("./day2/src/input.txt");
    let mut answer: i32 = 0;

    'outer: for line in text.lines() {
        for _ in Regex::new(r"(( 1[3-9]|[2-9]\d+) red|( 1[4-9]|[2-9]\d+) green|( 1[5-9]|[2-9]\d+) blue)").unwrap().find_iter(line) {
            continue 'outer;
        }
        let game_id = line.split(":").next().unwrap().split(" ").nth(1).unwrap().parse::<i32>().unwrap();
        answer += game_id;
    }
    println!("Answer: {}", answer);
}

fn part2() {
    let text = get_file_string("./day2/src/input.txt");
    let color_indexes = vec!["red", "green", "blue"];
    let mut answer: i32 = 0;

    for line in text.lines() {
        let mut min_color_values = vec![0, 0, 0];

        for (_, [value, color]) in Regex::new(r"(\d+) (red|green|blue)").unwrap().captures_iter(line).map(|c| c.extract()) {
            let color_idx = color_indexes.iter().position(|&r| r == color).unwrap();
            min_color_values[color_idx as usize] = cmp::max(value.parse::<i32>().unwrap(), min_color_values[color_idx]);
        }
        answer += min_color_values[0] * min_color_values[1] * min_color_values[2];
    }
    println!("Answer: {}", answer);
}

fn main() {
    part1();
    part2();
}
