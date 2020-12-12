use std::time::{Instant};
use std::io::{Error};
use std::fs;

fn main() -> Result<(), Error> {
    // seeing as pretty much every puzzle is going to be reading in a file and then manipulating the input, 
    // I might as well just build main out to be a template

    // read the input for today's puzzle
    let filepath = "input.txt";
    let file_data = fs::read_to_string(filepath).expect("failed to read file"); // returns a vector of strings split by line

    // part 1
    let start = Instant::now();
    part_1(&file_data);
    let duration = start.elapsed();
    println!("Time elapsed in part 1: {:?}", duration);
    
    // part 2
    let start = Instant::now();
    part_2(&file_data);
    let duration = start.elapsed();
    println!("Time elapsed in part 2: {:?}", duration);

    Ok(())
}

fn part_1(file_data: &String) {
    let mut splits: Vec<&str> = file_data.split("\n").collect(); // this will split each line seperately

    let mut coords = (0, 0); // x is east, y is north
    let mut facing = (1, 0); // the facing direction is a vector (in the math sense)

    for line in splits {
        // split out first character as the direction and second as movement
        let direction_char: Vec<&str> = line.split(|c: char| c.is_numeric()).filter(|&x| !x.is_empty()).collect();
        let move_char: Vec<&str> = line.split(|c: char| c.is_alphabetic()).filter(|&x| !x.is_empty()).collect();
    
        // println!("{:?}, {:?}", direction_char, move_char[0].parse::<isize>().unwrap());

        match direction_char[0] {
            "N" => coords.1 += move_char[0].parse::<isize>().unwrap(),
            "S" => coords.1 -= move_char[0].parse::<isize>().unwrap(),
            "E" => coords.0 += move_char[0].parse::<isize>().unwrap(),
            "W" => coords.0 -= move_char[0].parse::<isize>().unwrap(),
            "L" | "R" => {
                let mut move_float = move_char[0].parse::<f32>().unwrap();
                if direction_char[0] == "R" {move_float*=-1.0f32};
                let mut temp_facing = (0,0);
                temp_facing.0 = (facing.0 * (move_float.to_radians().cos() as isize)) - (facing.1 * (move_float.to_radians().sin() as isize));
                temp_facing.1 = (facing.0 * (move_float.to_radians().sin() as isize)) + (facing.1 * (move_float.to_radians().cos() as isize));
                facing.0 = temp_facing.0;
                facing.1 = temp_facing.1;
            },
            "F" => {
                coords.0 += move_char[0].parse::<isize>().unwrap() * facing.0;
                coords.1 += move_char[0].parse::<isize>().unwrap() * facing.1;
            }
            _ => ()
        }
        
        // println!("{:?} | {:?} | {:?}", coords, facing, line);

    }

        
    println!("manhattan distance | {:?}", coords.0.abs() + coords.1.abs());
}

fn part_2(file_data: &String) {
    let mut splits: Vec<&str> = file_data.split("\n").collect(); // this will split each line seperately

    let mut ship_coords = (0, 0); // x is east, y is north
    let mut waypoint_coords = (10, 1); // x is east, y is north

    for line in splits {
        // split out first character as the direction and second as movement
        let direction_char: Vec<&str> = line.split(|c: char| c.is_numeric()).filter(|&x| !x.is_empty()).collect();
        let move_char: Vec<&str> = line.split(|c: char| c.is_alphabetic()).filter(|&x| !x.is_empty()).collect();
    
        // println!("{:?}, {:?}", direction_char, move_char[0].parse::<isize>().unwrap());

        match direction_char[0] {
            "N" => waypoint_coords.1 += move_char[0].parse::<isize>().unwrap(),
            "S" => waypoint_coords.1 -= move_char[0].parse::<isize>().unwrap(),
            "E" => waypoint_coords.0 += move_char[0].parse::<isize>().unwrap(),
            "W" => waypoint_coords.0 -= move_char[0].parse::<isize>().unwrap(),
            "L" | "R" => {
                let mut move_float = move_char[0].parse::<f32>().unwrap();
                if direction_char[0] == "R" {move_float*=-1.0f32};
                let mut temp_facing = (0,0);
                temp_facing.0 = (waypoint_coords.0 * (move_float.to_radians().cos() as isize)) - (waypoint_coords.1 * (move_float.to_radians().sin() as isize));
                temp_facing.1 = (waypoint_coords.0 * (move_float.to_radians().sin() as isize)) + (waypoint_coords.1 * (move_float.to_radians().cos() as isize));
                waypoint_coords.0 = temp_facing.0;
                waypoint_coords.1 = temp_facing.1;
            },
            "F" => {
                ship_coords.0 += move_char[0].parse::<isize>().unwrap() * waypoint_coords.0;
                ship_coords.1 += move_char[0].parse::<isize>().unwrap() * waypoint_coords.1;
            }
            _ => ()
        }
        
        // println!("{:?} | {:?} | {:?}", ship_coords, waypoint_coords, line);

    }

        
    println!("manhattan distance | {:?}", ship_coords.0.abs() + ship_coords.1.abs());
   
}