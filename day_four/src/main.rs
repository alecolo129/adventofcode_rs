/*
    You can find the challenge description here: https://adventofcode.com/2024/day/4
*/
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufRead};

fn parse_matrix(reader: &mut BufReader<File>) -> Result<Vec<Vec<char>>, Box<dyn Error>>{
    let mut matrix= Vec::new();
    for line in reader.lines(){
        if let Some(line_str) = line.ok() {
            matrix.push(line_str.chars().collect::<Vec<char>>());
        }
    }
    Ok(matrix)
}

fn part_one(matrix: &Vec<Vec<char>>) -> Result<i32, Box<dyn Error>>{
    let mut tot = 0;
    let size = matrix.len();
    for i in 0..size{
        for j in 0..size{
            if matrix[i][j] == 'X' {
                
                // Look right
                if j+3 < size{
                    if (matrix[i][j+1], matrix[i][j+2], matrix[i][j+3]) == ('M', 'A', 'S') {
                        tot+=1;
                    }
                }

                // Look left
                if j >= 3{
                    if(matrix[i][j-1], matrix[i][j-2], matrix[i][j-3]) == ('M', 'A', 'S'){
                        tot+=1;
                    }
                }

                // Look down 
                if i+3 < size {
                    // Look vertical line
                    if (matrix[i+1][j], matrix[i+2][j], matrix[i+3][j]) == ('M', 'A', 'S') {
                        tot+=1;
                    }

                    // Look orizontal left
                    if j>=3 && (matrix[i+1][j-1], matrix[i+2][j-2], matrix[i+3][j-3]) == ('M', 'A', 'S') {
                        tot+=1;
                    }

                    // Look orizontal right
                    if  j+3 < size && (matrix[i+1][j+1], matrix[i+2][j+2], matrix[i+3][j+3]) == ('M', 'A', 'S') {
                        tot+=1;
                    }
                }

                // Look up
                if i >= 3{
                    // Look vertical line
                    if (matrix[i-1][j], matrix[i-2][j], matrix[i-3][j]) == ('M', 'A', 'S') {
                        tot+=1;
                    }

                    // Look orizontal left
                    if j>=3 && (matrix[i-1][j-1], matrix[i-2][j-2], matrix[i-3][j-3]) == ('M', 'A', 'S') {
                        tot+=1;
                    }

                    // Look orizontal right
                    if  j+3 < size && (matrix[i-1][j+1], matrix[i-2][j+2], matrix[i-3][j+3]) == ('M', 'A', 'S') {
                        tot+=1;
                    }
                }
            }
        }
    }
    Ok(tot)
}

fn part_two(matrix: &Vec<Vec<char>>) -> Result<i32, Box<dyn Error>>{
    let mut tot = 0;
    let rows = matrix.len();
    let columns = matrix[0].len();
    for i in 0..rows{
        for j in 0..columns{

            if j > 0 && j+1 < columns && i>0 && i+1<rows{
                /* Match:
                   M S     S S
                    A  ,    A
                   M S     M M
                */
                if (matrix[i-1][j+1], matrix[i][j], matrix[i+1][j-1]) == ('S', 'A', 'M') {
                    if(matrix[i-1][j-1], matrix[i+1][j+1]) == ('M', 'S'){
                        tot+=1;
                    }
                    if(matrix[i-1][j-1], matrix[i+1][j+1]) == ('S', 'M'){
                        tot+=1;
                    }
                }

                /* Match:
                   M M     S M
                    A  ,    A
                   S S     S M
                */
                else if (matrix[i-1][j+1], matrix[i][j], matrix[i+1][j-1]) == ('M', 'A', 'S') {
                    if(matrix[i-1][j-1], matrix[i+1][j+1]) == ('M', 'S'){
                        tot+=1;
                    }
                    if(matrix[i-1][j-1], matrix[i+1][j+1]) == ('S', 'M'){
                        tot+=1;
                    }
                }
            }
        }
    }
    Ok(tot)
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
    
    let mut reader = BufReader::new(File::open( "day_four/src/input.txt")?);
    let matrix = parse_matrix(&mut reader)?;

    let total = part_one(&matrix)?;    
    println!("Total (part one): {}", total);
    
    let total = part_two(&matrix)?;
    println!("Total (part two): {}", total);

    Ok(())
}
