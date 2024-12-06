/*
    You can find the challenge description here: https://adventofcode.com/2024/day/6
*/

use std::fs::File;
use std::io::{BufRead, BufReader};

fn is_edge_position(map: &Vec<Vec<bool>>, guard: char, position: &(usize, usize)) -> bool {
    match guard {
        '^' => position.0 == 0,
        '>' => position.1 == map[0].len() - 1,
        'v' => position.0 == map.len() - 1,
        '<' => position.1 == 0,
        _ => {
            eprintln!("Error: invalid guard char '{}'", guard);
            true
        }
    }
}

/// Parses input map saving obstacle as true and anything else as false. Sets the input 'guard_position' and 'guard' respectively as the (row, column) coordinates and guard label.
fn parse_map(
    reader: &mut BufReader<File>,
    guard_position: &mut (usize, usize),
    guard: &mut char,
) -> Result<Vec<Vec<bool>>, Box<dyn std::error::Error>> {
    let mut map = Vec::new();

    reader.lines().into_iter().try_for_each(|line| {
        let row = line?
            .chars()
            .enumerate()
            .map(|(col, ch)| match ch {
                '.' => false,
                '#' => true,
                _ => {
                    *guard_position = (map.len(), col);
                    *guard = ch;
                    false
                }
            })
            .collect();

        map.push(row);

        Ok::<(), Box<dyn std::error::Error>>(())
    })?;

    Ok(map)
}

// Solves part one, modifies the matrix 'visited' so that all visited positions are set to true 
fn part_one(
    map: &Vec<Vec<bool>>,
    visited: &mut Vec<Vec<char>>,
    guard_position: &(usize, usize),
    mut guard: char,
) -> i32 {
    let mut count_steps = 1;
    let (mut r, mut c) = *guard_position;
    visited[r][c] = guard;

    while !is_edge_position(map, guard, &(r, c)) {
        match guard {
            '>' => {
                if !map[r][c + 1] {
                    if visited[r][c + 1] == '\0' {
                        count_steps += 1;
                        visited[r][c + 1] = guard;
                    } else if visited[r][c + 1] == guard {
                        return -1;
                    }
                    c += 1;
                } else {
                    guard = 'v';
                }
            }

            'v' => {
                if !map[r + 1][c] {
                    if visited[r + 1][c] == '\0' {
                        count_steps += 1;
                        visited[r + 1][c] = guard;
                    } else if visited[r + 1][c] == guard {
                        return -1;
                    }
                    r += 1;
                } else {
                    guard = '<';
                }
            }

            '<' => {
                if !map[r][c - 1] {
                    if visited[r][c - 1] == '\0' {
                        count_steps += 1;
                        visited[r][c - 1] = guard;
                    } else if visited[r][c - 1] == guard {
                        return -1;
                    }
                    c -= 1;
                } else {
                    guard = '^';
                }
            }

            '^' => {
                if !map[r - 1][c] {
                    if visited[r - 1][c] == '\0' {
                        count_steps += 1;
                        visited[r - 1][c] = guard;
                    } else if visited[r - 1][c] == guard {
                        return -1;
                    }
                    r -= 1;
                } else {
                    guard = '>';
                }
            }
            _ => {
                eprintln!("Error: invalid guard character {}", guard)
            }
        }
    }

    count_steps
}

fn part_two(
    map: &mut Vec<Vec<bool>>,
    visited: &mut Vec<Vec<char>>,
    guard_position: &(usize, usize),
    guard: char,
) -> i32 {
    let mut tot = 0;
    for r in 0..map.len() {
        for c in 0..map[0].len() {
            // Ignore all cells with an obstacle or that were never visited
            if !map[r][c] && visited[r][c] != '\0' {
                map[r][c] = true;
                if part_one(
                    map,
                    &mut vec![vec!['\0'; map[0].len()]; map.len()],
                    guard_position,
                    guard,
                ) == -1
                {
                    tot += 1;
                }
                map[r][c] = false;
            }
        }
    }
    tot
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("day_six/src/input.txt")?;
    let mut reader = BufReader::new(file);

    let mut guard_position = (0, 0);
    let mut guard = ' ';
    let mut map = parse_map(&mut reader, &mut guard_position, &mut guard)?;

    let mut visited = vec![vec!['\0'; map[0].len()]; map.len()];
    let result = part_one(&map, &mut visited, &mut guard_position, guard);
    println!("Result (part one): {result}");

    let result = part_two(&mut map, &mut visited, &mut guard_position, guard);
    println!("Result (part two): {result}");

    Ok(())
}
