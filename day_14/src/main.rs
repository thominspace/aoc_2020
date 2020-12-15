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
    let splits: Vec<&str> = file_data.split("\n").collect(); // this will split each line seperately

    let mut memory: HashMap<&str, u64> = HashMap::new(); // keeping as a str because im lazy, and i expect this to bite me on part 2
    let mut mask: Vec<(usize, char)> = Vec::new();

    for line in splits {

        // store the split
        let this_line: Vec<&str> = line.split(" ").collect();

        // handle a new mask
        if line.contains("mask") {
            // the third line in the split will give us the mask
            mask.clear(); // clear it out if we had anything
            // read the string and save just the mask information into a vector
            for (ix, this_char) in this_line[2].chars().enumerate() {
                if this_char != 'X' {
                    mask.push((ix, this_char));
                }
            }

        }
        // handle a memory line
        if line.contains("mem") {

            // read the string in as an unsigned int
            let mut digit_as_u64 = this_line[2].parse::<u64>().unwrap(); 
            // convert to binary (as a string which feels silly)
            let mut digit_as_bin = format!("{:036b}", digit_as_u64);
            // println!("{:?}", digit_as_bin);

            unsafe { // spooky
                let digit_as_bin_vec = digit_as_bin.as_bytes_mut();

                // now process the mask
                for mask_val in &mask {
                    // in rust you can't index strings, so you have to use an iterator. which is not fun
                    digit_as_bin_vec[mask_val.0] = mask_val.1 as u8;
                }
            };
            // read back as a u64
            digit_as_u64 = u64::from_str_radix(&digit_as_bin, 2).unwrap();

            // now determine memory location
            let mem_target = this_line[0].strip_prefix("mem[").unwrap().strip_suffix("]").unwrap();
            memory.insert(mem_target, digit_as_u64);
        }

    }

    // now count
    let mut sum = 0;
    for (_, val) in memory {
        sum += val;
    }

    println!("ans: {:?}", sum);
}

fn part_2(file_data: &String) {

    // ok this one is weird
    let splits: Vec<&str> = file_data.split("\n").collect(); // this will split each line seperately

    let mut memory: HashMap<u64, u64> = HashMap::new(); // keeping as a str because im lazy, and i expect this to bite me on part 2
    let mut mask: Vec<(usize, char)> = Vec::new();

    for line in splits {

        // store the split
        let this_line: Vec<&str> = line.split(" ").collect();

        // handle a new mask
        if line.contains("mask") {
            // the third line in the split will give us the mask
            mask.clear(); // clear it out if we had anything
            // read the string and save just the mask information into a vector
            for (ix, this_char) in this_line[2].chars().enumerate() {
                if this_char != '0' { // in this case, 0 means the value is unchanges
                    mask.push((ix, this_char));
                }
            }

        }
        // handle a memory line
        if line.contains("mem") {

            // read the string in as an unsigned int
            let mut digit_as_u64 = this_line[2].parse::<u64>().unwrap(); 
            // convert to binary (as a string which feels silly)
            let digit_as_bin = format!("{:036b}", digit_as_u64);
            // read back as a u64
            digit_as_u64 = u64::from_str_radix(&digit_as_bin, 2).unwrap();
            
            // now determine memory location
            let mem_target = this_line[0].strip_prefix("mem[").unwrap().strip_suffix("]").unwrap();
            let mem_target_as_u64 = mem_target.parse::<u64>().unwrap();             
            let mem_target_string = format!("{:036b}", mem_target_as_u64);
            // look for that first X
            mask_combination(&0, &mut mask, &digit_as_u64, &mem_target_string, &mut memory);

        }
    }
    // now count
    let mut sum: u64 = 0;
    // println!("ans: {:?}", memory);
    for (_, val) in memory {
        sum += val as u64;
    }
    println!("ans: {:?}", sum);
}

fn mask_combination(mask_ix: &usize, mask: &mut Vec<(usize, char)>, digit_as_u64: &u64, mem_target: &String, memory: &mut HashMap<u64, u64>) {


    let mut this_mask_ix = *mask_ix;
    // look for the next X
    'x_gon_give_it_to_ya: while this_mask_ix < mask.len()  {
        if mask[this_mask_ix].1 == 'X' {
            break 'x_gon_give_it_to_ya;
        }
        else {
            this_mask_ix += 1;
        }
    }

    if this_mask_ix < mask.len() {
        // we found an X at this_mask_ix    
        // change it to a 1
        mask[this_mask_ix].1 = '1';
        mask_combination(&this_mask_ix, mask, digit_as_u64, &mem_target, memory);
    
        // change it to a 0
        mask[this_mask_ix].1 = '0';
        mask_combination(&this_mask_ix, mask, digit_as_u64, &mem_target, memory);
    
        // reset
        mask[this_mask_ix].1 = 'X';
        // println!("{:?}", mask);
    }

    if this_mask_ix == mask.len() {
        // println!("{:?}", mask); // read back as a u64
        // now process the mask
        // If the bitmask bit is 0, the corresponding memory address bit is unchanged.
        // If the bitmask bit is 1, the corresponding memory address bit is overwritten with 1.
        // If the bitmask bit is X, the corresponding memory address bit is floating.
        let mut this_mem_target: Vec<char> = mem_target.chars().collect::<Vec<_>>();
        for mask_val in mask.clone() {
            // in rust you can't index strings, so you have to use an iterator. which is not fun
            match mask_val {
                (ix, overwrite_char) => {
                    this_mem_target[ix] = overwrite_char;
                }
            }
        }           
            
        // insert at the new memory location
        // now back to a u64
        let this_mem_target_str: String = this_mem_target.into_iter().collect();
        let this_mem_target_u64: u64 = u64::from_str_radix(&this_mem_target_str, 2).unwrap();
        insert_to_mem(memory, &this_mem_target_u64, &digit_as_u64);
    }


}

fn insert_to_mem(memory: &mut HashMap<u64, u64>, this_mem_target_u64: &u64, digit_as_u64: &u64) {
    memory.insert(*this_mem_target_u64, *digit_as_u64);

}