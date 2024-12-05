use std::fs::File;
use std::io::{BufReader, BufRead};

fn parse_vectors(reader: &mut BufReader<File>)-> Result<(Vec<i32>, Vec<i32>), Box<dyn std::error::Error>>{
    let (mut vec_a, mut vec_b) = (Vec::new(), Vec::new());

    for line in reader.lines(){
        match line {
            Ok(line) => {
                let values: Vec<_> = line.split("   ").collect();
                vec_a.push(values[0].parse::<i32>()?);
                vec_b.push(values[1].parse::<i32>()?);
            }
            Err(e) => eprintln!("Error occured: {}", e)
        }
    }

    Ok((vec_a, vec_b))
}

/// Solves part one sorting input vectors
fn part_one(vec_a: &mut Vec<i32>, vec_b: &mut Vec<i32>)->i32{
    vec_a.sort(); vec_b.sort();
    vec_a.iter().zip(vec_b.iter()).map(|(el_a, el_b)| (el_a-el_b).abs()).sum()
}

/// Solves part two assuming input vectors are already sorted
fn part_two(vec_a: &mut Vec<i32>, vec_b: &mut Vec<i32>)->i128{
    let mut result = 0i128;
    let mut j=0;
    for i  in 0..vec_a.len(){
        let mut count = 0;
        while j<vec_b.len() && vec_b[j] <= vec_a[i] {
            if vec_a[i] == vec_b[j] {
                count+=1;
            }
            j+=1; 
        };
        result += i128::from(vec_a[i]) * count;
    }
    result
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("day_one/src/input.txt")?;
    let mut reader = BufReader::new(file);
    
    let (mut vec_a, mut vec_b) = parse_vectors(&mut reader)?;
    
    let result = part_one(&mut vec_a, &mut vec_b);
    println!("Result (part one): {result}");
    
    let result = part_two(&mut vec_a, &mut vec_b);
    println!("Result (part two): {result}");
    Ok(())
}
