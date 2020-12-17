// #[macro_use] extern crate lazy_static;
extern crate regex;

use std::time::{Instant};
use std::io::{Error};
use std::fs;
use std::collections::{HashMap, HashSet};
use regex::Regex;


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

fn part_2(file_data: &String) {

    // a ton of this puzzle is file parsing, so lets get cracking
    let splits: Vec<&str> = file_data.split("\n").collect(); // this will split each line seperately

    let mut my_ticket: Vec<usize> = Vec::new();
    let mut nearby_tickets: Vec<Vec<usize>> = Vec::new();
    let mut fields_master: HashMap<String, usize> = HashMap::new();
    let mut fields_elimination: HashMap<String, HashSet<usize>> = HashMap::new();
    let mut rules: HashMap<String, Vec<(usize, usize)>> = HashMap::new();

    let mut file_line_iter = splits.iter();

    // set up regex
    let re = Regex::new(r"(.*): (.*)-(.*) or (.*)-(.*)").unwrap();

    while let Some(mut line) = file_line_iter.next() {
        // the file has three categories, field info, my ticket, and nearby tickets
        // all of the rules follow a <this> or <that> range, and none of them overlap
        
        let capture_option = re.captures(line);
        match capture_option {
            Some(capture) => {                
                fields_elimination.insert(capture[1].to_string(), HashSet::new());
                // another example of "i should probably just learn regex"
                // println!("parsing constraint");
                println!("{:?}", capture);
                rules.insert(capture[1].to_string(), vec![(capture[2].parse::<usize>().unwrap(), capture[3].parse::<usize>().unwrap()),
                                                          (capture[4].parse::<usize>().unwrap(), capture[5].parse::<usize>().unwrap())]);
            },
            None => ()
        }

        if *line == "your ticket:" {
            // println!("parsing my ticket");
            // get the next line
            line = file_line_iter.next().unwrap();
            // split on commas
            let words: Vec<&str> = line.split(",").collect();
            // println!("{:?}", words);
            for num in words {
                my_ticket.push(num.parse::<usize>().unwrap());
            }
        }

        if *line == "nearby tickets:" {
            // println!("parsing nearby tickets");
            // assume all other lines here are nearby tickets
            // get the next line
            'nearby_ticket_insert_loop: while let Some(line) = file_line_iter.next() {
                let mut val_is_valid = false;
                let mut temp_vec: Vec<usize> = Vec::new();
                // split on commas
                let words: Vec<&str> = line.split(",").collect();
                // println!("{:?}", words);
                for num in words {
                    let this_num = num.parse::<usize>().unwrap();
                    // check this number against all the  rules
                    'check_loop: for (_, rule_bounds) in &rules {
                        for (lower_limit, upper_limit) in rule_bounds {
                            if (this_num >= *lower_limit) && (this_num <= *upper_limit) {
                                // value is valid in some rule somewhere, we're good to proceed
                                val_is_valid = true;
                                break 'check_loop;
                            }
                        }
                    }
                    if val_is_valid == true {
                        temp_vec.push(this_num);
                    } else {
                        break 'nearby_ticket_insert_loop;
                    }
                }
                // we haven't broken out of the check loop at this point, so go ahead and push the vec
                nearby_tickets.push(temp_vec.clone());
            }            
        }
    }

    println!("rules {:?}", rules);
    println!("my ticket {:?}", my_ticket);
    println!("nearby tickets {:?}", nearby_tickets.len());
    println!("fields_elimination {:?}", fields_elimination);


    // // check now for bad rows, and just remove them
    // for (ix, this_ticket) in nearby_tickets.iter().enumerate() {
    //     for val in this_ticket {
    //         let mut val_is_valid = false;
    //         'check_loop: for (_, rule_bounds) in &rules {
    //             for (lower_limit, upper_limit) in rule_bounds {
    //                 if (val >= *lower_limit) && (val <= *upper_limit) {
    //                     // value is valid, we're good to proceed
    //                     val_is_valid = true;
    //                     break 'check_loop;
    //                 }
    //             }
    //         }
    //         // we've determined that the value isnt in any of the ranges
    //         // add that number to the error rate
    //         if val_is_valid == false {
    //             err_sum += val;
    //         }
    //     }        
    // }

    // I should parse the rules and find the min, max, and all values that are not within the rules, but
    // I have a feeling part 2 will bite me on that. Instead (and this is slower probably) im going to
    // just "enforce" the rules across the board

    let mut field_indices_to_solve: HashSet<usize> = (0..fields_elimination.len()).collect();

    // check each ticket
    for this_ticket in nearby_tickets {
        // check each number in the nearby ticket
        for (ticket_field_ix, val) in this_ticket.iter().enumerate() {
            // check this number against each rule, finding valid rules
            for (rule_field_name, rule_bounds) in &rules {
                let mut val_is_valid = false;
                // check the number against the individual bound ranges
                'second_check_loop: for (lower_limit, upper_limit) in rule_bounds {
                    if (val >= lower_limit) && (val <= upper_limit) {
                        // value is valid, we're good to proceed
                        val_is_valid = true;
                        break 'second_check_loop;
                    }
                }
                // we've determined that the value isnt in any of the ranges, which means that this
                // index can not contain that field
                if val_is_valid == false {                    
                    println!("slot {} can not be {}", ticket_field_ix, rule_field_name);
                    let field_options = fields_elimination.get_mut(rule_field_name).unwrap();
                    field_options.insert(ticket_field_ix);
                }
            }
        }        
    }

    println!("fields_elimination {:?}", &fields_elimination);
    println!("field_indices_to_solve {:?}", field_indices_to_solve);

    // we can check the hashsets against each other to determine missing fields
    let mut key_to_remove = String::from("");
    let mut ix_to_remove: Option<usize> = None;
    let mut elim_check = fields_elimination.len();
    
    'panic_loop: while elim_check > 0 {
    
        for (this_key, this_set) in &fields_elimination {
            println!("{:?}", (this_key, this_set.len()));
    
        }
        // go through the fields that are left unsolved
        'elim_loop: for (this_key, this_set) in &fields_elimination {
            println!("{:?}", (elim_check, this_set.len()));
            // check if there's only one option left

            if this_set.len() == elim_check-1 {
                
                println!("range check success");
                let diff = field_indices_to_solve.difference(&this_set).collect::<Vec<&usize>>();
                println!("{:?} must be in slot {:?}", this_key, diff[0]);

                // add the answer to the master
                fields_master.insert(this_key.clone(), *diff[0]);
                key_to_remove = this_key.clone();
                ix_to_remove = Some(*diff[0]);

                // remove it from the search space
                break 'elim_loop
    
            }
        }
        // I had some issues with Rust's insistence on safety (problems with borrowing and trying to pop
        // things out of the HashSet while looping over said HashSet). Had to split this out later to make
        // if "safer". I think there's a better way to do this mid-loop but im not 100% on it.
        
        // remove it from the search space
        println!("removing {:?}", key_to_remove);
        println!("fields_elimination {:?}", &fields_elimination);
        match &fields_elimination.contains_key(&key_to_remove) {
            true => {
                &fields_elimination.remove(&key_to_remove).unwrap();
                field_indices_to_solve.remove(&ix_to_remove.unwrap());
                
                key_to_remove = String::from("");
            },
            false => {
                println!("attempted to remove key that wasn't there!");
                break 'panic_loop;
            },
        }
        println!("fields_elimination {:?}", &fields_elimination);
        println!("field_indices_to_solve {:?}", field_indices_to_solve);
        println!("fields_master {:?}", fields_master);
        elim_check = fields_elimination.len();
    }
    

}

