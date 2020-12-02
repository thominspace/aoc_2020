// For this puzzle we want to take a list of numbers and find the two numbers in a list that add to 2020, and then multiply them.
// Easy enough!

// Let's assume the input data is clean. At first I thought of assuming that all numbers were positive, but honestly that doesn't really change too much.
// I suppose the way to tackle this is to just compare line by line down the hatch until we find our pair?

// actually, if we sort first, we can do some fun stuff by comparing the edges.
// maybe leave that as an exercise to the reader

use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};
use std::path::Path;
use std::time::{Duration, Instant};
use std::env;
use std::collections::HashMap;
use std::fs;

fn main() -> Result<(), Error> {
    let start = Instant::now();
    part_1();
    let duration = start.elapsed();
    println!("Time elapsed in part 1: {:?}", duration);
    
    let start = Instant::now();
    part_2();
    let duration = start.elapsed();
    println!("Time elapsed in part 2: {:?}", duration);

    Ok(())
}

fn part_2() {
    let filepath = "input.txt";
    let vec = read_file(filepath).expect("failed to read file");
    let mut good_pass_count = 0;
    for ix in 0..vec.len() {
        // iterate through strings
        // splits based on whitespace
        // should leave us with [numbers, key, str]
        let split_str: Vec<&str> = vec[ix].split(" ").collect();

        // split the numbers portions again by the "-" and convert to numbers
        let split_numbers: Vec<i64> = split_str[0].split("-").map(|x| x.parse::<i64>().unwrap()).collect();

        // split the key by just pulling the first character
        let key_char = split_str[1].chars().nth(0).unwrap();

        // if the password conforms, increase the count
        good_pass_count += (((split_str[2].chars().nth((split_numbers[0]-1) as usize).unwrap() == key_char) as isize) + ((split_str[2].chars().nth((split_numbers[1]-1) as usize).unwrap() == key_char) as isize)) % 2;
        // println!("{} {} {} {}", (split_str[2].chars().nth((split_numbers[0]-1) as usize).unwrap() == key_char) as isize,
        //                         (split_str[2].chars().nth((split_numbers[1]-1) as usize).unwrap() == key_char) as isize,
        //                         (((split_str[2].chars().nth((split_numbers[0]-1) as usize).unwrap() == key_char) as isize) + ((split_str[2].chars().nth((split_numbers[1]-1) as usize).unwrap() == key_char) as isize)) % 2,
        //                         good_pass_count)
    }
    println!("found {} conforming passwords", good_pass_count)
}

fn part_1() {
    let filepath = "input.txt";
    let vec = read_file(filepath).expect("failed to read file");
    let mut good_pass_count = 0;
    for ix in 0..vec.len() {
        // iterate through strings
        // splits based on whitespace
        // should leave us with [numbers, key, str]
        let split_str: Vec<&str> = vec[ix].split(" ").collect();

        // split the numbers portions again by the "-" and convert to numbers
        let split_numbers: Vec<i64> = split_str[0].split("-").map(|x| x.parse::<i64>().unwrap()).collect();

        // split the key by just pulling the first character
        let key_char = split_str[1].chars().nth(0).unwrap();

        // now iterate through the password to match the key char
        let mut char_count = 0;
        for chx in split_str[2].chars() {            
            if key_char == chx {
                char_count += 1;
            }
        }
        // if the password conforms, increase the count
        if (char_count >= split_numbers[0]) && (char_count <= split_numbers[1]) {
            good_pass_count += 1;
        }
    }
    println!("found {} conforming passwords", good_pass_count)
}

fn read_file(path: &str) -> Result<Vec<String>, Error> {

    // let path = "input_long.txt";

    let file = File::open(path)?; // open file by given path
    // wrap file into generic buffered reader, it will use 4 KB buffer internally
    // to reduce number of syscalls, thus improving performance
    let br = BufReader::new(file);
    // create an empty vector, type of the stored elements will be inferred
    let mut v = Vec::new();
    // br.lines() creates an iterator over lines in the reader
    // see: https://doc.rust-lang.org/std/io/trait.BufRead.html#method.lines
    for line in br.lines() {
        // IO operations generally can return error, we check if got
        // an error,in which case we return this error as the function result
        let line = line?;
        let n = line;
        v.push(n); // push acquired integer to the vector
    }
    Ok(v) // everything is Ok, return vector
}