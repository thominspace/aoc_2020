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

fn check_seating_rules(seat_ix: &usize, flat_grid: &Vec<char>, grid_width: &usize) -> char {

    // derive grid properties (yes this should be split out)
    let row_ix: usize = seat_ix / grid_width;
    let grid_height: usize = flat_grid.len() / grid_width;
    let this_seat: &char = flat_grid.get(*seat_ix).unwrap();


    let mut up: char = ' ';   
    let up_ix = seat_ix.checked_sub(*grid_width); // we want to keep this to speed things up later
    match up_ix {
        Some(left_ix) => {
            match flat_grid.get(left_ix) { // left
                Some(this_char) => up = *this_char,
                None => up = ' '
            }
        },
        None => up = ' '
    }
    let mut down: char = ' ';
    let down_ix = seat_ix + grid_width; // again, saving this to make things more clear later
    match flat_grid.get(down_ix) { // right
        Some(this_char) => down = *this_char,
        None => down = ' '
    }
    let mut left: char = ' ';
    match seat_ix.checked_sub(1){
        Some(left_ix) => {
            match flat_grid.get(left_ix) { // left
                Some(this_char) => {
                    if seat_ix % grid_width > left_ix % grid_width {
                        left = *this_char;
                    }
                },
                None => left = ' '
            }
        },
        None => left = ' '
    }
    let mut right: char = ' ';
    match flat_grid.get(seat_ix + 1) { // right
        Some(this_char) => { // check in the range of grid
            if seat_ix % grid_width < (seat_ix + 1) % grid_width {
                right = *this_char;
            }
        },
        None => right = ' '
    }
    let mut upleft: char = ' ';
    match up_ix { // check if we found an up ix that worked
        Some(ix) => { // try to subtract
            match ix.checked_sub(1){
                Some(upleft_ix) => { // upleft is valid
                    match flat_grid.get(upleft_ix) { // left
                        Some(this_char) => {
                            if seat_ix % grid_width > upleft_ix % grid_width {
                                upleft = *this_char;
                            }
                        },
                        None => upleft = ' '
                    }
                },
                None => upleft = ' '
            }
        },
        None => upleft = ' '
    }
    let mut upright: char = ' ';
    match up_ix { // check if we found an up ix that worked
        Some(ix) => { // try to subtract
            match flat_grid.get(ix + 1) { // right
                Some(this_char) => { // check in the range of grid
                    if ix % grid_width < (ix + 1) % grid_width {
                        upright = *this_char;
                    }
                },
                None => ()
            }
        },
        None => ()
    }
    let mut downleft: char = ' ';
    match flat_grid.get(down_ix - 1) { // right
        Some(this_char) => { // check in the range of grid
            if down_ix % grid_width > (down_ix - 1) % grid_width {
                downleft = *this_char;
            }
        },
        None => downleft = ' '
    }
    let mut downright: char = ' ';
    match flat_grid.get(down_ix + 1) { // right
        Some(this_char) => { // check in the range of grid
            if down_ix % grid_width < (down_ix + 1) % grid_width {
                downright = *this_char;
            }
        },
        None => downright = ' '
    }

    // println!("---"); 
    // println!("{}{}{}\n{}{}{}\n{}{}{}", 
    //           upleft, up, upright, 
    //           left, this_seat, right,
    //           downleft, down, downright);
    // println!("---");

    let seat_neighbors = [upleft, up, upright, left, right, downleft, down, downright];

    // if L and no occupied nearby, occupy
    let neighbors_occupied = seat_neighbors.iter().filter(|seat| **seat == '#').count();
    if this_seat == & 'L' {
        if neighbors_occupied == 0 {
            // println!("{}", this_seat);
            return '#';
        } else {return 'L'}
    } else if this_seat == &'#' { // if # and 4 or more occupied adjacent (include diags), unoccupy        
        // println!("Seat taken");
        if neighbors_occupied >= 4 {
            return 'L'
        }
        else { return '#'}
    }

    // println!("PANIC");
    return '.'
}

fn part_1(file_data: &String) {
   
    let mut splits: Vec<&str> = file_data.split("\n").collect(); // this will split each line seperately

    // after some brief googling, the concensus was to flatten your arrays instead of doing vectors of vectors
    // ...
    // which sucks
    let mut flat_grid: Vec<char> = splits.join("").chars().collect();
    let mut next_grid: Vec<char> = vec![' '; flat_grid.len()];

    let grid_width = splits[0].len();
    let grid_height = splits.len();

    // . is floor
    // L is empty
    // # is open

    // if L and no occupied nearby, occupy
    // if # and 4 or more occupied adjacent (include diags), unoccupy
    
    let mut changes_made;
    'conway_loop : loop {
    // 'conway_loop : for _ in 0..10 {
        changes_made = false;

        // // printing for debugging
        // let mut print_ix = 0;
        // println!("\n");
        // for print_ix in 0..grid_height {
        //     println!("{:?}", &flat_grid[print_ix*grid_height..print_ix*grid_height+grid_width]);
        // }

        for (ix, this_char) in flat_grid.iter().enumerate() {
            let next_char = check_seating_rules(&ix, &flat_grid, &grid_width);
            if next_char != *this_char { // check if we made changes
                changes_made = true;
            }
            next_grid[ix] = next_char;
        }

        // check if we have our done state
        if changes_made == false { break 'conway_loop} else {flat_grid = next_grid.to_vec()} // no, this is not the right way to do this

    }

    println!("Number of occupied seats: {}", flat_grid.iter().filter(|seat| **seat == '#').count());
}

