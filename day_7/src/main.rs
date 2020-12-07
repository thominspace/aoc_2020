use std::time::{Instant};
use std::io::{Error};
use std::fs;
use std::collections::HashMap;
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
// assume each line has a new entry
// a lot of this will be string parsing
// most of the formatting should be <descriptor> <color> bag contains {<number> <descriptor> <color> bags,... OR no other bags}
// the prefix should be consistently 4 words: <descriptor> <color> bag contains
// the suffix can be split by a comma
// if the first element is "no other bags" we stop

// Probably want a struct containing the color and the descriptor of each bag

// Could I just use <descriptor> <color> as a single string to index to index the hashmap?
// Yes.
// Do I think part 2 is going to care about either the description or the color of the bag?
// Also yes.

// what we want in the end is the total number of bags that can contain a gold bag
// this is tree traversal (I think) but I'm a bit scared of recursive loops here.
// we can put a loop limiter in, since we know that it can only be as deep as it is long

// I wonder if hashmaps can index off of enums/structs

#[derive(Clone, Hash, Eq, PartialOrd, Ord, PartialEq, Debug)]
struct Bag {
    BagType: String,
    BagColor: String,
}


fn part_1(file_data: &String) {
    let splits: Vec<&str> = file_data.split("\n").collect(); // this will split each line seperately

    let mut bag_rulez: HashMap<Bag, Vec<(usize, Bag)>> = HashMap::new();

    for line in splits {
        // this is the part where I go "I should probably just learn some fucking regex"        
        let re_full = Regex::new(r"^(\S+)\s(\S+) bags contain (.*).").unwrap(); // note had to use an r string here which im assuming is a literal or something
        // let re_rules = Regex::new(r"^(\S+)\s(\S+) bags").unwrap(); // note had to use an r string here which im assuming is a literal or something
        
        println!("{}", line);

        let cap = re_full.captures(line).unwrap();
        // println!("type: {} color: {} rules: {}", &cap[1], &cap[2], &cap[3]);
        let this_bag = Bag { // split out for clarity, this one's about being easy to read not short
            BagType: String::from(cap.get(1).map_or("", |m| m.as_str())),
            BagColor: String::from(cap.get(2).map_or("", |m| m.as_str())) };
        // add it to the map
        // bag_rulez.insert(Bag { // split out for clarity, this one's about being easy to read not short
        //     BagType: String::from(&cap[1]),
        //     BagColor: String::from(&cap[2]) }, Vec::new());
        bag_rulez.insert(this_bag.clone(), Vec::new());
        match &cap[3] {
            "no other bags" => (),
            _ => {
                // we have some extra stuff. split on commas and the trailing whitespace (we already regex'd out the period!)
                let rulez: Vec<&str> = cap[3].split(", ").collect();
                for rule in rulez { // for each of our rules
                    // simple regex (i.e. "I dont know how to do it right") would miss bag vs bags. So just split on spaces
                    let split_rule: Vec<&str>  = rule.split(" ").collect();
                    // println!("---- number: {} type: {} color: {}", split_rule[0], split_rule[1], split_rule[2]);
                    let num_bags = split_rule[0].parse::<usize>().unwrap();
                    let sub_bag = Bag {
                        BagType: String::from(split_rule[1]),
                        BagColor: String::from(split_rule[2]) 
                    };
                    let x = bag_rulez.get_mut(&this_bag).unwrap();
                    x.push((num_bags, sub_bag));
                    // println!("{:?}", this_bag)
                }
            }
        }
        // we should now have processed an entire line.
        println!("{:?}", bag_rulez.get(&this_bag).unwrap());

    }
    // we should now have processed ALL lines
    println!("{:?}", bag_rulez);

    // now, we can solve the puzzle
    let iter_limit = bag_rulez.len();
    println!("iter limit: {}", iter_limit);

    let target_bag = Bag{BagType: String::from("shiny"), BagColor: String::from("gold")};
    let mut total_valid_bags = 0;
    let mut current_bag_num = 0;

    for (bag, rule) in &bag_rulez { // check all our unique bag rules
        println!("************* NOW SEARCHING BAG NUMBER {} OF {} {:?} *************", current_bag_num, bag_rulez.len()-1, bag);
        if bag == &target_bag { //it's the key, we're good
            println!("found a bag (it's the key)");
            // total_valid_bags += 1;
        }
        else {

            // let mut bag_found = false;
            let mut iter_count: usize = 0;
            'check_loop: for (_this_num, this_subbag) in rule { // why not match here? Well, it's in case we have empty bag vectors. That and we need to dive
                // println!("---- WOOP {:?}", this_subbag);
                if check_nested_bag(&bag_rulez, &this_subbag, &target_bag, &mut iter_count, &iter_limit){
                    println!("----  ---- Got it!");
                    total_valid_bags += 1;
                    break 'check_loop;
                }
            }
        }
        current_bag_num += 1;
        
    }
    println!("total valid bags: {}", total_valid_bags)
}

fn check_nested_bag(bag_rules: &HashMap<Bag, Vec<(usize, Bag)>>, this_bag: &Bag, target_bag: &Bag, iter_count: &mut usize, iter_limit: &usize) -> bool {
    if this_bag == target_bag {
        println!("---- found it in a subbag!");
        return true;
    }
    // else if *iter_count < *iter_limit {
        *iter_count += 1;
        let ref sub_bags: Vec<(usize, Bag)> = *bag_rules.get(&this_bag).unwrap();
        for (_this_num, this_subbag) in sub_bags {
            // println!("---- Searching in {:?} on iter {}", this_subbag, iter_count);
            if check_nested_bag(&bag_rules, &this_subbag, &target_bag, iter_count, iter_limit) {
                return true;
            }
        }
    // }
    return false;
}

fn part_2(file_data: &String) {}