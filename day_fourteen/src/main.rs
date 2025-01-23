use regex::Regex;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

const INPUT_PATH: &str = "day_fourteen/src/input.txt";
const X_LIMIT: i32 = 101;
const Y_LIMIT: i32 = 103;
const TIME: i32 = 100;

fn print_tree(map: &HashMap<(i32, i32), i32>) {
    println!("Tree (part two): ");
    for i in 0..X_LIMIT {
        for j in 0..Y_LIMIT {
            match map.get(&(i, j)) {
                Some(entry) => {
                    if *entry != 0 {
                        print!("{}", entry);
                    } else {
                        print!(".")
                    }
                }
                None => {
                    print!(".");
                }
            }
        }
        println!("");
    }
}

fn is_valid_tree(map: &HashMap<(i32, i32), i32>) -> bool {
    let mut found = false;
    let threshold = 1;

    'outer: for (x, y) in map.keys() {
        // Try to find many robots disposed over the diagonal lines centered in (x,y). As this implies a dense area, a threshold of 1 is sufficient for seeing the tree.
        for c in 0..X_LIMIT {
            let down_left = map.get(&(x - c, y + c)).unwrap_or(&0);
            let down_right = map.get(&(x + c, y + c)).unwrap_or(&0);
            let up_left = map.get(&(x - c, y - c)).unwrap_or(&0);
            let up_right = map.get(&(x + c, y - c)).unwrap_or(&0);

            // We are not in a dense area
            if *up_left == 0 || *up_right == 0 || *down_left == 0 || *down_right == 0 {
                break;
            }

            // We have found a dense area
            if c > threshold {
                found = true;
                break 'outer;
            }
        }
    }

    return found;
}

fn part_two(coordinates: &Vec<Vec<i32>>, velocity_vec: &Vec<Vec<i32>>) -> i32 {
    
    let mut time = 1;
    let mut new_coordinates = coordinates.clone();
    let mut map = new_coordinates.iter().fold(HashMap::new(), |mut acc, c| {
        *acc.entry((c[0], c[1])).or_insert(0) += 1;
        acc
    });

    loop {
        new_coordinates
            .iter_mut()
            .zip(velocity_vec)
            .for_each(|(c, v)| {
                let entry = map.get_mut(&(c[0], c[1])).unwrap();
                if *entry == 1 {
                    map.remove(&(c[0], c[1]));
                } else {
                    *entry -= 1;
                }

                c[0] = (c[0] + v[0]) % X_LIMIT;
                c[0] = if c[0] >= 0 { c[0] } else { c[0] + X_LIMIT };
                c[1] = (c[1] + v[1]) % Y_LIMIT;
                c[1] = if c[1] >= 0 { c[1] } else { c[1] + Y_LIMIT };
                *map.entry((c[0], c[1])).or_insert(0) += 1;
            });

        if is_valid_tree(&map) {
            print_tree(&map);
            return time;
        }

        time += 1;
    }
}

fn part_one(coordinates: &Vec<Vec<i32>>, velocities: &Vec<Vec<i32>>) -> i32 {

    // Get middle column and row (assumimg odd number of rows/columns)
    let (mid_x, mid_y) = (X_LIMIT/2, Y_LIMIT/2); 

    // Counters for each quadrant
    let (mut up_left, mut up_right, mut down_left, mut down_right) = (0, 0, 0, 0);


    for (coordinate, velocity) in coordinates.iter().zip(velocities){
        let res_x = (coordinate[0] + TIME * velocity[0]) % X_LIMIT;
        let res_x = if res_x >= 0 { res_x } else { X_LIMIT + res_x };

        let res_y = (coordinate[1] + TIME * velocity[1]) % Y_LIMIT;
        let res_y = if res_y >= 0 { res_y } else { Y_LIMIT + res_y };
        
        // Assign drone to quadrant (if possible)
        if res_x < mid_x {
            if res_y < mid_y {
                up_left += 1;
            } else if res_y > mid_y {
                down_left += 1;
            }
        } else if res_x > mid_x {
            if res_y < mid_y {
                up_right += 1;
            } else if res_y > mid_y {
                down_right += 1;
            }
        }
    
    }

    // Return safety factor
    return up_left * up_right * down_left * down_right;
}

fn main() -> Result<(), Box<dyn Error>> {
    let reader = BufReader::new(File::open(INPUT_PATH)?);

    let re = Regex::new(r"([a-z]+)=([-]?\d+),([-]?\d+)")?;

    let mut coordinates = Vec::new();
    let mut velocities = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let captures = re.captures_iter(&line);
        let mut coordinate = Vec::new();
        let mut velocity = Vec::new();

        for capture in captures {
            if &capture[1] == "p" {
                coordinate.extend([
                    capture[2].parse::<i32>().unwrap(),
                    capture[3].parse::<i32>().unwrap(),
                ]);
            } else {
                velocity.extend([
                    capture[2].parse::<i32>().unwrap(),
                    capture[3].parse::<i32>().unwrap(),
                ]);
            }
        }

        coordinates.push(coordinate);
        velocities.push(velocity);
    }

    let result_1 =  part_one(&coordinates, &velocities);
    let result_2 = part_two(&coordinates, &velocities);
    
    println!(
        "Result (part one): {}",
        result_1
    );

    println!(
        "Result (part two): {} (look at picture above)",
        result_2
    );

    Ok(())
}
