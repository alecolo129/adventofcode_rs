use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

const FILE_PATH: &str = "day_fifteen/src/input.txt";

#[allow(dead_code)]
fn print_map(map: &Vec<Vec<Cell>>) {
    for v in map.clone() {
        for c in v {
            match c {
                Cell::Empty => {
                    print!(".");
                }
                Cell::LeftBox => {
                    print!("[");
                }
                Cell::RightBox => {
                    print!("]");
                }
                Cell::Box => {
                    print!("O");
                }
                Cell::Robot => {
                    print!("@");
                }
                _ => {
                    print!("#")
                }
            }
        }
        println!("");
    }
}

fn parse_input(
    map: &mut Vec<Vec<Cell>>,
    actions: &mut Vec<char>,
) -> Result<(usize, usize), Box<dyn Error>> {
    let mut pos_robot = (0, 0);

    let reader = BufReader::new(File::open(FILE_PATH)?);

    for (row_index, line) in reader.lines().enumerate() {
        let line: Vec<char> = line?.trim_end().chars().collect();
        let mut map_line = Vec::new();
        
        if line.is_empty() {
            continue;
        }

        line.iter().enumerate().for_each(|(col_index, c)|{
            match c {
                '@' => {
                    pos_robot = (row_index, col_index);
                    map_line.push(Cell::Robot);
                }

                'O' => {
                    map_line.push(Cell::Box);
                }

                '.' => {
                    map_line.push(Cell::Empty);
                }

                '#' => {
                    map_line.push(Cell::Wall);
                }
    
                '<' | '>' | 'v' | '^' => {
                    actions.push(*c);
                }

                _ => {}
            }
        });
        if !map_line.is_empty(){
            map.push(map_line);
        }
        
    }

    Ok(pos_robot)
}

fn get_deltas(label: &char) -> Result<(isize, isize), Box<dyn Error>> {
    match label {
        // Move left
        '<' => Ok((0, -1)),
        // Move right
        '>' => Ok((0, 1)),
        // Move up
        '^' => Ok((-1, 0)),
        // Move down
        'v' => Ok((1, 0)),
        // Invalid action label
        c => Err(format!("Invalid action: {}", c).into()),
    }
}

fn part_one(
    map: &mut Vec<Vec<Cell>>,
    actions: &mut Vec<char>,
    pos_robot: (usize, usize),
) -> Result<i32, Box<dyn Error>> {
    let map_height = map.len();
    let map_width = map[0].len();
    let (mut r, mut c) = pos_robot;

    for action in actions {
        let (mut i, mut j) = (r, c);
        let (d_r, d_c) = get_deltas(action)?;

        while (1..map_height).contains(&i)
            && (1..map_width).contains(&j)
            && !matches!(map[i][j], Cell::Wall | Cell::Empty)
        {
            i = i.wrapping_add_signed(d_r);
            j = j.wrapping_add_signed(d_c);
        }

        if (1..map_height).contains(&i)
            && (1..map_width).contains(&j)
            && !matches!(map[i][j], Cell::Wall | Cell::Robot)
        {
            loop {
                let (i_new, j_new) = (i.wrapping_add_signed(-d_r), j.wrapping_add_signed(-d_c));
                map[i][j] = map[i_new][j_new];
                if i_new == r && j_new == c {
                    break;
                }
                i = i_new;
                j = j_new;
            }
            map[r][c] = Cell::Empty;
            (r, c) = (i, j);
        }
    }

    // print_map(map);

    let mut res = 0;
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if matches!(map[i][j], Cell::Box) {
                res += (i) * 100 + j;
            }
        }
    }
    return Ok(res as i32);
}

#[derive(Clone, Copy, Debug)]
enum Cell {
    Robot,
    Box,
    LeftBox,
    RightBox,
    Wall,
    Empty,
}

