use std::time::{Instant};
use std::io::{BufRead, BufReader, Error};
use std::fs::File;

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

// make a function that takes "movement" in the form of "direction" and "amount"
// I only need a x or y direction, and then positive or negative amounts
// We can actually assume that any movement can be represented as an x movement and then a y movement, even if the instructions are more than that

// we get "infinite" rows of . and # characters which we can just use a single vector and loop over the ends for
// we do get to start at 0,0 which is nice
// x propegates along the row, y along the column

// plan of attack:
// origin doesn't count
// read in a row, throw it in a vector
// put that row in a vector, so we have a vector of vectors
// let x = row_movement, let y = column_movement
// let all_data = vec[vec[]]
// character at that space is then all_data[][]
// then match such that . is +0 and # is +1

fn part_1() {
    let filepath = "input.txt";
    let vec = read_file(filepath).expect("failed to read file");

    let ow = traverse_slope(3, 1, &vec).expect("tobaggan crashed");
    // print answer
    println!("ow: {}", ow)
}

fn part_2() {
    let filepath = "input.txt";
    let vec = read_file(filepath).expect("failed to read file");

    // same as before, but more sledding
    // Right 1, down 1.
    // Right 3, down 1. (This is the slope you already checked.)
    // Right 5, down 1.
    // Right 7, down 1.
    // Right 1, down 2.
    let slopes = vec![(1,1), (3,1), (5,1), (7,1), (1,2)];
    let mut ow = 1;
    for ix in 0..slopes.len() {
        ow *= traverse_slope(slopes[ix].0, slopes[ix].1, &vec).expect("tobaggan crashed");
    }
    println!("ow: {}", ow)
}

fn traverse_slope(move_right: usize, move_down: usize, vec: &Vec<String>) -> Result<isize, Error>  {
    // set up coordinates
    // skip the origin
    let mut row_ix = move_down;
    let mut col_ix = move_right;
    let mut trees_hit = 0;

    // traverse our tree
    while row_ix < vec.len() {
        // check if we hit a tree
        match vec[row_ix].chars().nth(col_ix).unwrap() {
            '#' => trees_hit += 1,
            _ => ()
        }
        // move our "cursor"
        row_ix += move_down;
        // if row_ix < vec.len() {
        //     col_ix = (col_ix + move_right) % vec[row_ix].chars().count(); // [TODO] put some error handling here instead of if statement
        // }
        match vec.get(row_ix) {
            // using get here allows us to catch the panic state, which is probably more Rustic
            Some(substr) => {col_ix = (col_ix + move_right) % substr.chars().count();}
            None => ()
        }
    }

    Ok(trees_hit)
}

fn read_file(path: &str) -> Result<Vec<String>, Error> {


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