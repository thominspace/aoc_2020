use std::time::{Instant};
use std::io::{Error};
use std::fs;
use std::collections::HashMap;

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

    let mut total_yes_answers = 0;
    let splits: Vec<&str> = file_data.split("\n\n").collect(); // this will split each doublenewline seperately
    let mut question_checker = HashMap::new();
    for this_str in splits.iter() {
        // println!("{:?}", this_str.chars());
        for this_char in this_str.chars() {
            if this_char != '\n' { 
                question_checker.entry(this_char).or_insert(true); 
            }
        }
            
        // println!("number of keys: {:?}", question_checker.keys().len());
        total_yes_answers += question_checker.keys().len();
        question_checker.clear();
       
    }


    println!("total yes answers: {:?}", total_yes_answers);
    
}

fn part_2(file_data: &String) {

    let mut total_yes_answers = 0;
    let mut total_group_members = 1;
    let splits: Vec<&str> = file_data.split("\n\n").collect(); // this will split each doublenewline seperately
    let mut question_checker = HashMap::new();
    for this_str in splits.iter() {
        // println!("{:?}", this_str.chars());
        for this_char in this_str.chars() {
            if this_char == '\n' { 
                total_group_members += 1;
            } else {
                let count = question_checker.entry(this_char).or_insert(0);
                *count += 1;
            }
        }
        // println!("{:?}", question_checker);
            
        // println!("number of members: {:?}", total_group_members);
        // total_group_members += 1;
        // now check for where everyone said yes
        for (_, val) in question_checker.iter() {
            // println!("members: {:?} value {:?}", total_group_members, val);
            if val == &total_group_members {    
                total_yes_answers += 1;
            }
        }
        total_group_members = 1;
        question_checker.clear();
       
    }


    println!("total yes answers: {:?}", total_yes_answers);
    
}
