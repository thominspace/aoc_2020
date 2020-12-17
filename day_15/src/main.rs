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
    let splits: Vec<&str> = file_data.split(",").collect(); // this will split each line seperately

    let target_turn: usize = 2020;

    let mut spoken: HashMap<usize, usize> = HashMap::new();
    for (ix, line) in splits.iter().enumerate() {
        // insert the first numbers
        spoken.insert(line.parse::<usize>().unwrap(), ix+1);
        // println!("{}, {}", ix+1, line);
    }

    // assume the first number outside the starters is always a 0, since it's the first time we've seen the number
    let mut next_spoken: usize = 0;

    for ix in splits.len()+1..target_turn+1 {
        // spoken.insert(0)        
        if ix == target_turn {println!("{}, {}", ix , next_spoken);}
        // println!("{:?}", spoken);

        if spoken.contains_key(&next_spoken) {
            // println!("number already in list, insert distance");
            // if it contains this next number, the next number will be the difference between now and the last time the number was said
            let gap: usize = ix - *spoken.get(&next_spoken).unwrap();
            spoken.insert(next_spoken, ix);
            next_spoken = gap;
        } else {
            // println!("number is new, insert 0");
            spoken.insert(next_spoken, ix);
            next_spoken = 0;
        }
    }
}

fn part_2(file_data: &String) {
    let splits: Vec<&str> = file_data.split(",").collect(); // this will split each line seperately

    // let target_turn: usize = 2020;
    let target_turn: usize = 30000000;

    let mut spoken: HashMap<usize, usize> = HashMap::new();
    for (ix, line) in splits.iter().enumerate() {
        // insert the first numbers
        spoken.insert(line.parse::<usize>().unwrap(), ix+1);
        // println!("{}, {}", ix+1, line);
    }

    // assume the first number outside the starters is always a 0, since it's the first time we've seen the number
    let mut next_spoken: usize = 0;

    for ix in splits.len()+1..target_turn+1 {
        // spoken.insert(0)        
        if ix == target_turn {println!("{}, {}", ix , next_spoken);}
        // println!("{:?}", spoken);

        if spoken.contains_key(&next_spoken) {
            // println!("number already in list, insert distance");
            // if it contains this next number, the next number will be the difference between now and the last time the number was said
            let gap: usize = ix - *spoken.get(&next_spoken).unwrap();
            spoken.insert(next_spoken, ix);
            next_spoken = gap;
        } else {
            // println!("number is new, insert 0");
            spoken.insert(next_spoken, ix);
            next_spoken = 0;
        }
    }
}