fn step_up(index: &usize, grid_width: &usize) -> Option<usize> {
    return index.checked_sub(*grid_width);
}

fn step_down(index: &usize, grid_width: &usize, grid_size: &usize) -> Option<usize> {
    match index+grid_width < *grid_size {
        true => return Some(index + grid_width),
        false => return None
    }
}

fn step_left(index: &usize, grid_width: &usize) -> Option<usize> {
    
    match index.checked_sub(1) {
        Some(left_ix) => {
            if index % grid_width > left_ix % grid_width {
                return Some(left_ix);
            } else { return None}
        },
        None => return None
    }

}

fn step_right(index: &usize, grid_width: &usize, grid_size: &usize) -> Option<usize> {
    match (index % grid_width < (index + 1) % grid_width) && (index+1 < *grid_size) {
        true => Some(index+1),
        false => None
    }
}


fn part_2(file_data: &String) {
    
    let mut splits: Vec<&str> = file_data.split("\n").collect(); // this will split each line seperately

    // after some brief googling, the concensus was to flatten your arrays instead of doing vectors of vectors
    // ...
    // which sucks
    let mut flat_grid: Vec<char> = splits.join("").chars().collect();
    let mut next_grid: Vec<char> = vec![' '; flat_grid.len()];

    let grid_width = splits[0].len();
    let grid_height = splits.len();

    // . is floor
    // L is empty
    // # is open

    // if L and no occupied nearby, occupy
    // if # and 4 or more occupied adjacent (include diags), unoccupy
    
    let mut changes_made;
    'conway_loop : loop {
    // 'conway_loop : for _ in 0..10 {
        changes_made = false;

        // // printing for debugging
        // let mut print_ix = 0;
        // println!("\n");
        // for print_ix in 0..grid_height {
        //     println!("{:?}", &flat_grid[print_ix*grid_height..print_ix*grid_height+grid_width]);
        // }

        for (ix, this_char) in flat_grid.iter().enumerate() {
            // println!("ix: {}, this_char: {}", ix, this_char);
            let next_char = check_better_seating_rules(&ix, &flat_grid, &grid_width);
            if next_char != *this_char { // check if we made changes
                changes_made = true;
            }
            next_grid[ix] = next_char;
        }

        // check if we have our done state
        if changes_made == false { break 'conway_loop} else {flat_grid = next_grid.to_vec()} // no, this is not the right way to do this

    }

    println!("Number of occupied seats: {}", flat_grid.iter().filter(|seat| **seat == '#').count());
    
}

