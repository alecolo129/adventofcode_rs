/*
    Challenge description can be found here: https://adventofcode.com/2024/day/7
*/

use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_lines(reader: &mut BufReader<File>) -> Result<Vec<Vec<i64>>, Box<dyn std::error::Error>> {
    let mut result = vec![];

    for line in reader.lines() {
        let nums = line?
            .split_ascii_whitespace()
            .map(|num_str| num_str.trim_end_matches(':').parse::<i64>().unwrap())
            .collect();

        result.push(nums);
    }

    Ok(result)
}

/// Checks if there exists an expression containing the given operands that returns the target
fn exists_expression_part_one(target: i64, partial_eval: i64, operands: &[i64]) -> bool {
    // If we have no more operands we must have hit the target
    if operands.is_empty() {
        return target == partial_eval;
    }

    // // Cannot get any valid solution
    if target < partial_eval {
        return false;
    }

   
    exists_expression_part_one(target, partial_eval + operands[0], &operands[1..])  // Try reaching solution by adding ...
        || exists_expression_part_one(target, partial_eval * operands[0], &operands[1..]) // or multiplying next operand
}

/// Checks if there exists an expression containing the given operands that returns the target
fn exists_expression_part_two(target: i64, partial_eval: i64, operands: &[i64]) -> bool {
    // If we have no more operands we must have hit the target
    if operands.is_empty() {
        return target == partial_eval;
    }

    // Cannot get any valid solution
    if target < partial_eval {
        return false;
    }

    // Shift for computing partial_eval||operands[0]
    let shift = 10i64.pow(operands[0].ilog10() + 1);
    
    exists_expression_part_two(target, partial_eval + operands[0], &operands[1..]) // Try reaching solution by adding ...
        || exists_expression_part_two(target, partial_eval * operands[0], &operands[1..]) // or multiplying ...
        || exists_expression_part_two(target, partial_eval * shift + operands[0], &operands[1..]) // or concatenating next operand
}

// Solves part one
fn part_one(nums: &Vec<Vec<i64>>) -> i64 {
    nums.iter()
        .filter_map(|line| {
            if exists_expression_part_one(line[0], 0, &line[1..]) {
                return Some(line[0]);
            }
            return None;
        })
        .sum()
}

// Solves part two
fn part_two(nums: &Vec<Vec<i64>>) -> i64 {
    nums.iter()
        .filter_map(|line| {
            if exists_expression_part_two(line[0], 0, &line[1..]) {
                return Some(line[0]);
            }
            return None;
        })
        .sum()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("day_seven/src/input.txt")?;
    let mut reader = BufReader::new(file);
    let nums = parse_lines(&mut reader)?;

    println!("Result (part one): {:?}", part_one(&nums));
    println!("Result (part two): {:?}", part_two(&nums));

    Ok(())
}
