use std::time::{Instant};
use std::io::{Error};
use std::fs;
use std::num::{ParseIntError};
// use std::error::Error;
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
                // println!("passport {:?}", passport);
                if check_passport_keys(&required_keys, &passport) == true {
                    good_passports += 1;
                };
                passport = HashMap::new(); // reset passport
            },
            _ => { 
                let key_value_pair: Vec<&str> = substr.split(":").collect();
                match key_value_pair[0] { // now insert the key data with the relevant
                    "byr" | "iyr" | "eyr" | "cid" => {
                        let field_int = key_value_pair[1].parse();
                        match field_int {
                            Ok(value) => passport.insert(key_value_pair[0], PassportField::IntType(value)),
                            Err(..) => None //panic!("shit {}", error)
                        };
                        // let field_str = PassportField::StringType(String::from(key_value_pair[1]));                        
                    },
                    "pid" | "hgt" | "hcl" | "ecl" => {
                        let field_str = String::from(key_value_pair[1]);
                        passport.insert(key_value_pair[0], PassportField::StringType(field_str));
                    },
                    _ => ()
                };
            },
        }
    }
    // check the last one
    // println!("------- CHECKING PASSPORT -------"); // if we find a blank string, it's a blank line, so new passport
    if check_passport_keys(&required_keys, &passport) == true {
        good_passports += 1;
    };
    passport = HashMap::new(); // reset passport
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
    // based on my last version, I should now assume that we have the following "types" of passport fields
    // IntType(isize) --- byr, iyr, eyr, pid, cid
    // StringType(String) --- ecl
    // ColorCodeType(String) --- hcl
    // HeightType(isize, String) --- hgt

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
                // println!("passport {:?}", passport);
                if validate_passport(&required_keys, &passport) == true {
                    good_passports += 1;
                };
                passport = HashMap::new(); // reset passport
            },
            _ => { 
                let key_value_pair: Vec<&str> = substr.split(":").collect();
                match key_value_pair[0] { // now insert the key data
                    "byr" | "iyr" | "eyr"  => {
                        let field_int = key_value_pair[1].parse();
                        match field_int {
                            Ok(value) => passport.insert(key_value_pair[0], PassportField::IntType(value)),
                            Err(..) => None
                        };
                    },
                    "hgt" => {
                        let field_height_num: String = key_value_pair[1].trim_matches(char::is_alphabetic).to_string();
                        let field_height_num = field_height_num.parse();
                        let field_height_unit: String = key_value_pair[1].trim_matches(char::is_numeric).to_string();
                        match (field_height_num, field_height_unit) {
                            (Ok(num), unit) => {                                
                                // println!("height {} {}", num, unit);
                                passport.insert(key_value_pair[0], PassportField::HeightType(num, unit))
                            },
                            _ => None 
                        };
                    },
                    "hcl" => {
                        let field_str = String::from(key_value_pair[1]);
                        passport.insert(key_value_pair[0], PassportField::ColorCodeType(field_str));
                    },
                    "ecl" | "cid" | "pid" => {
                        let field_str = String::from(key_value_pair[1]);
                        passport.insert(key_value_pair[0], PassportField::StringType(field_str));
                    },
                    _ => ()
                };
            },
        }
    }
    // check the last one
    // println!("------- CHECKING PASSPORT -------"); // if we find a blank string, it's a blank line, so new passport
    // println!("passport {:?}", passport);
    if validate_passport(&required_keys, &passport) == true {
        good_passports += 1;
    };
    passport.clear(); // reset passport
    // print the answer
    println!("found {} valid passports", good_passports)
}

