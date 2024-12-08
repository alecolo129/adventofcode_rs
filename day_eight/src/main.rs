use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::usize;

const INPUT_PATH: &str = "day_eight/src/input.txt";

/// Parse input map saving it as a dictionary label_of_antenna => {position_1, ..., position_n}
fn parse_map(
    reader: &mut BufReader<File>,
    map_height: &mut usize,
    map_width: &mut usize,
) -> Result<HashMap<char, Vec<(usize, usize)>>, Box<dyn Error>> {
    let mut map = HashMap::new();

    for (i, line) in reader.lines().into_iter().enumerate() {
        line?
            .chars()
            .enumerate()
            .filter(|(_, c)| *c != '.')
            .for_each(|(j, c)| {
                map.entry(c)
                    .and_modify(|vec: &mut Vec<_>| vec.push((j, i)))
                    .or_insert(vec![(j, i)]);
            });
        (*map_height) += 1;
        (*map_width) += 1;
    }

    Ok(map)
}

/// Counts first two antinodes alligned to input antennas
fn count_antinodes_part_one(
    antenna_a: &(usize, usize),
    antenna_b: &(usize, usize),
    map_height: usize,
    map_width: usize,
    taken_positions: &mut HashSet<(isize, isize)>,
) -> i32 {

    let dx = antenna_a.0 as isize - antenna_b.0 as isize;
    let dy = antenna_a.1 as isize - antenna_b.1 as isize; // Always < 0 as we parse the map in-order

    // First node from antenna_a's side 
    let antinode_up = (antenna_a.0 as isize + dx, antenna_a.1 as isize + dy);
    // First node from antenna_b's side
    let antinode_down = (antenna_b.0 as isize - dx, antenna_b.1 as isize - dy);

    // antinode down is inside map and was never taken before
    ((0..map_width as isize).contains(&antinode_down.0)
        && (0..map_height as isize).contains(&antinode_down.1)
        && taken_positions.insert(antinode_down)) as i32
        + 
        // antinode up is inside map and was never taken before
        ((0..map_width as isize).contains(&antinode_up.0)
            && (0..map_height as isize).contains(&antinode_up.1)
            && taken_positions.insert(antinode_up)) as i32
}

/// Counts all antinodes alligned to the input antennas
fn count_antinodes_part_two(
    antenna_a: &(usize, usize),
    antenna_b: &(usize, usize),
    map_height: usize,
    map_width: usize,
    taken_positions: &mut HashSet<(isize, isize)>,
) -> i32 {

    let dx = antenna_a.0 as isize - antenna_b.0 as isize;
    let dy = antenna_a.1 as isize - antenna_b.1 as isize;
    
    let mut sum = 0;

    // Count all antinodes from antenna_a's side (dy is always <= 0 as we parse the map in-order)
    let mut antinode = (antenna_a.0 as isize, antenna_a.1 as isize);
    while (0..map_width as isize).contains(&antinode.0) && (0..map_height as isize).contains(&antinode.1) {
        sum += taken_positions.insert(antinode) as i32;
        antinode.0 += dx;
        antinode.1 += dy;
    }

    // Count all antinodes from antenna_b's side
    let mut antinode = (antenna_b.0 as isize, antenna_b.1 as isize);
    while (0..map_width as isize).contains(&antinode.0) && (0..map_height as isize).contains(&antinode.1) {
        sum += taken_positions.insert(antinode) as i32;
        antinode.0 -= dx;
        antinode.1 -= dy;
    }

    sum
}

/// Solves second challenge
fn part_one(map: &HashMap<char, Vec<(usize, usize)>>, map_height: usize, map_width: usize) -> i32 {
    let mut taken_positions = HashSet::new();
    map.iter()
        .map(|(_, vec)| {
            // For each antenna label, take all antenna pairs and count antinodes
            vec.iter()
                .tuple_combinations()
                .map(|(a, b)| {
                    count_antinodes_part_one(
                        a,
                        b,
                        map_height,
                        map_width,
                        &mut taken_positions,
                    )
                })
                .sum::<i32>()
        })
        .sum()
}


/// Solves second challenge
fn part_two(map: &HashMap<char, Vec<(usize, usize)>>, map_height: usize, map_width: usize) -> i32 {
    let mut taken_positions = HashSet::new();
    map.iter()
        .map(|(_, vec)| {
            // For each antenna label, take all antenna pairs and count antinodes
            vec.iter()
                .tuple_combinations()
                .map(|(a, b)| {
                    count_antinodes_part_two(
                        a,
                        b,
                        map_height,
                        map_width,
                        &mut taken_positions,
                    )
                })
                .sum::<i32>()
        })
        .sum()
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut reader = BufReader::new(File::open(INPUT_PATH)?);
    let (mut map_height, mut map_width) = (0, 0);

    let map = parse_map(&mut reader, &mut map_height, &mut map_width)?;

    println!(
        "Result (part one): {}",
        part_one(&map, map_height, map_width)
    );

    println!(
        "Result (part two): {}",
        part_two(&map, map_height, map_width)
    );

    Ok(())
}
