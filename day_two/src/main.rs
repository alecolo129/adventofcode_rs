/*
    You can find the challenge description here: https://adventofcode.com/2024/day/2
*/

use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_reports(
    reader: &mut BufReader<File>,
) -> Result<Vec<Vec<i32>>, Box<dyn std::error::Error>> {
    let mut reports = Vec::new();

    for line in reader.lines() {
        match line {
            Ok(line_str) => {
                reports.push(
                    line_str
                        .split(" ")
                        .map(|el| el.parse::<i32>().unwrap())
                        .collect::<Vec<i32>>(),
                );
            }
            Err(e) => eprintln!("Error: {:?}", e),
        }
    }

    Ok(reports)
}

/// Check if input reports are valid according to rules
fn is_valid_report(report: &[i32], report_right: Option<&[i32]>) -> bool {
    let mut last_num = report[0];

    let ascending = if report.len() >= 2 {
        report[1] > report[0]
    } else {
        report_right.expect("Should have at least two elements in report")[1] > report[0]
    };

    for num in &report[1..] {
        if ascending && ((num - last_num) < 1 || (num - last_num) > 3) {
            return false;
        }
        if !ascending && ((last_num - num) < 1 || (last_num - num) > 3) {
            return false;
        }
        last_num = *num;
    }

    if let Some(report) = report_right {
        for num in report {
            if ascending && ((num - last_num) < 1 || (num - last_num) > 3) {
                return false;
            }
            if !ascending && ((last_num - num) < 1 || (last_num - num) > 3) {
                return false;
            }
            last_num = *num;
        }
    }

    true
}

fn part_one(reports: &Vec<Vec<i32>>) -> i32 {
    // Count number of valid reports
    reports
        .iter()
        .map(|report| is_valid_report(&report, None) as i32)
        .sum()
}

fn part_two(reports: &Vec<Vec<i32>>) -> i32 {
    let mut safe_levels = 0;

    for report in reports {
        // Try without removing anything
        if is_valid_report(&report, None) {
            safe_levels += 1;
            continue;
        }

        let len = report.len();

        // Try removing first and last element
        if is_valid_report(&report[1..], None) || is_valid_report(&report[..len - 1], None) {
            safe_levels += 1;
            continue;
        }

        // Try removing elements in between
        for i in 1..len - 1 {
            if is_valid_report(&report[0..i], Some(&report[i + 1..])) {
                safe_levels += 1;
                break;
            }
        }
    }
    safe_levels
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("day_two/src/input.txt")?;
    let mut reader = BufReader::new(file);

    let reports = parse_reports(&mut reader)?;

    let safe_levels = part_one(&reports);
    println!("Safe levels (part one): {safe_levels}");

    let safe_levels = part_two(&reports);
    println!("Safe levels (part two): {safe_levels}");
    Ok(())
}
