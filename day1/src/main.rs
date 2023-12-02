#![allow(dead_code)]

use std::{fs, collections::HashMap};

fn get_file_string(file_path: &str) -> String {
    let contents = fs::read_to_string(file_path).expect("Unable to read string");

    return contents;
}

fn part1() {
    let text = get_file_string("./day1/src/input.txt");
    let mut cal_values: Vec<i32> = Vec::new();
    let mut line_nums: String = String::default();

    for line in text.lines() {
        for c in line.chars() {
            if c.is_digit(10) {
                line_nums.push(c);
            }
        }

        let cal_v: i32 = (line_nums.chars().nth(0).unwrap().to_string()
            + &line_nums.chars().last().unwrap().to_string())
            .parse::<i32>()
            .unwrap();
        cal_values.push(cal_v);
        line_nums = String::default();
    }
    let result: i32 = cal_values.iter().sum();
    println!("Total: {:?}", result);
}

fn part2() {
    let text = get_file_string("./day1/src/input.txt");
    let mut cal_values: Vec<i32> = Vec::new();
    let numbers_repr = HashMap::from([
        ("one", "1e"),
        ("two", "2o"),
        ("three", "3e"),
        ("four", "4"),
        ("five", "5e"),
        ("six", "6"),
        ("seven", "7n"),
        ("eight", "8t"),
        ("nine", "9e")
    ]);

    for line in text.lines() {
        let mut nums_vec: Vec<_> = line.match_indices("one").collect();
        let mut twos_vec: Vec<_> = line.match_indices("two").collect();
        let mut threes_vec: Vec<_> = line.match_indices("three").collect();
        let mut fours_vec: Vec<_> = line.match_indices("four").collect();
        let mut fives_vec: Vec<_> = line.match_indices("five").collect();
        let mut sixs_vec: Vec<_> = line.match_indices("six").collect();
        let mut sevens_vec: Vec<_> = line.match_indices("seven").collect();
        let mut eights_vec: Vec<_> = line.match_indices("eight").collect();
        let mut nines_vec: Vec<_> = line.match_indices("nine").collect();

        nums_vec.append(&mut twos_vec);
        nums_vec.append(&mut threes_vec);
        nums_vec.append(&mut fours_vec);
        nums_vec.append(&mut fives_vec);
        nums_vec.append(&mut sixs_vec);
        nums_vec.append(&mut sevens_vec);
        nums_vec.append(&mut eights_vec);
        nums_vec.append(&mut nines_vec);
        nums_vec.sort();

        let mut line_chars;

        if nums_vec.len() > 0 {
            let fst_num: (usize, &str) = *nums_vec.first().unwrap();
            let lst_num: (usize, &str) = *nums_vec.last().unwrap();
            line_chars = line.replacen(fst_num.1, numbers_repr.get(fst_num.1).unwrap(), 1);
            line_chars = line_chars.replace(lst_num.1, numbers_repr.get(lst_num.1).unwrap());
        } else {
            line_chars = line.to_string();
        }

        let mut line_nums: String = String::default();

        for c in line_chars.chars() {
            if c.is_digit(10) {
                line_nums.push(c);
            }
        }

        let cal_v: i32 = (line_nums.chars().nth(0).unwrap().to_string()
            + &line_nums.chars().last().unwrap().to_string())
            .parse::<i32>()
            .unwrap();
        cal_values.push(cal_v);
    }
    let result: i32 = cal_values.iter().sum();
    println!("Total: {:?}", result);
}

fn main() {
    part1();
    part2();
}
