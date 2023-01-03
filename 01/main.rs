use std::cmp;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

fn part_one() {
    let filename = "input2.txt";

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut maximum_value: i32 = 0;
    let mut current_sum: i32 = 0;

    for line in reader.lines() {
        match line.unwrap().as_str() {
            "" => {
                maximum_value = cmp::max(maximum_value, current_sum);
                current_sum = 0;
            }
            content => {
                let calories: i32 = i32::from_str(content).unwrap();
                current_sum += calories;
            }
        }
    }

    maximum_value = cmp::max(maximum_value, current_sum);

    println!("Result: {}", maximum_value);
}

fn part_two() {
    let filename = "input2.txt";

    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut calories_sum: Vec<i32> = Vec::new();
    let mut current_sum: i32 = 0;

    for line in reader.lines() {
        match line.unwrap().as_str() {
            "" => {
                calories_sum.push(current_sum);
                current_sum = 0;
            }
            content => {
                let calories = i32::from_str(content).unwrap();
                current_sum += calories;
            }
        }
    }

    if current_sum > 0 {
        calories_sum.push(current_sum)
    };

    calories_sum.sort();

    let result: i32 = calories_sum.iter().rev().take(3).sum();

    println!("Result: {}", result);
}

fn main() {
    part_one();
    part_two();
}