fn part_1(file_data: &String) {

    // a ton of this puzzle is file parsing, so lets get cracking
    let splits: Vec<&str> = file_data.split("\n").collect(); // this will split each line seperately

    let mut my_ticket: Vec<usize> = Vec::new();
    let mut nearby_tickets: Vec<Vec<usize>> = Vec::new();
    let mut rules: HashMap<&str, Vec<(usize, usize)>> = HashMap::new();

    let mut file_line_iter = splits.iter();

    // set up regex
    let re = Regex::new(r" (.*)-(.*) or (.*)-(.*)").unwrap();

    while let Some(mut line) = file_line_iter.next() {
        // the file has three categories, field info, my ticket, and nearby tickets
        // all of the rules follow a <this> or <that> range, and none of them overlap
        let words: Vec<&str> = line.split(" ").collect();

        // println!("{:?}", words);
        // check if this is a constraint line
        if words[0].contains(":") {
            // another example of "i should probably just learn regex"
            // println!("parsing constraint");
            let capture = re.captures(line).unwrap();
            rules.insert(words[0], vec![(capture[1].parse::<usize>().unwrap(), capture[2].parse::<usize>().unwrap()),
                                        (capture[3].parse::<usize>().unwrap(), capture[4].parse::<usize>().unwrap())]);
        }

        if *line == "your ticket:" {
            // println!("parsing my ticket");
            // get the next line
            line = file_line_iter.next().unwrap();
            // split on commas
            let words: Vec<&str> = line.split(",").collect();
            // println!("{:?}", words);
            for num in words {
                my_ticket.push(num.parse::<usize>().unwrap());
            }
        }

        if *line == "nearby tickets:" {
            // println!("parsing nearby tickets");
            // assume all other lines here are nearby tickets
            // get the next line
            while let Some(line) = file_line_iter.next() {
                let mut temp_vec: Vec<usize> = Vec::new();
                // split on commas
                let words: Vec<&str> = line.split(",").collect();
                // println!("{:?}", words);
                for num in words {
                    temp_vec.push(num.parse::<usize>().unwrap());
                }
                // make sure to push a copy of the vector
                // this is very likely incredibly slow
                nearby_tickets.push(temp_vec.clone());

            }
            
        }


        

    }

    // println!("rules {:?}", rules);
    // println!("my ticket {:?}", my_ticket);
    // println!("nearby tickets {:?}", nearby_tickets);

    // the error rate is equal to sum of all numbers that are not valid in any range

    // I should parse the rules and find the min, max, and all values that are not within the rules, but
    // I have a feeling part 2 will bite me on that. Instead (and this is slower probably) im going to
    // just "enforce" the rules across the board

    let mut err_sum: usize = 0;

    for this_ticket in nearby_tickets {
        for val in this_ticket {
            let mut val_is_valid = false;
            'check_loop: for (_, rule_bounds) in &rules {
                for (lower_limit, upper_limit) in rule_bounds {
                    if (val >= *lower_limit) && (val <= *upper_limit) {
                        // value is valid, we're good to proceed
                        val_is_valid = true;
                        break 'check_loop;
                    }
                }
            }
            // we've determined that the value isnt in any of the ranges
            // add that number to the error rate
            if val_is_valid == false {
                err_sum += val;
            }
        }        
    }

    println!("err rate: {}", err_sum);}
