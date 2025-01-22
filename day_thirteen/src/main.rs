/*
You can find the challenge description here: https://adventofcode.com/2024/day/13
*/
use regex::Regex;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

const INPUT_PATH: &str = "day_thirteen/src/input.txt";

fn solve(inputs: &Vec<Vec<i128>>, target: &Vec<i128>) -> i128 {

    let (x_t, y_t) = (target[0], target[1]);
    let (x_1, y_1) = (inputs[0][0], inputs[0][1]);
    let (x_2, y_2) = (inputs[1][0], inputs[1][1]);


    // Get a and b solving the following system of equations:
    // a*x_1 + b*x_2 = x_t
    // a*y_2 + b*y_2 = y_t 

    let mut b = y_t * x_1 - x_t * y_1;
    let div = x_1 * y_2 - x_2 * y_1;
    // "b" must be a positive integer
    if b % div != 0 || b.is_positive() != div.is_positive(){
        return i128::MAX;
    }
    b /= x_1 * y_2 - x_2 * y_1;

    let mut a = x_t-b*x_2;
    // "a" must be a positive integer 
    if a % x_1 != 0 || a.is_negative(){
        return i128::MAX;
    }
    a/= x_1;

    return a*3+b;
}

fn part_one(inputs: &Vec<Vec<i32>>, target: &mut Vec<i32>) -> i32 {
    let res = solve(
        &inputs
            .iter()
            .map(|vec: &Vec<i32>| vec.iter().map(|&x| x as i128).collect())
            .collect(),
        &target.iter().map(|&x| x as i128).collect(),
    );
    if res == i128::MAX {
        i32::MAX
    } else {
        res as i32
    }
}

fn part_two(inputs: &Vec<Vec<i32>>, target: &mut Vec<i32>) -> i128 {
    solve(
        &inputs
            .iter()
            .map(|vec: &Vec<i32>| vec.iter().map(|&x| x as i128).collect())
            .collect(),
        &target.iter().map(|&x| 10000000000000 + x as i128).collect(),
    )
}

fn main() -> Result<(), Box<dyn Error>> {
    let reader = BufReader::new(File::open(INPUT_PATH)?);

    let mut count = 0;

    let mut inputs = Vec::new();

    let re = Regex::new(r"[+-]?\d+")?;
    let mut sum = 0;
    let mut sum_2 = 0i128;

    for line in reader.lines() {
        let line = line?;

        if line.trim().is_empty() {
            continue;
        }
        let mut digits: Vec<i32> = re
            .captures_iter(&line)
            .filter_map(|capture| capture[0].parse().ok())
            .collect();

        match count {
            0..=1 => {
                inputs.push(digits);
            }

            2 => {
                // Part_one
                let res = part_one(&inputs, &mut digits);
                sum += if res == i32::MAX { 0 } else { res };

                // Part_two
                let res = part_two(&inputs, &mut digits);
                sum_2 += if res == i128::MAX { 0 } else { res };

                // Clean inputs
                inputs.clear();
            }

            _ => {
                break;
            }
        }

        count = (count + 1) % 3;
    }

    println!("Result (part one): {}", sum);
    println!("Result (part two): {}", sum_2);

    Ok(())
}