fn check_better_seating_rules(seat_ix: &usize, flat_grid: &Vec<char>, grid_width: &usize) -> char {

    // derive grid properties (yes this should be split out)
    let row_ix: usize = seat_ix / grid_width;
    let grid_height: usize = flat_grid.len() / grid_width;
    let ref grid_size: usize = flat_grid.len();
    let this_seat: &char = flat_grid.get(*seat_ix).unwrap();

    let mut seat_neighbors: HashMap<&str, &char> = HashMap::new();
    let seat_neighbors_keys = [
    "upleft",
    "up",
    "upright",
    "left",
    "right",
    "downleft",
    "down",
    "downright"];

    // get the first ones

    // check if there are any open neighbors        
    for key in seat_neighbors_keys.iter() {
        let mut temp_char = &'D'; // just placeholder
        let mut temp_ix = *seat_ix;
        
        match *key {
            "upleft" => { 
                while (temp_char != &' ') && (temp_char != &'#') && (temp_char != &'L') && (temp_char != &'L') {
                    match step_up(&temp_ix, grid_width) {
                            Some(up_ix) => {
                                match step_left(&up_ix, grid_width) {
                                    Some(left_ix) => match flat_grid.get(left_ix) { // right
                                        Some(this_char) => {
                                            temp_ix = left_ix;
                                            temp_char = this_char
                                        },
                                        None => temp_char = &' '
                                    }
                                    None => temp_char = &' '
                            }
                        },
                        None => temp_char = &' '
                    }
                }
                // set the furthest value
                seat_neighbors.insert("upleft", temp_char);
            },
            "up" => {
                while (temp_char != &' ') && (temp_char != &'#') && (temp_char != &'L') {
                    match step_up(&temp_ix, grid_width) {
                        Some(up_ix) => match flat_grid.get(up_ix) { // left
                            Some(this_char) =>  {
                                            temp_ix = up_ix;
                                            temp_char = this_char
                                        },
                            None => temp_char = &' '
                        },        
                        None => temp_char = &' '
                    }
                }
                seat_neighbors.insert("up", temp_char);
            },
            "upright" => {
                while (temp_char != &' ') && (temp_char != &'#') && (temp_char != &'L') {
                    match step_up(&temp_ix, grid_width) {
                            Some(up_ix) => {
                                match step_right(&up_ix, grid_width, grid_size) {
                                    Some(right_ix) => match flat_grid.get(right_ix) { // right
                                        Some(this_char) =>  {
                                            temp_ix = right_ix;
                                            temp_char = this_char
                                        },
                                        None => temp_char = &' '
                                    }
                                    None => temp_char = &' '
                            }
                        },
                        None => temp_char = &' '
                    }
                }
                seat_neighbors.insert("upright", temp_char);
            },
            "left" => {
                while (temp_char != &' ') && (temp_char != &'#') && (temp_char != &'L') {
                    match step_left(&temp_ix, grid_width) {
                        Some(left_ix) => match flat_grid.get(left_ix) { // right
                            Some(this_char) =>  {
                                            temp_ix = left_ix;
                                            temp_char = this_char
                                        },
                            None => temp_char = &' '
                        }
                        None => temp_char = &' '
                    }
                    
                }
                seat_neighbors.insert("left", temp_char);
            },
            "right" => {
                while (temp_char != &' ') && (temp_char != &'#') && (temp_char != &'L') {
                    match step_right(&temp_ix, grid_width, grid_size) {
                        Some(right_ix) => match flat_grid.get(right_ix) { // right
                            Some(this_char) =>  {
                                            temp_ix = right_ix;
                                            temp_char = this_char
                                        },
                            None => temp_char = &' '
                        }
                        None => temp_char = &' '
                    }
                    
                }
                seat_neighbors.insert("right", temp_char);
            },
            "downleft" => {
                while (temp_char != &' ') && (temp_char != &'#') && (temp_char != &'L') {
                    match step_down(&temp_ix, grid_width, grid_size) {
                            Some(down_ix) => {
                                match step_left(&down_ix, grid_width) {
                                    Some(left_ix) => match flat_grid.get(left_ix) { // right
                                        Some(this_char) =>  {
                                            temp_ix = left_ix;
                                            temp_char = this_char
                                        },
                                        None => temp_char = &' '
                                    }
                                    None => temp_char = &' '
                            }
                        },
                        None => temp_char = &' '
                    }     
                }
                seat_neighbors.insert("downleft", temp_char);
            },
            "down" => {    
                while (temp_char != &' ') && (temp_char != &'#') && (temp_char != &'L') {
                    match step_down(&temp_ix, grid_width, grid_size)
                    {
                        Some(down_ix) => match flat_grid.get(down_ix) { // right
                            Some(this_char) =>  {
                                            temp_ix = down_ix;
                                            temp_char = this_char
                                        },
                            None => temp_char = &' '
                        }
                        None => temp_char = &' '
                    }
                }
                seat_neighbors.insert("down", temp_char);
            },
            "downright" => {
                while (temp_char != &' ') && (temp_char != &'#') && (temp_char != &'L') {
                    // println!("FOO {:?}, {:?}", temp_char, temp_char != &' ');
                    match step_down(&temp_ix, grid_width, grid_size) {
                            Some(down_ix) => {
                                match step_right(&down_ix, grid_width, grid_size) {
                                    Some(right_ix) => match flat_grid.get(right_ix) { // right
                                        Some(this_char) =>  {
                                            temp_ix = right_ix;
                                            temp_char = this_char
                                        },
                                        None => temp_char = &'?'
                                    }                                    
                                    None => temp_char = &' '
                            }
                        },
                        None => temp_char = &' '
                    }                    
                }
                seat_neighbors.insert("downright", temp_char);
            },
            _ => ()
        }
    }

    

    // println!("---"); 
    // println!("{:?}{:?}{:?}\n{:?}{:?}{:?}\n{:?}{:?}{:?}", 
    //         seat_neighbors.get("upleft"), seat_neighbors.get("up"), seat_neighbors.get("upright"), 
    //         seat_neighbors.get("left"), this_seat, seat_neighbors.get("right"),
    //         seat_neighbors.get("downleft"), seat_neighbors.get("down"), seat_neighbors.get("downright"));
    // println!("---");

    // for some god awful reason my previous map solution no longer works. I blame HashMap
    let mut neighbors_occupied = 0;
    for &val in seat_neighbors.values() {
        if *val == '#' {
            neighbors_occupied += 1;
        }
    }
    // let seat_neighbors_iter = seat_neighbors.values();
    // let neighbors_occupied = seat_neighbors_iter.map(|&seat| *seat == '#').count();
    // println!("neighbors_occupied {:?}", neighbors_occupied);

    // if L and no occupied nearby, occupy
    if this_seat == & 'L' { // we have an open seat
        if neighbors_occupied == 0 {
            // println!("{}", this_seat);
            return '#';
        } else {return 'L'}
    } else if this_seat == &'#' { // if # and 4 or more occupied adjacent (include diags), unoccupy        
        // check how many neighbors are occupied
        if neighbors_occupied >= 5 {
            return 'L'
        }
        else { return '#'}
    }



    
    // println!("PANIC");
    return '.'
}
