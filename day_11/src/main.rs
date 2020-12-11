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

fn part_2(file_data: &String) {
    let splits: Vec<&str> = file_data.split("\n").collect(); // this will split each line seperately
}