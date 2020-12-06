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

    // replace everything with 1 and 0;
    let replaced_data = file_data.replace("B", "1").replace("F", "0").replace("R", "1").replace("L", "0");
    // convert to an int
    let conv_data = replaced_data.split("\n").map(|x| usize::from_str_radix(x, 2).unwrap()).collect::<Vec<_>>();
    // print the max
    println!("max: {:?}", conv_data.iter().max().unwrap());

    // ok but sanity check a.k.a. the slow way

    // let mut largest_id = 0;
    // // split into lines
    // let split_data: Vec<&str>= replaced_data.split("\n").collect();
    // for this_str in split_data { // get one line
    //     let mut temp_sum: u32 = 0;
    //     for (ix, ch) in this_str.chars().enumerate() {
    //         match ch {
    //             '1' => temp_sum += 2u32.pow(9-(ix as u32)),
    //             _ => ()
    //         }      
    //     }
    //     if temp_sum > largest_id { 
    //         largest_id = temp_sum;
    //     } 

    // println!("sanitch check max {:?}", largest_id);


}

fn part_2(file_data: &String) {

    // part 2 is like part 1, but we care about the min too
    // sorting sounds like a great idea here

    // replace everything with 1 and 0;
    let replaced_data = file_data.replace("B", "1").replace("F", "0").replace("R", "1").replace("L", "0");
    // convert to an int
    let mut conv_data = replaced_data.split("\n").map(|x| usize::from_str_radix(x, 2).unwrap()).collect::<Vec<_>>();
    conv_data.sort();
    let min_val = conv_data.iter().min().unwrap();
    'checkloop: for (ix, val) in conv_data.iter().enumerate() {
        if val-min_val != ix {
            println!("gap at ix {} which is val {}", val-min_val, val);
            println!("my seat is {}", val-1);
            break 'checkloop
        }
    }
}