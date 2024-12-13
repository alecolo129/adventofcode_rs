/*
You can find the challenge description here: https://adventofcode.com/2024/day/11
*/
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

const INPUT_PATH: &str = "day_eleven/src/input.txt";

fn parse(reader: &mut BufReader<File>) -> Result<HashMap<u64, u64>, Box<dyn Error>> {
    let mut line = String::new();
    reader.read_line(&mut line)?;
    let mut stones_to_count = HashMap::new();
    line.split(" ").for_each(|s| {
        let key = s
            .parse::<u64>()
            .expect(&format!("Cannot convert \"{}\" to number", s));
        stones_to_count
            .entry(key)
            .and_modify(|occ| *occ += 1)
            .or_insert(1);
    });
    Ok(stones_to_count)
}

fn part_one(stones_to_count: &mut HashMap<u64, u64>, n_blinks: u32) -> u64 {
    (0..n_blinks).for_each(|i| {
        let stones_to_count_old = stones_to_count.clone();
        stones_to_count.clear();

        stones_to_count_old
            .iter()
            .for_each(|(num, count)| {
                let n_digits = if *num != 0 { num.ilog10() + 1 } else { 1 };
                
                // Split number in 2
                if n_digits > 0 && (n_digits % 2) == 0 {
                    let base = 10u64.pow(n_digits >> 1);
                    let rem = *num % base; //num[..n_digits >> 1].trim_start_matches('0').to_string();
                    let div = num / base; //num[n_digits >> 1..].trim_start_matches('0').to_string();
                    stones_to_count.entry(rem).and_modify(|old_count| *old_count += *count).or_insert(*count);
                    stones_to_count.entry(div).and_modify(|old_count| *old_count += *count).or_insert(*count);
                }
                
                // Replace 0s with 1s
                else if *num == 0 {
                    stones_to_count.entry(1).and_modify(|old_count| *old_count += *count).or_insert( *count);
                }

                // Multiply by 2024
                else{
                    // stones_to_count.remove(num);
                    stones_to_count.entry(num*2024).and_modify(|old_count| *old_count += *count).or_insert( *count); 
                }
            });
    });
    stones_to_count.iter().map(|(_, v)| *v).sum()
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut reader = BufReader::new(File::open(INPUT_PATH)?);
    let mut stones_to_count = parse(&mut reader)?;
    println!("Result (part one): {}", part_one(&mut stones_to_count.clone(), 25));
    println!("Result (part two): {} ", part_one(&mut stones_to_count, 75));

    Ok(())
}