fn move_vertically(
    map: &mut Vec<Vec<Cell>>,
    pos_robot: (usize, usize),
    action: char,
) -> Option<Vec<Vec<Cell>>> {
    let mut map = map.clone();
    let map_height = map.len();
    let (mut r, c) = pos_robot;

    let d_r = if action == 'v' { 1 } else { -1 };

    let mut boxes = vec![];


    let new_r = r.wrapping_add_signed(d_r);

    match map[new_r][c] {
        Cell::Empty => {
            map[r][c] = Cell::Empty;
            map[new_r][c] = Cell::Robot;
            return Some(map);
        }
        Cell::LeftBox => {
            map[r][c] = Cell::Empty;
            map[new_r][c] = Cell::Robot;
            map[new_r][c + 1] = Cell::Empty;
            boxes.push(c);
        }
        Cell::RightBox => {
            map[r][c] = Cell::Empty;
            map[new_r][c] = Cell::Robot;
            map[new_r][c - 1] = Cell::Empty;
            boxes.push(c - 1);
        }
        _ => {
            return None;
        }
    }

    r = new_r;


    let mut stop = false;
    while (1..map_height).contains(&r) && !boxes.is_empty() && !stop {
        stop = true;

        let new_r = r.wrapping_add_signed(d_r);


        if (1..map_height).contains(&new_r) {

            let old_boxes = boxes.clone();
            boxes.clear();

            for start in old_boxes {
                match (map[new_r][start], map[new_r][start + 1]) {

                    // Two empty blocks, we can just move up
                    (Cell::Empty, Cell::Empty) => {
                        // Move current block
                        map[new_r][start] = Cell::LeftBox;
                        map[new_r][start + 1] = Cell::RightBox;
                    }

                    // Next block is alligned, push up and continue
                    (Cell::LeftBox, Cell::RightBox) => {
                        // Move current block
                        map[new_r][start] = Cell::LeftBox;
                        map[new_r][start + 1] = Cell::RightBox;

                        // Add alligned block in blocks to move
                        boxes.push(start);
                        stop = false;
                    }

                    // Next block is not aligned
                    (Cell::Empty | Cell::RightBox, Cell::Empty | Cell::LeftBox) => {

                        // If we have a left block
                        if matches!(map[new_r][start], Cell::RightBox){
                            // Push left block and clean remaining space
                            boxes.push(start - 1);
                            map[new_r][start - 1] = Cell::Empty;
                        }
                        
                        // If we have a right block
                        if matches!(map[new_r][start+1], Cell::LeftBox){
                            // Push right block and clean remaining space
                            boxes.push(start + 1);
                            map[new_r][start + 2] = Cell::Empty;
                        }

                        // Move current block
                        map[new_r][start] = Cell::LeftBox;
                        map[new_r][start + 1] = Cell::RightBox;

                        stop = false;
                    }

                    // Next block contains walls
                    (_, _) => {
                        return None;
                    }
                }
            }
        }

        else{
            return None;
        }

        r = new_r;
    }

    if stop {
        return Some(map)
    }
 
    return None;
}

fn part_two(
    map: &mut Vec<Vec<Cell>>,
    actions: &mut Vec<char>,
    pos_robot: (usize, usize),
) -> Result<usize, Box<dyn Error>> {
    let map_height = map.len();
    let map_width = map[0].len();

    let (mut r, mut c) = pos_robot;

    for action in actions.iter() {
        let (mut i, mut j) = (r, c);

        let (d_r, d_c) = get_deltas(action)?;

        match action {
            '<' | '>' => {
                while (1..map_height).contains(&i)
                    && (1..map_width).contains(&j)
                    && !matches!(map[i][j], Cell::Wall | Cell::Empty)
                {
                    i = i.wrapping_add_signed(d_r);
                    j = j.wrapping_add_signed(d_c);
                }
                if (1..map_height).contains(&i)
                    && (1..map_width).contains(&j)
                    && matches!(map[i][j], Cell::Empty)
                {
                    loop {
                        let j_new = j.wrapping_add_signed(-d_c);
                        map[i][j] = map[i][j_new];
                        if j_new == c {
                            break;
                        }
                        j = j_new;
                    }
                    map[r][c] = Cell::Empty;
                    (r, c) = (i, j);
                }
            }

            '^' | 'v' => {
                if let Some(mut new_map) = move_vertically(map, (r, c), *action) {
                    std::mem::swap(map, &mut new_map);
                    if *action == 'v' {
                        r = r + 1;
                    } else {
                        r = r - 1;
                    }
                }
            }

            _ => {}
        }
    }


    let mut res = 0;
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if matches!(map[i][j], Cell::LeftBox) {
                res += i * 100 + j;
            }
        }
    }
    return Ok(res);
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut map = Vec::new();
    let mut actions = Vec::new();
    let mut pos_robot = parse_input(&mut map, &mut actions)?;

    println!(
        "Result (part one): {}",
        part_one(&mut map.clone(), &mut actions, pos_robot)?
    );


    let mut map: Vec<Vec<Cell>> = map
    .into_iter()
    .map(|v| {
        v.into_iter()
            .map(|c| match c {
                Cell::Empty => [Cell::Empty, Cell::Empty],

                Cell::Box => [Cell::LeftBox, Cell::RightBox],

                Cell::Robot => [Cell::Robot, Cell::Empty],

                _ => [Cell::Wall, Cell::Wall],
            })
            .flatten()
            .collect()
    })
    .collect();

    pos_robot.1 = 2 * pos_robot.1;


    println!(
        "Result (part two): {}",
        part_two(&mut map, &mut actions, pos_robot)?
    );

    Ok(())
}
