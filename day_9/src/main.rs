use std::time::{Instant};
use std::io::{BufRead, BufReader, Error, ErrorKind, Read};
use std::collections::VecDeque;
use std::fs;

fn main() -> Result<(), Error> {
    // seeing as pretty much every puzzle is going to be reading in a file and then manipulating the input, 
    // I might as well just build main out to be a template

    // read the input for today's puzzle
    let filepath = "input.txt";
    // going to parse this one slightly differently, just because I want everything as a usize and I dont care about the entire file at the same time.
    // also a good way to practice some vector manipulation with pop and push.
    // let file_data = fs::read_to_string(filepath).expect("failed to read file"); // returns a vector of strings split by line
    
    let buffer = BufReader::new(fs::File::open(filepath)?);

    // part 1
    let start = Instant::now();
    let part_1_ans = part_1(buffer).unwrap();
    let duration = start.elapsed();
    println!("Time elapsed in part 1: {:?}", duration);
    
    // part 2
    // refreshing the buffer
    let buffer = BufReader::new(fs::File::open(filepath)?);
    let start = Instant::now();
    part_2(buffer, &part_1_ans);
    let duration = start.elapsed();
    println!("Time elapsed in part 2: {:?}", duration);

    Ok(())
}



fn part_1<R: BufRead>(buffer: R) -> Option<usize> {

    let preamble = 25;
    let mut buffer_iterator = buffer.lines();
    let mut numbers: VecDeque<usize> = VecDeque::new();

    // read the preamble into the vector
    for ix in 0..preamble {
        match buffer_iterator.next() {
            Some(Ok(num_as_string)) => numbers.push_back(num_as_string.parse::<usize>().unwrap()), // convert to usize
            Some(Err(_)) => (), // well this is spooky, but I'm not going to worry about that now
            None => ()
        }
    }

    // cycle through the vector enforcing our rules
    'file_iter_loop: for line in buffer_iterator {
        match line {
            Ok(num_as_string) => {
                numbers.push_back(num_as_string.parse::<usize>().unwrap()); // push the next number
                let mut number_passes_rules = false;
                'outer_ix_loop: for ix in 0..preamble-1 { // burte force comparing the math
                    'inner_ix_loop: for iy in ix+1..preamble {
                        if numbers[ix] + numbers[iy] == *numbers.back().unwrap() {
                            number_passes_rules = true;
                            break 'outer_ix_loop;
                        }

                    }
                }
                if number_passes_rules { // found a valid number, move on and drain the first element
                    numbers.pop_front();
                }
                else { // found a rule break, we're done here.
                    println!("{:?} does not match the rules", *numbers.back().unwrap());
                    return Some(*numbers.back().unwrap());
                }
            },
            Err(_) => () // still spooky, but im getting closer to error handling! go me.
        }
    }
    return None;
}

// we can be somewhat clever (memory wise) on part 2
// since the numbers we add together have to be contiguous, we can start adding numbers until we
// go PAST our target. If we go past, we know the first number isn't part of the sequence that we want
// After removing the first number, we will still need to restart the search from the (new) first
// number and *just the next* number, even though there could be more values in the vector
// After that, we can continue (as long as we don't get a sum larger than the target) as normal, and only .push()
// as needed

fn part_2<R: BufRead>(buffer: R, target: &usize) {
    
    let mut buffer_iterator = buffer.lines();
    let mut numbers: VecDeque<usize> = VecDeque::new();
    let mut search_len: usize = 2;
    // get the first 2 values in (minimum 2 values needed for this case)
    for _ in 0..2 { // coding this way to set up the possibility of a different minimum case
        match buffer_iterator.next() { // I really don't know how to get this as a one-line
            Some(Ok(num_as_string)) => numbers.push_back(num_as_string.parse::<usize>().unwrap()), // convert to usize
            Some(Err(_)) => (), // well this is spooky, but I'm not going to worry about that now
            None => ()
        }
    }
    println!("{:?}", numbers);

    // now start adding things to the vector and processing
    'search_loop: for line in buffer_iterator {
        match line { // we have a nwe line available
            Ok(num_as_string) => {
                // now loop over the data we already have
                while numbers.len() >= search_len {
                    // println!("{:?}", numbers);
                    // check if the list of numbers we have works out
                    let mut seq_sum: usize = 0;            
                    for ix in 0..search_len { // sum it
                        seq_sum += numbers[ix];
                    }
                    if &seq_sum == target {
                        println!("Found exploit for {}: {}", target, numbers.iter().min().unwrap() + numbers.iter().max().unwrap());
                        break 'search_loop;
                    } else if &seq_sum > target {
                        numbers.pop_front();
                        search_len = 2;
                    } else if &seq_sum < target {
                        search_len += 1;
                    }
                }
                // we need more numbers!                
                if numbers.len() < search_len {
                    numbers.push_back(num_as_string.parse::<usize>().unwrap()); // push the next number
                }
            },
            Err(_) => () // still spooky, but im getting closer to error handling! go me.
        }
    }
}