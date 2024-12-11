use std::borrow::BorrowMut;
use std::collections::BTreeSet;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

const INPUT_PATH: &str = "day_nine/src/input.txt";

fn parse_input(reader: &mut BufReader<File>) -> Result<Vec<i32>, Box<dyn Error>> {
    let mut disk_map = String::new();

    reader.read_line(&mut disk_map)?;

    let disk_map = disk_map
        .chars()
        .filter_map(|c| c.to_digit(10).map(|c| c as i32))
        .collect::<Vec<i32>>();

    Ok(disk_map)
}

fn to_previous_full_chunk(disk_map: &mut Vec<i32>, index_full_chunk: &mut usize, end: &mut usize) {
    *index_full_chunk -= disk_map[*end] as usize; // Subtract all remaining elements in full chunk
    *index_full_chunk -= disk_map[*end - 1] as usize; // Subtract all free spaces before full chunk
    disk_map[*end] = 0;
    *end -= 2; // Go to next full chunk
}

fn to_next_free_chunk(
    disk_map: &mut Vec<i32>,
    index_free_chunk: &mut usize,
    start: &mut usize,
    end: &usize,
    check_sum: &mut i64,
) {
    *index_free_chunk += disk_map[*start] as usize;

    // Compute checksum of the busy chunk in between two free chunks
    if *start + 1 < *end {
        let chunk_in_between = *start + 1;
        let index_in_between = *index_free_chunk;
        //println!("In between: {}x{index_in_between}...{} => {}", to_tag(chunk_in_between), index_in_between + disk_map[chunk_in_between] as usize - 1, to_tag(chunk_in_between) * compute_sum(index_in_between, index_in_between + disk_map[chunk_in_between] as usize - 1,));
        *check_sum += to_tag(chunk_in_between)
            * compute_sum(
                index_in_between,
                index_in_between + disk_map[chunk_in_between] as usize - 1,
            );
        *index_free_chunk += disk_map[chunk_in_between] as usize;
        disk_map[chunk_in_between] = 0;
    }

    *start += 2;
}

fn compute_sum(start_index: usize, end_index: usize) -> i64 {
    let (start_index, end_index) = (start_index as i64, end_index as i64);
    if start_index == end_index {
    }
    ((end_index) * (end_index + 1) - (start_index - 1) * (start_index)) >> 1
}

fn to_tag(index: usize) -> i64 {
    (index >> 1) as i64
}

fn part_one(mut disk_map: Vec<i32>) -> i64 {
    let mut check_sum = 0;

    let mut start = 1; // First free entry
    let mut end = if (disk_map.len() - 1) % 2 == 0 {
        disk_map.len() - 1
    } else {
        disk_map.len() - 2
    }; // Last occupied entry

    let mut index_free_chunk = disk_map[0] as usize;
    let mut index_full_chunk = disk_map.iter().sum::<i32>() as usize;

    while start < end {
        // We have enough free spaces to move the entire full chunk
        if disk_map[start] > disk_map[end] {
            // Update checksum
            check_sum += to_tag(end)
                * compute_sum(
                    index_free_chunk,
                    index_free_chunk + disk_map[end] as usize - 1,
                );

            // Move all elements to free chunk
            index_free_chunk += disk_map[end] as usize;
            disk_map[start] -= disk_map[end];

            // Go to previous full chunk
            to_previous_full_chunk(&mut disk_map, &mut index_full_chunk, &mut end);
        }
        // We can only move some elements of last chunk
        else {
            // Update checksum
            check_sum += to_tag(end)
                * compute_sum(
                    index_free_chunk,
                    index_free_chunk + disk_map[start] as usize - 1,
                );

            // Move all elements we can in free chunk
            index_full_chunk -= disk_map[start] as usize;
            disk_map[end] -= disk_map[start];

            // If full chunk is empty go to previous full chunk
            if disk_map[end] == 0 {
                to_previous_full_chunk(&mut disk_map, &mut index_full_chunk, &mut end);
            }

            // Go to next free chunk
            to_next_free_chunk(
                &mut disk_map,
                &mut index_free_chunk,
                &mut start,
                &mut end,
                &mut check_sum,
            );
        }
    }

    // Empty any remaining element
    check_sum += to_tag(end)
        * compute_sum(
            index_full_chunk - disk_map[end] as usize,
            index_full_chunk - 1,
        );

    check_sum
}

fn part_two(disk_map: &Vec<i32>) -> i64{
    let mut free_ranges = BTreeSet::new();

    disk_map
        .iter()
        .enumerate()
        .fold(0, |start_idx, (i, &capacity)| {
            if i % 2 == 1 && capacity != 0 {
                let free_range = (start_idx as usize, (start_idx + capacity) as usize);
                free_ranges.insert(free_range);
            }
            start_idx + capacity
        });
    
    let mut end_index = disk_map.iter().map(|&el| el as usize).sum();
    
    disk_map.iter().enumerate().rev().map(|(i, &n_elements )|{
        let n_elements = n_elements as usize;

        let mut res = 0;

        // Entry is occupied
        if i!=0 && i%2==0 {

            let mut selected_range = None;

            for free_range in free_ranges.borrow_mut().iter() {
                    
                    // Free chunk must be in lower positions 
                    if free_range.0 > end_index {
                        break;
                    }

                    let capacity = free_range.1-free_range.0;

                    // Free chunk is now large enough
                    if capacity < n_elements{
                        continue;
                    }
                    
                    selected_range = Some(free_range.clone());
                    
                    // Compute checksum and exit loop
                    res = to_tag(i) * compute_sum(free_range.0,free_range.0+n_elements-1);

                    break;
            }  

            // If some free space was found, remove it from the set of free ranges 
            if let Some(free_range) = selected_range {
                free_ranges.remove(&free_range);
                // Insert new free chunk with remaining free spaces
                if (free_range.1-free_range.0) > n_elements {
                    free_ranges.insert((free_range.0 + n_elements, free_range.1));
                }
            }

            // If no free space was found, compute checksum of occupied chunk 
            else {
                res = to_tag(i) * compute_sum(end_index-n_elements, end_index-1);
            }
        }

        // Decrease end_index and yield result
        end_index -= n_elements;
        res
    }).sum()
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut reader = BufReader::new(File::open(INPUT_PATH)?);
    let disk_map = parse_input(&mut reader)?;

    println!("Result (part one): {:?}", part_one(disk_map.clone()));

    println!("Result (part two): {:?}", part_two(&disk_map));

    Ok(())
}
