use std::time::{Instant};
use std::io::{Error};
use std::fs;
use std::collections::{HashSet};

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

// ok so today is a bit more spooky. Not 100% how to do this one the "right" way,
// so I'll probably have to end up doing it the dumbest way possible

// We have three commands we need to care about (might as well fucntionize them) and we need to 
// track when we get a duplicate value.

// Using a hashmap here would double our data. We don't need to keep the commands, just the line numbers.
// we do also need to keep track of the line number right meow.

fn nop(index: &mut usize) -> usize {
    return *index + 1;
}

fn acc(commands: &Vec<&str>, index: &mut usize, accumulator: &mut isize) -> usize {    
    let val: isize = (commands[*index].split(" ").collect::<Vec<&str>>())[1].parse::<isize>().unwrap();
    *accumulator += val;
    return *index + 1;
}

fn jmp(commands: &Vec<&str>, index: &mut usize) -> usize {
    // did you know that in Rust you can't subtract an isize from a usize?
    // python has made me lazy
    let jump_ix: isize = (commands[*index].split(" ").collect::<Vec<&str>>())[1].parse::<isize>().unwrap();
    match jump_ix > 0 {
        true => return *index + (jump_ix as usize),
        false => return *index - (jump_ix.abs() as usize)
    }
    
}

fn part_1(file_data: &String) {
    let splits: Vec<&str> = file_data.split("\n").collect(); // this will split each line seperately

    let mut line_record: HashSet<usize> = HashSet::new();
    let mut active_line: usize = 0;
    let mut accumulator: isize = 0;

    'boot_check: loop {
        match line_record.insert(active_line) {
            true => (),
            false => break 'boot_check
        }
        // peel off the command and run the associated function
        let this_command: &str = (splits[active_line].split(" ").collect::<Vec<&str>>())[0];
        match this_command {
            "acc" => active_line = acc(&splits, &mut active_line, &mut accumulator),
            "nop" => active_line = nop(&mut active_line),
            "jmp" => active_line = jmp(&splits, &mut active_line),
            _ => ()
        }

    }

    println!("loop found at command {}", active_line);
    println!("accumulator: {}", accumulator);

}

// ok so part 2 is uhhhhh *a lot more* than part 1
// or im just bad at this and there's a known class of problem here.
// we actually know a little more than I thought: the corrupted line MUST
// be in my line record somewhere, so we never need to worry about it going ahead

// so now: does it matter if we go in order changing commands? If I don't care about order, the hashset
// still works nicely. If I DO care about order, I need to change to something different.
// I don't think order matters. I'll look at other solutions later to see if I'm right.

fn part_2(file_data: &String) {
    let splits: Vec<&str> = file_data.split("\n").collect(); // this will split each line seperately

    let mut line_record: HashSet<usize> = HashSet::new();
    let mut active_line: usize = 0;
    let mut accumulator: isize = 0;

    'boot_check: loop {
        match line_record.insert(active_line) {
            true => (),
            false => break 'boot_check
        }
        // peel off the command and run the associated function
        let this_command: &str = (splits[active_line].split(" ").collect::<Vec<&str>>())[0];
        match this_command {
            "acc" => active_line = acc(&splits, &mut active_line, &mut accumulator),
            "nop" => active_line = nop(&mut active_line),
            "jmp" => active_line = jmp(&splits, &mut active_line),
            _ => ()
        }

    }

    // at this point we have the infinite loop.

    println!("loop found at command {}", active_line);
    println!("accumulator: {}", accumulator);

    'new_repair: for known_line in line_record {
        let repair_command: &str = (splits[known_line].split(" ").collect::<Vec<&str>>())[0];
        match repair_command {
            "acc" => (),
            "nop" | "jmp" => {
                // println!("Repairing command {} on line {}", repair_command, known_line);
                // possible bad command, test it with the change
                let mut repair_record: HashSet<usize> = HashSet::new();
                let mut active_line: usize = 0;
                let mut accumulator: isize = 0;
                'inject_check: loop {
                    // check if we end
                    if active_line > splits.len()-1 {
                        println!("Exit success!");
                        println!("accumulator: {}", accumulator);
                        return;
                    }
                    // check for an infinite boot loop
                    match repair_record.insert(active_line) {
                        true => (),
                        false => break 'inject_check
                    }
                    // peel off the command and run the associated function
                    let mut this_command: &str = (splits[active_line].split(" ").collect::<Vec<&str>>())[0];                     
                    // flip the command if it's on the injected line
                    if known_line == active_line {
                        // println!("swapping command {} on line {}", this_command, active_line);
                        if this_command == "nop" {
                            this_command = "jmp";
                        } else {
                            this_command = "nop";
                        }
                    }
                    // now do the match
                    match this_command {
                        "acc" => active_line = acc(&splits, &mut active_line, &mut accumulator),
                        "nop" => active_line = nop(&mut active_line),
                        "jmp" => active_line = jmp(&splits, &mut active_line),
                        _ => ()
                    }
                }                
                // println!("Repair failed");
            },
            _ => ()
        }
        
    }
    
}