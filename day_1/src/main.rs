// For this puzzle we want to take a list of numbers and find the two numbers in a list that add to 2020, and then multiply them.
// Easy enough!

// Let's assume the input data is clean. At first I thought of assuming that all numbers were positive, but honestly that doesn't really change too much.
// I suppose the way to tackle this is to just compare line by line down the hatch until we find our pair?

// actually, if we sort first, we can do some fun stuff by comparing the edges.
// maybe leave that as an exercise to the reader

use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};
use std::path::Path;

fn main() -> Result<(), Error> {
    part_1();
    part_2();
    Ok(())
}

fn part_2() {
    // find the first set of indices that add to 2020
    let vec = read_file().expect("failed to read file");
    'outer: for ix in 0..vec.len()-1 {
        for iy in 1..vec.len() {
            if vec[ix]+vec[iy] <= 2020 {
                for iz in 1..vec.len() {
                    if vec[ix]+vec[iy]+vec[iz] == 2020 {                        
                        println!("{} + {} + {} = {}", vec[ix], vec[iy], vec[iz], vec[ix]+vec[iy]+vec[iz]); 
                        println!("{} * {} * {} = {}", vec[ix], vec[iy], vec[iz], vec[ix]*vec[iy]*vec[iz]); 
                        break 'outer;
                    }
                }
            }
        }
    }
}

fn part_1() {
    // find the first set of indices that add to 2020
    let vec = read_file().expect("failed to read file");
    'outer: for ix in 0..vec.len()-1 {
        for iy in 1..vec.len() {
            if vec[ix]+vec[iy] == 2020 {
                println!("{} + {} = {}", vec[ix], vec[iy], vec[ix]+vec[iy]); 
                println!("{} * {} = {}", vec[ix], vec[iy], vec[ix]*vec[iy]); 
                break 'outer;
            }
        }
    }
}

fn read_file() -> Result<Vec<i64>, Error> {

    let path = "input.txt";

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
        let n = line   
            .trim() // trim "whitespaces"
            .parse() // call `str::parse::<i64>(&self)` method on the trimmed line, which parses integer
            .map_err(|e| Error::new(ErrorKind::InvalidData, e))?; // parse() can return error (e.g. for string "abc"), here if we got it, we convert it to `std::io::Error` type and return it as function result
        v.push(n); // push acquired integer to the vector
    }
    Ok(v) // everything is Ok, return vector
}