use std::time::{Instant};
use std::io::{Error};
use std::fs;
use std::collections::HashMap;

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

// passport checking! How,... close to home
// we need a struct-like on this one (at the very least)
// Using Option here lets us have Some or None

#[derive(Debug)]
enum PassportField {
    IntType(isize),
    StringType(String),
    ColorCodeType(String),
    HeightType(isize, String)
}


fn part_1(file_data: &String) {
    // our data contains passports
    // splitting on spaces and newlines is fine
    // splitting on blank lines means a new passport

    // finding a blank line will be most of the difficulty here. 
    // let's see what happens when we split on newlines and print

    // let mut iter = file_data.split_ascii_whitespace(); // does not work! it treats the double newline as single whitespace so we can't split here
    let iter = file_data.split(|c| c == ' ' || c == '\n'); // this will split each newline seperately

    // store valid data in a has map
    let mut passport = HashMap::new();
    let required_keys = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let mut good_passports = 0;

    // go through the data
    for substr in iter {
        // note, I think this should have some of that fancy "?" error handling in here somewhere
        match substr {
            "" => { 
                // println!("------- CHECKING PASSPORT -------"); // if we find a blank string, it's a blank line, so new passport
                if check_passport_keys(&required_keys, &passport) == true {
                    good_passports += 1;
                };
                passport.clear(); // reset passport
            },
            _ => { 
                let key_value_pair: Vec<&str> = substr.split(":").collect();
                match key_value_pair[0] { // now insert the key data with the relevant
                    "byr" | "iyr" | "eyr" | "pid" | "cid" => {
                        // let field_int = PassportField::IntType(key_value_pair[1].parse().unwrap());
                        let field_str = PassportField::StringType(String::from(key_value_pair[1]));
                        passport.insert(key_value_pair[0], field_str)
                    },
                    "hgt" | "hcl" | "ecl" => {
                        let field_str = PassportField::StringType(String::from(key_value_pair[1]));
                        passport.insert(key_value_pair[0], field_str)
                    },
                    _ => None
                };
            },
        }
    }
    // check the last one
    // println!("------- CHECKING PASSPORT -------"); // if we find a blank string, it's a blank line, so new passport
    if check_passport_keys(&required_keys, &passport) == true {
        good_passports += 1;
    };
    passport.clear(); // reset passport
    // print the answer
    println!("found {} valid passports", good_passports)
}

fn check_passport_keys(keys_to_check: &[&str], hashmap_to_check: &HashMap<&str, PassportField>) -> bool {
    for key_check in keys_to_check {        
        if hashmap_to_check.contains_key(key_check) == false {
            // println!("Failed to find key {} in passport!", key_check);
            return false;
        }
    }
    true
}

fn part_2(file_data: &String) {

}


