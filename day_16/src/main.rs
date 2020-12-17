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
                // println!("{:?}", capture);
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
            while let Some(line) = file_line_iter.next() {
                let mut temp_vec: Vec<usize> = Vec::new();
                let mut ticket_is_valid = true;
                // split on commas
                let words: Vec<&str> = line.split(",").collect();
                // println!("{:?}", words);
                // for each number in the ticket string
                'nearby_ticket_insert_loop: for num in words {
                    let mut val_is_valid = false;
                    let this_num = num.parse::<usize>().unwrap();
                    // check this number against all the rules
                    // if this value in the ticket matches NONE of the rules, remove the ticket
                    // otherwise (if it matches at least one rule) add it
                    'check_loop: for (_, rule_bounds) in &rules {
                        for (lower_limit, upper_limit) in rule_bounds {
                            if (this_num >= *lower_limit) && (this_num <= *upper_limit) {
                                // value is valid in some rule somewhere, we're good to proceed
                                val_is_valid = true;
                                break 'check_loop;
                            }
                        }
                    }
                    // if we found a rule that the value follows, add it to the ticket
                    if val_is_valid == true {
                        temp_vec.push(this_num);
                    } else { // if the value followed NO rules, skip this ticket without adding it
                        ticket_is_valid = false;
                        break 'nearby_ticket_insert_loop;
                    }
                }
                // we haven't broken out of the check loop at this point, so go ahead and push the vec
                if ticket_is_valid {
                    nearby_tickets.push(temp_vec.clone());
                }
            }            
        }
    }

    let mut fields_master: HashMap<usize, HashSet<String>> = HashMap::new();
    let mut keys_hashset: HashSet<String> = HashSet::new(); // the .into_keys() method is a nightly only thing and is unstable, doing it manually
    
    for (key, _) in &rules {
        keys_hashset.insert(key.clone());
    }

    // prime the master keys, which is very memory expensive but the way I decided to do it because
    // i got sick of fighting rust ownership
    for ix in 0..rules.len() {
        fields_master.insert(ix, keys_hashset.clone());
    }

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
                // we've check all the bounds. if the value isn't valid here, remove it from
                // the list
                if val_is_valid == false {                    
                    // println!("slot {} can not be {}", ticket_field_ix, rule_field_name);
                    let slot_set = fields_master.get_mut(&ticket_field_ix).unwrap();
                    slot_set.remove(rule_field_name);
                }
            }
        }        
    }

    // // debug checking tickets
    // for (key, val) in &fields_master {
    //     println!("fields_master {:?}", (key, val));
    // }

    // we can check the hashsets against each other to determine missing fields
    let mut answer_map: HashMap<String, usize> = HashMap::new();
    // let mut elim_check = fields_master.len();
    
    for (ix, _) in fields_master.iter().enumerate() {        
        let mut first_field: Option<usize> = None;
        let mut second_field: Option<usize> = None;    
        // let mut first_field: HashSet<String>;
        // let mut second_field: HashSet<String>;
        // look for two sets that are different by one
        for (key, set) in &fields_master {
            if set.len() == ix {
                first_field = Some(key.clone());
            } else if set.len() == ix+1 {
                second_field = Some(key.clone());
            }
        }
        // check for a field with one value in it (that's a known entity)
        // all of the match shenanagins are handling Rust safety stuff
        // and yes, im sure there's a way to do this a lot cleaner but
        // im not about to dump a week into it
        match first_field {
            Some(first_key) => {
                match second_field {
                    Some(second_key) => {                        
                        let diff_left = {
                            let get_set = fields_master.get(&first_key);
                            get_set.unwrap()
                        };                       
                        let diff_right = {
                            let get_set = fields_master.get(&second_key);
                            get_set.unwrap()
                        };
                        let diff = diff_right.difference(&diff_left).collect::<Vec<&String>>(); 
                        // println!("{:?} is in slot {}", diff, second_key);
                        answer_map.insert(diff[0].clone(), second_key);
                    }
                    None => (),
                }
            }
            None => (),
        }
    }

    // need to go back and insert the 1 case
    let mut single_field: Option<usize> = None; 
    for (key, set) in &fields_master {
        if set.len() == 1 {
            single_field = Some(key.clone());
        }
    }
    match single_field {
        Some(first_key) => {                          
            let first_ans = {
                let get_set = fields_master.get(&first_key);
                get_set.unwrap()
            };             
            // println!("{:?} is in slot {}", first_ans.iter().next().unwrap(), first_key);
            // the cheaty way to get the first element of a set that only has one element
            answer_map.insert(first_ans.iter().next().unwrap().clone(), first_key);
        }
        None => (),
    }


    // println!("answer_map {:?}", answer_map);

    let mut ans_multi: usize = 1;

    for (key, ticket_slot) in answer_map {
        if key.contains("departure"){
            ans_multi *= my_ticket[ticket_slot];
        }
    }
    println!("ans_multi {:?}", ans_multi);

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
