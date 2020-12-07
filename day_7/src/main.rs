use std::time::{Instant};
use std::io::{Error};
use std::fs;
use std::collections::HashMap;
use regex::Regex;

fn main() -> Result<(), Error> {
    // seeing as pretty much every puzzle is going to be reading in a file and then manipulating the input, 
    // I might as well just build main out to be a template

    // read the input for today's puzzle
    let filepath = "test_input.txt";
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

#[derive(Debug)]
struct Bag {
    BagType: String,
    BagColor: String,
    BagContains: Option(Vec<(isize, Bag)>),
}

impl Bag {
    fn push_rule(&mut self, bag_num: isize, bag_type: String, bag_color: String) {
        self.BagContains.push((bag_num, Bag{BagType: bag_type, BagColor: bag_color, None}));
    }
}


fn part_1(file_data: &String) {
    let splits: Vec<&str> = file_data.split("\n").collect(); // this will split each line seperately

    let mut bag_rulez: HashMap<Bag, Vec<(isize, Bag)>> = HashMap::new();

    for line in splits {
        // this is the part where I go "I should probably just learn some fucking regex"        
        let re_full = Regex::new(r"^(\S+)\s(\S+) bags contain (.*).").unwrap(); // note had to use an r string here which im assuming is a literal or something
        let re_rules = Regex::new(r"^(\S+)\s(\S+) bags").unwrap(); // note had to use an r string here which im assuming is a literal or something
        
        println!("{}", line);

        for cap in re_full.captures_iter(line) {
            // println!("type: {} color: {} rules: {}", &cap[1], &cap[2], &cap[3]);
            let mut this_bag = Bag { // split out for clarity, this one's about being easy to read not short
                BagType: cap[1].to_string(),
                BagColor: cap[2].to_string(),
                BagContains: Vec::new() };
            match &cap[3] {
                // "no other bags" => println!("---- no other bags have been found! we can leave!"),
                _ => {
                    // we have some extra stuff. split on commas and the trailing whitespace (we already regex'd out the period!)
                    let rulez: Vec<&str> = cap[3].split(", ").collect();
                    for rule in rulez { // for each of our rules
                        // simple regex (i.e. "I dont know how to do it right") would miss bag vs bags. So just split on spaces
                        let split_rule: Vec<&str>  = rule.split(" ").collect();

                        // println!("---- number: {} type: {} color: {}", split_rule[0], split_rule[1], split_rule[2]);
                        
                        // push a new rule to the bag
                        this_bag.push_rule( split_rule[0].parse::<isize>().unwrap(),
                                            String::from(split_rule[1]),
                                            String::from(split_rule[2]));

                        // println!("{:?}", this_bag)
                    }


                }
            }
        }
        // we should now have processed an entire line.
        println!("{:?}", this_bag);
    }

    
}

fn part_2(file_data: &String) {}