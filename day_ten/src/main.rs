use std::collections::HashSet;
/*
    You can find the challenge description here: https://adventofcode.com/2024/day/10
*/
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

const INPUT_PATH: &str = "day_ten/src/input.txt";

/// Parses input map and fill the set 'starting_ponts' with positions of all 0-entries.
fn parse_map(
    reader: &mut BufReader<File>,
    starting_points: &mut HashSet<(usize, usize)>,
    ending_points: &mut HashSet<(usize, usize)>
) -> Result<Vec<Vec<u32>>, Box<dyn std::error::Error>> {
    let mut map = Vec::new();

    reader
        .lines()
        .into_iter()
        .enumerate()
        .try_for_each(|(i, line)| {
            let row = line?
                .chars()
                .enumerate()
                .map(|(j, ch)| {
                    if ch == '0' {
                        starting_points.insert((i, j));
                    }
                    else if ch == '9'{
                        ending_points.insert((i,j));
                    }
                    ch.to_digit(10)
                        .expect(&format!("Cannot convert '{}' to digit", ch))
                })
                .collect();

            map.push(row);

            Ok::<(), Box<dyn std::error::Error>>(())
        })?;

    Ok(map)
}

/// Counts the number of trailheads from a single starting point
fn count_trailheads(
    map: &Vec<Vec<u32>>,
    ending_points: &mut HashSet<(usize, usize)>,
    current_point: &(usize, usize),   
    part_one: bool
) -> i32 {
    let (i, j) = *current_point;

    // We have completed a trail
    if map[i][j] == 9 {

        // In part one we count all trails ending at a specific position as a single trail
        if part_one && !ending_points.remove(&(i,j)){
            return 0;
        }

        return 1;
    }

    let mut sum = 0;
    for delta in vec![(-1, 0), (1, 0), (0, -1), (0, 1)] {

        // Coordinates of neighboring cell
        let (i_new, j_new) = ((i as i32) + delta.0, (j as i32) + delta.1);

        // Neighboring cell is within bounds, and contains next number in sequence
        if (0..map.len()).contains(&(i_new as usize))
            && (0..map[0].len()).contains(&(j_new as usize))
            && map[i_new as usize][j_new as usize] == map[i][j] + 1
        {
            sum += count_trailheads(map, ending_points, &(i_new as usize, j_new as usize), part_one);
        }
    }

    sum
}

fn part_one(map: &Vec<Vec<u32>>, starting_points: &HashSet<(usize, usize)>, ending_points: &HashSet<(usize, usize)>) -> i32 {
    starting_points
        .iter()
        .map(|starting_point| {
            {
                let sum = count_trailheads(
                map,
                &mut ending_points.clone(),
                starting_point,
                    true
                );
                // println!("");
                sum
            }
        })
        .sum()
}

fn part_two(map: &Vec<Vec<u32>>, starting_points: &HashSet<(usize, usize)>, ending_points: &HashSet<(usize, usize)>) -> i32 {
    starting_points
        .iter()
        .map(|starting_point| {
            {
                let sum = count_trailheads(
                map,
                &mut ending_points.clone(),
                starting_point,
                false
                );
                // println!("");
                sum
            }
        })
        .sum()
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut reader = BufReader::new(File::open(INPUT_PATH)?);
    let mut starting_points = HashSet::new();
    let mut ending_points = HashSet::new();

    let map = parse_map(&mut reader, &mut starting_points, &mut ending_points)?;
    // println!("{:?}", starting_points);
    println!("Result (part one): {}", part_one(&map, &starting_points, &ending_points));
    println!("Result (part two): {}", part_two(&map, &starting_points, &ending_points));

    Ok(())
}
