use std::fs::File;
use std::io::{BufReader, BufRead, Seek, SeekFrom};

use regex::Regex;

// Execute a multiplication of type mul(num_1, num_2), returns num_1*num_2
fn mul_numbers(re: &Regex, matching_mul: &str) -> Result<i128, Box<dyn std::error::Error>>{
    let captures = re.captures(matching_mul).ok_or("Error: cannot parse input multiplication")?;
    let op_one = captures.get(1).unwrap().as_str().parse::<i128>()?;
    let op_two = captures.get(2).unwrap().as_str().parse::<i128>()?;
    Ok(op_one * op_two)
}

fn part_one(reader: &mut BufReader<File>) -> Result<i128, Box<dyn std::error::Error>>{
    let re = Regex::new(r"mul\((\d+),(\d+)\)").expect("Invalid regex");
    let mut total: i128 = 0;

    for line in reader.lines(){
        match line {
            Ok(input_str) => {
                for matching_mul in re.find_iter(&input_str) {
                    total += mul_numbers(&re, matching_mul.as_str())?;
                }
            }
            Err(err) => eprintln!("Error: {:?}",err)
        }
    }
    Ok(total)
}


fn part_two(reader: &mut BufReader<File>) -> Result<i128, Box<dyn std::error::Error>>{
    reader.seek(SeekFrom::Start(0))?;
    let re = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").expect("Invalid regex");
    let mut total: i128 = 0;
    let mut active = true;

    for line in reader.lines(){
        match line {
            Ok(input_str) => {
                for matching_string in re.find_iter(&input_str) {

                    match  matching_string.as_str() {
                        "do()" => {
                            active = true; // Re-start counting matching multiplications
                        }
                        "don't()" => {
                            active = false; // Stop counting matching multiplication
                        }
                        matching_mul => {
                            if active{
                                total += mul_numbers(&re, matching_mul)?;
                            }
                        }
                    }
                    
                }
            }
            Err(err) => eprintln!("Error: {:?}",err)
        }
    }
    Ok(total)
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let file = File::open("day_three/src/input.txt")?;
    let mut reader = BufReader::new(file);

    let result = part_one(&mut reader)?;
    println!("Result (part one): {}", result);

    let result = part_two(&mut reader)?;
    println!("Result (part two): {}", result);
    Ok(())
}
