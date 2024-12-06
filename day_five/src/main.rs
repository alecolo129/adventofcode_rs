/*
    You can find the challenge description here: https://adventofcode.com/2024/day/5
*/
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

// Parse input ordering rules
fn parse_ordering(
    reader: &mut BufReader<File>,
) -> Result<HashMap<i32, HashSet<i32>>, Box<dyn std::error::Error>> {
    let mut ordering_rules: HashMap<i32, HashSet<i32>> = HashMap::new();
    for line in reader.lines() {
        match line {
            Ok(line_str) => {
                let numbers = line_str
                    .split('|')
                    .map(|num| num.trim_end())
                    .collect::<Vec<&str>>();

                if numbers.len() != 2 {
                    // We have already parsed all ordering rules
                    break;
                }

                let (num_0, num_1) = (numbers[0].parse::<i32>()?, numbers[1].parse::<i32>()?);
                ordering_rules
                    .entry(num_0)
                    .and_modify(|set| {
                        (*set).insert(num_1);
                    })
                    .or_insert({
                        HashSet::from([num_1])
                    });
            }
            Err(e) => eprintln!("{}", e),
        }
    }
    Ok(ordering_rules)
}

// Finds ordering relation between 'a' and 'b' using input ordering rules
fn find_ordering(
    ordering_rules: &HashMap<i32, HashSet<i32>>,
    a: i32,
    b: i32,
) -> Result<std::cmp::Ordering, Box<dyn std::error::Error>> {
    
    // b is in set of numbers smaller than a
    if ordering_rules
        .get(&a)
        .ok_or(format!("Error: index {} not found", a))?
        .contains(&b){
            return Ok(std::cmp::Ordering::Greater);
    }

    // a is in set of numbers smaller than b
    if ordering_rules
        .get(&b)
        .ok_or(format!("Error: index {} not found", b))?
        .contains(&a){
        return Ok(std::cmp::Ordering::Less);
    }

    return Err("Cannot determine ordering".into());
}

// Solves part one filling the vector 'unordered_lines' with all out-of-order lines
fn part_one(
    reader: &mut BufReader<File>,
    ordering_rules: &HashMap<i32, HashSet<i32>>,
    unordered_lines: &mut Vec<Vec<i32>>,
) -> Result<i32, Box<dyn std::error::Error>> {
    let mut tot = 0;

    for line in reader.lines() {
        match line {
            Ok(line_str) => {
                let nums: Vec<i32> = line_str
                    .split(',')
                    .map(|num_str| num_str.parse::<i32>().unwrap())
                    .collect();
                let mut valid = true;
                for i in 0..nums.len() - 1 {
                    if find_ordering(ordering_rules, nums[i], nums[i + 1])?
                        != std::cmp::Ordering::Greater
                    {
                        valid = false;
                    }
                }
                if valid {
                    tot += nums[nums.len() / 2];
                } else {
                    unordered_lines.push(nums);
                }
            }
            Err(e) => eprintln!("{}", e),
        }
    }
    Ok(tot)
}

// Solves part two taking as input all the out-of-order lines identified in part one
fn part_two(
    undordered_lines: &mut Vec<Vec<i32>>,
    ordering_rules: &HashMap<i32, HashSet<i32>>,
) -> i32 {
    let mut tot = 0;
    for line in undordered_lines {
        // Sort line using the input ordering rules and add mid element to total
        line.sort_by(|&a, &b| find_ordering(ordering_rules, b, a).unwrap());
        tot += line[line.len() / 2];
    }
    tot
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("day_five/src/input.txt")?;
    let mut reader = BufReader::new(file);
    let ordering_rules = parse_ordering(&mut reader)?;

    let mut unordered_lines = Vec::new();
    println!(
        "Result (part one): {:?}",
        part_one(&mut reader, &ordering_rules, &mut unordered_lines)?
    );
    println!(
        "Result (part two): {:?}",
        part_two(&mut unordered_lines, &ordering_rules)
    );
    Ok(())
}
