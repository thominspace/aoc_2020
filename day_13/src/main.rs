// #[macro_use]
// extern crate rulinalg;

use std::time::{Instant};
use std::io::{Error};
use std::fs;

extern crate num;

// use rulinalg::matrix::Matrix;
// use rulinalg::vector::Vector;

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

    // first line is target, second line needs to be split out futher

    let target = splits[0].parse::<isize>().unwrap();
    let schedule: Vec<&str> = splits[1].split(",").collect();

    let mut earliest_bus = isize::MAX; // largest number I can put in an isize
    let mut bus_id = 0;


    for this_bus in schedule { // go through the split schedule
        match this_bus { 
            "x" => (), // skip if it's an x
            _ => { // assume all other options work
                let this_num = this_bus.parse::<isize>().unwrap(); // make it an int
                // println!("bus_id: {}, mod: {}", this_num, this_num - (target % this_num));

                if this_num - (target % this_num) < earliest_bus { // check the int
                    earliest_bus = this_num - (target % this_num);
                    bus_id = this_num;
                }
            } 
        }
    }
    println!("earliest_bus: {}, bus_id: {}, ans: {}", earliest_bus, bus_id, earliest_bus*bus_id)
}

fn part_2(file_data: &String) {
    let splits: Vec<&str> = file_data.split("\n").collect(); // this will split each line seperately


    // first line no longer matters, second line needs to be split out futher AND we need to care about index
    let schedule: Vec<&str> = splits[1].split(",").collect();

    // we want to find where each (bus_id - offset) % bus_id = timestamp, for all bus_id
    // just a set of linear equations(?), but im not certain how I feel about throwing modulo at LAPAC

    // notably, the bus_ids are not in order. working from largest to smallest is probably most efficient ehre
    // there's probably a oneliner math solution and im alerady mad Im not going to find it

    // pull the data out and put it in a vector, then sort.
    // first val is ID; second is place in array
    let mut bus_schedule: Vec<(f64, f64)> = Vec::new();

    for (ix, &this_bus) in schedule.iter().enumerate() { // go through the split schedule
        match this_bus { 
            "x" => (), // skip if it's an x
            _ => {
                    bus_schedule.push((this_bus.parse::<f64>().unwrap(), ix as f64));
            }
        } 
    }

    // sort over tuples sorts by the first value, which is conventient    
    bus_schedule.sort_by(|a, b| b.partial_cmp(a).unwrap()); // this sort just sorts in reverse
    println!("sched: {:?}", bus_schedule);

    let mut last_good = 0;
    let mut first_check = true;
    let mut check_bus = 1;

    println!("Trying the slow way");
    let mut solution_found;
    let mut check_val;
    let mut n_multi = 1;
    let mut this_iter = 0;
    let mut n_0: i64 = 0;
    'search_loop: loop {
        solution_found = true;
        let t = (n_0 as f64)*bus_schedule[0].0 - bus_schedule[0].1;
        // println!("foo: {:?}", t);
        'check_loop: for bus_i in 1..bus_schedule.len() { // check agains the next val
            check_val = ((t+bus_schedule[bus_i].1)/bus_schedule[bus_i].0).fract();
            
            // if there isn't a perfect divisor, move on
            if check_val.fract() != 0.0 { 
                solution_found = false;
                break 'check_loop
            }
            if bus_i >= check_bus {
                
                println!("bus_i: {:?}, {}, last good diff {} iter {}", bus_i, n_0, n_0-last_good, this_iter);

                // if this is the first time we've found this lock, skip it and find the next one
                if first_check {
                    first_check = false;
                } else {
                    first_check = true;
                    check_bus += 1;
                    
                    // println!("Foo!");
                    n_multi = n_0-last_good;
                    // if n_0-last_good > 0 { n_multi = n_0-last_good}
                }
                last_good = n_0;
            }
        }
        if solution_found { 
            println!("Solution found!");
            println!("t: {}",  t);
            break 'search_loop;
        }
        if this_iter > 20000 {break 'search_loop}
        n_0 += n_multi;
        this_iter += 1;
    }
}