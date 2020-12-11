use std::time::{Instant};
use std::io::{Error};
use std::fs;
use std::collections::{HashMap};

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
    // split each line seperately into a usize
    let mut adapters: Vec<usize> = file_data.split("\n").map(|x| x.parse::<usize>().unwrap()).collect(); 

    // sort it
    adapters.sort();
    println!("{:?}", adapters[0]);

    // find our mins and maxes and record via array (no need for hashmap,.. maybe part 2)
    let mut differences: [usize; 3] = [0; 3]; 

    // account for the first adapter
    differences[adapters[0]-1] += 1;

    for ix in 0..adapters.len()-1 {
        // forward differences
        differences[adapters[ix+1]-adapters[ix]-1] += 1;
    }

    // account for the last adapter
    differences[2] += 1;

    println!("{:?}, {:?}", differences[0], differences[2]);
    println!("{:?}", differences[0] * differences[2]);
}

fn part_2(file_data: &String) {
    // after a bunch of pencil whippin, i found something that should work.

    // starting at the beginning, keep track of which number has been "solved", in order
    // for each number (let's call it the root number), find all forward numbers that are within 3
    // walk through each number we found ahead, and add the "solved" values behind it including root number to "solve" that number

    // example: 0 1 4 5 6 7 10 11 12 15 16 19 22
    // formatting for the example below:
    // if the number is considered "solved" have the number in ()
    // solved numbers start with 1
    // <root> | <forward> ... <forward>

    // 0(1)   | 1    <--- sum of all numbers behind this is 1.
    // 1(1)   | 4    <--- sum of all numbers behind this is 1.
    // 4(1)   | 5, 6, 7     <--- at this point we hit 5, which "solves" to 1.
    // 4(1)   | 5(1), 6(2), 7     <--- now we see 6, which solves across to (1+1)=(2).
    // 4(1)   | 5(1), 6(2), 7(4)     <--- now we see 7, which solves across to (2+1+1)=(4).
    // 5(1)   | 6(2), 7(4)     <--- now we see 6, which is already solved.
    // 5(1)   | 6(2), 7(4)     <--- now we see 7, which is already solved.
    // 6(2)   | 7(4)     <--- now we see 7, which is already solved.
    // 7(4)   | 10    <--- 10 solves across to 4.
    // 10(4)  | 11, 12   <--- 11 solves across to 4.
    // 10(4)  | 11(4), 12   <--- 12 solves across to (4+4)=8.
    // 11(4)  | 12(8)   <--- 12 is already solved.
    // 12(8)  | 15(8)   <--- 12 is already solved.
    // ...
    // From here on each connection only has one option, but the process is the same.

    // it's worth noting that we can't really just watch the total and increase it, 
    // because we need to know the previous totals as well (up to 3 pervious in this case)
    // I could make a deque and set that up all nice, but instead I'll just track it as a 
    // second element in the adapters vector and pair them off
    // yes, I could use a hashmap. no, i'm not going to.
    // ... I guess Im making a struct beause SOMEBODY is all "make it a named tuple"

    // split each line seperately into a usize
    let mut adapter_jolts: Vec<usize> = file_data.split("\n").map(|x| x.parse::<usize>().unwrap()).collect(); 
    let mut adapter_branches: Vec<usize> = file_data.split("\n").map(|x| 0).collect(); 

    // add the front and end
    adapter_jolts.push(0);
    adapter_jolts.push(adapter_jolts.iter().max().unwrap() +3);
    
    // give the branch tracker a few more as well
    adapter_branches.push(0);
    adapter_branches.push(0);

    // sort it
    adapter_jolts.sort(); // I'm surprised that sorted correctly. thats super neat
    // println!("{:?}", adapters);

    // initialize
    adapter_branches[0] = 1;

    // now we start the algorithm
    for (ix, this_jolts) in adapter_jolts.iter().enumerate() {
        // find all values ahead that are applicable, max 3
        let mut row_val_ix: Vec<usize> = vec![ix]; // keep track of the "row" numbers (see comments above)
        for ahead_ix in ix+1..ix+4 { // look ahead 3
            if ahead_ix < adapter_jolts.len() { // don't go out of bounds
                // println!("row ix: {:?}", row_val_ix);
                // println!("jolts: {:?}", &adapter_jolts[ix..ahead_ix-1]);
                let adapter_jolts_ahead = adapter_jolts[ahead_ix];
                if adapter_jolts_ahead - this_jolts <= 3 { // we found a valid adapter, check if we've solved it
                    if adapter_branches[ahead_ix] == 0 { // if the branch is 0 it's considered "unsolved"
                        let mut running_sum: usize = 0; // keep track of the row sum
                        for this_ix in &row_val_ix { // I dont know how to index a vector with another vector of indices
                            running_sum += adapter_branches[*this_ix];
                        }
                        // println!("-------- running sum {:?}", running_sum);
                        adapter_branches[ahead_ix] = running_sum;
                    }
                    row_val_ix.push(ahead_ix);
                }
            }

        }
    }
    println!("{:?} distinct arrangements", adapter_branches.last().unwrap())
}