fn validate_passport(keys_to_check: &[&str], hashmap_to_check: &HashMap<&str, PassportField>) -> bool {
    // check if we have all the required keys
    for this_key in keys_to_check {        
        if hashmap_to_check.contains_key(this_key) == false {
            // println!("Failed to find key {} in passport!", this_key);
            return false;
        }
    }
    // now check individual keys for valid data
    // Our input validation is now more strict
    // byr (Birth Year) - four digits; at least 1920 and at most 2002.
    // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
    // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
    // hgt (Height) - a number followed by either cm or in:
    // If cm, the number must be at least 150 and at most 193.
    // If in, the number must be at least 59 and at most 76.
    // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
    // ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
    // pid (Passport ID) - a nine-digit number, including leading zeroes.
    // !!!! pid is ACTUALLY a string, we need to count leading zeros. We can try to convert to number in validation
    // cid (Country ID) - ignored, missing or not.

    for this_key in hashmap_to_check.keys() {
        match this_key {
            &"byr" => {
                if let PassportField::IntType(val) = &hashmap_to_check[this_key] {
                    // println!("byr: {:?}", &hashmap_to_check[this_key]);
                    // println!("byr: {:?}", val);
                    if (val < &1920) || (val > &2002) {
                        // println!("passport: {:?}", hashmap_to_check);
                        // println!("byr failed with {:?}", val);
                        return false
                    };
                } else { return false };
            },
            &"iyr" => {
                if let PassportField::IntType(val) = &hashmap_to_check[this_key] {
                    // println!("iyr: {:?}", &hashmap_to_check[this_key]);
                    // println!("iyr: {:?}", val);
                    if (val < &2010) || (val > &2020) {
                        // println!("passport: {:?}", hashmap_to_check);
                        // println!("iyr failed with {:?}", val);
                        return false
                    };
                } else { return false };
            },
            &"eyr" => {
                if let PassportField::IntType(val) = &hashmap_to_check[this_key] {
                    // println!("eyr: {:?}", &hashmap_to_check[this_key]);
                    // println!("eyr: {:?}", val);
                    if (val < &2020) || (val > &2030) {
                        // println!("passport: {:?}", hashmap_to_check);
                        // println!("eyr failed with {:?}", val);
                        return false
                    };
                } else { return false };
            },
            &"pid" => {
                if let PassportField::StringType(val) = &hashmap_to_check[this_key] {
                    // println!("pid: {:?}", &hashmap_to_check[this_key]);
                    // println!("pid: {:?}", val);
                    // if parse worked, check the make sure it's exactly 9 digits
                    if val.chars().count() != 9 {
                        // println!("passport: {:?}", hashmap_to_check);
                        // println!("pid failed with {:?}", val);
                        return false
                    };
                    let pid_as_int = val.parse::<isize>();
                    // println!("PID ERRS {:?}", pid_as_int);
                    match pid_as_int { // check if parse failed
                        Ok(..) => (),
                        Err(..) => return false 
                    };
                } else { return false };
            },
            &"cid" => (), // we're just going to happily ignore this one
            &"hgt" => {
                if let PassportField::HeightType(val, unit) = &hashmap_to_check[this_key] {
                    match unit.as_str() {
                        "cm" => {
                            if (val < &150) || (val > &193) {
                                // println!("passport: {:?}", hashmap_to_check);
                                // println!("hgt failed with {:?}, {:?}", val, unit);
                                return false
                            };
                        },
                        "in" => {
                            if (val < &59) || (val > &76) {
                                // println!("passport: {:?}", hashmap_to_check);
                                // println!("hgt failed with {:?}, {:?}", val, unit);
                                return false
                            };
                        },
                        _ => return false
                    }
                } else { return false };
            },
            &"hcl" => {
                if let PassportField::ColorCodeType(val) = &hashmap_to_check[this_key] {
                    // check the first character
                    if val.chars().nth(0).unwrap() != '#' {
                        // println!("passport: {:?}", hashmap_to_check);
                        // println!("hcl failed with {:?}", val);
                        return false
                    }; // this should be safe. I can unwrap it.
                    let pid_as_int = val[1..].to_string();
                    match pid_as_int.len() { // check if parse failed
                        6 => {
                            // println!("hcl: {:?}", &hashmap_to_check[this_key]);
                            // println!("hcl: {:?}", pid_as_int.chars().all(|c| c.is_numeric() || ['a', 'b', 'c', 'd', 'e', 'f'].contains(&c)));
                            if pid_as_int.chars().all(|c| c.is_numeric() || ['a', 'b', 'c', 'd', 'e', 'f'].contains(&c)) == false {
                                // println!("hcl failed with {:?} (enough characters, not all alphanumeric)", val);
                                return false
                            }
                        },
                        _ => {
                            // println!("passport: {:?}", hashmap_to_check);
                            // println!("hcl failed with {:?}", val);
                            return false
                        },
                    }
                } else { return false };
            }
            &"ecl" => {
                if let PassportField::StringType(val) = &hashmap_to_check[this_key] {
                    // println!("ecl: {:?}", val);
                    match &val[..] {
                        "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => (),
                        _ => {
                            // println!("passport: {:?}", hashmap_to_check);
                            // println!("ecl failed with {:?}", val);
                            return false
                        },                    
                    }
                } else { return false };
            }
            _ => ()
        }
    }

    return true
}
