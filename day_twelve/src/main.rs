/*
You can find the challenge description here: https://adventofcode.com/2024/day/12
*/
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

const INPUT_PATH: &str = "day_twelve/src/input.txt";

fn parse_map(reader: &mut BufReader<File>) -> Result<Vec<Vec<char>>, Box<dyn Error>> {
    let mut map = Vec::new();
    reader.lines().try_for_each(|line| {
        map.push(line?.chars().collect());
        Ok::<(), Box<dyn Error>>(())
    })?;
    Ok(map)
}

fn eval_boundaries(map: &Vec<Vec<char>>, point: (usize, usize)) -> (bool, bool, bool, bool) {
    let (i, j) = point;
    let up = (1..map.len()).contains(&i);
    let down = (0..map.len()).contains(&(i + 1));
    let left = (1..map[0].len()).contains(&j);
    let right = (0..map[0].len()).contains(&(j + 1));
    (up, down, left, right)
}

fn visit_part_one(
    map: &Vec<Vec<char>>,
    visited: &mut Vec<Vec<bool>>,
    point: (usize, usize),
) -> (u32, u32) {
    let (i, j) = point;

    visited[i][j] = true;

    let label = map[i][j];

    let mut area = 1;
    let mut perimeter = 0;

    let (up, down, left, right) = eval_boundaries(map, point);

    let choices = [
        (up, (-1, 0)),
        (down, (1, 0)),
        (left, (0, -1)),
        (right, (0, 1)),
    ];

    for (direction, (delta_x, delta_y)) in choices {
        let (new_i, new_j) = ((i as i32  + delta_x) as usize, (j as i32 + delta_y) as usize);
        
        if direction && map[new_i][new_j] == label {
            
            if !visited[new_i][new_j] {
                let (area_to_add, perimeter_to_add) = visit_part_one(map, visited, (new_i, new_j));
                area += area_to_add;
                perimeter += perimeter_to_add;
            }
        } else {
            perimeter += 1;
        }
    }

    (area, perimeter)
}

fn visit_part_two(
    map: &Vec<Vec<char>>,
    visited: &mut Vec<Vec<bool>>,
    point: (usize, usize),
) -> (u32, u32) {
    let (i, j) = point;

    visited[i][j] = true;

    let label = map[i][j];

    let mut area = 1;
    let mut sides = 0;

    let (up, down, left, right) = eval_boundaries(map, point);

    // Look for external _|
    if (!down || map[i + 1][j] != label) && (!right || map[i][j + 1] != label) {
        sides += 1;
    }
    // Look for internal _|
    if up && right && map[i][j + 1] == label && map[i - 1][j + 1] == label && map[i - 1][j] != label
    {
        sides += 1;
    }

    // Look for external ¯|
    if (!up || map[i - 1][j] != label) && (!right || map[i][j + 1] != label) {
        sides += 1;
    }
    // Look for internal ¯|
    if down
        && right
        && map[i][j + 1] == label
        && map[i + 1][j + 1] == label
        && map[i + 1][j] != label
    {
        sides += 1;
    }

    // Look for external |_
    if (!down || map[i + 1][j] != label) && (!left || map[i][j - 1] != label) {
        sides += 1;
    }
    // Look for internal |_
    if up && left && map[i][j - 1] == label && map[i - 1][j - 1] == label && map[i - 1][j] != label
    {
        sides += 1;
    }

    // Look for external |¯
    if (!up || map[i - 1][j] != label) && (!left || map[i][j - 1] != label) {
        sides += 1;
    }
    // Look for internal |¯
    if down
        && left
        && map[i][j - 1] == label
        && map[i + 1][j - 1] == label
        && map[i + 1][j] != label
    {
        sides += 1;
    }

    // Look left
    if left && map[i][j - 1] == label {
        if !visited[i][j - 1] {
            let (area_left, sides_left) = visit_part_two(map, visited, (i, j - 1));
            area += area_left;
            sides += sides_left;
        }
    }

    // Look right
    if right && map[i][j + 1] == label {
        if !visited[i][j + 1] {
            let (area_right, sides_right) = visit_part_two(map, visited, (i, j + 1));
            area += area_right;
            sides += sides_right;
        }
    }

    // Look up
    if up && map[i - 1][j] == label {
        if !visited[i - 1][j] {
            let (area_up, sides_up) = visit_part_two(map, visited, (i - 1, j));
            area += area_up;
            sides += sides_up;
        }
    }

    // Look down
    if down && map[i + 1][j] == label {
        if !visited[i + 1][j] {
            let (area_up, sides_down) = visit_part_two(map, visited, (i + 1, j));
            area += area_up;
            sides += sides_down;
        }
    }

    (area, sides)
}

fn part_one(map: &Vec<Vec<char>>) -> u32 {
    let mut visited = vec![vec![false; map[0].len()]; map.len()];
    let mut sum = 0;

    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if !visited[i][j] {
                let (area, perimeter) = visit_part_one(map, &mut visited, (i, j));
                sum += area * perimeter;
            }
        }
    }

    sum
}

fn part_two(map: &Vec<Vec<char>>) -> u32 {
    let mut visited = vec![vec![false; map[0].len()]; map.len()];
    let mut sum = 0;

    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if !visited[i][j] {
                let (area, sides) = visit_part_two(map, &mut visited, (i, j));
                sum += area * sides;
            }
        }
    }

    sum
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut reader = BufReader::new(File::open(INPUT_PATH)?);
    let map = parse_map(&mut reader)?;
    println!("Result (part one): {}", part_one(&map));
    println!("Result (part two): {}", part_two(&map));

    Ok(())
}
