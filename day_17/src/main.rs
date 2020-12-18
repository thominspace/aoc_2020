use std::time::{Instant};
use std::io::{Error};
use std::fs;

#[derive(Debug)]
struct Grid {
    flat_grid: Vec<usize>,
    grid_width: usize,
    total_grid_cells: usize,
    expansion: usize

}

impl Grid {
    // if we ever hit a point where we need to expand the grid, handle that here
    // this includes a full copy of the grid in order to reindex it
    fn expand_grid(&mut self) {
        // save the old grid
        let temp_grid: Vec<usize> = self.flat_grid.clone();
        // let temp_grid_width = self.grid_width.clone();

        // need to translate all of the inidices in the grid into cartesian coords, so we can reinsert them into the new grid
        // we also need to do this before we increase the grid size, since it requires old info
        let mut temp_reindex: Vec<(isize,isize,isize)> = Vec::with_capacity(temp_grid.len());
        for ix in 0..temp_grid.len() {
            temp_reindex.push(self.ix_to_cart(ix).unwrap());
        }

        // expand the grid
        self.grid_width += 2;
        self.expansion += 1;
        self.total_grid_cells = self.grid_width.pow(3);

        // reinit the grid
        self.flat_grid = vec![0; self.total_grid_cells];


        // fill with old data
        for (ix, (x, y, z)) in temp_reindex.iter().enumerate() {
            // convert the old cartesian into new index
            let new_ix = self.cart_to_ix(*x, *y, *z).unwrap();
            // insert the data
            self.flat_grid[new_ix] = temp_grid[ix];
        }
    }
    
    // convert a cartesian position to an array index
    fn cart_to_ix(&self, x: isize, y: isize, z: isize) -> Option<usize> {
        let grid_isize = self.grid_width as isize;
        let expansion_isize = self.expansion as isize;
        let return_val = (x+expansion_isize) + ((y+expansion_isize)*grid_isize) + ((z+expansion_isize)*grid_isize*grid_isize);
    
        // check boundary conditions
        if ((x+expansion_isize) >= 0) && ((x+expansion_isize) < (self.grid_width as isize)) &&
           ((y+expansion_isize) >= 0) && ((y+expansion_isize) < (self.grid_width as isize)) &&
           ((z+expansion_isize) >= 0) && ((z+expansion_isize) < (self.grid_width as isize)) {
            Some(return_val as usize)
        } else {
            None
        }
    }
    
    // converts an array index to a cartesian location
    fn ix_to_cart(&self, index: usize) -> Option<(isize, isize, isize)> {
        let max_ix = self.grid_width.pow(3);
        // let return_val = (x+expansion_isize) + ((y+expansion_isize)*grid_isize) + ((z+expansion_isize)*grid_isize*grid_isize);
    
        // check boundary conditions
        if index < max_ix {
            let return_x = (index % self.grid_width) as isize - self.expansion as isize;
            let return_y = ((index / self.grid_width) % self.grid_width) as isize - self.expansion as isize;
            let return_z = (index / (self.grid_width.pow(2))) as isize - self.expansion as isize;
            Some((return_x, return_y, return_z))
        } else {
            None
        }
    }

    // finds (and verifies) all neighbors given an index
    fn get_neighbors_ix(&self, index: usize) -> Vec<Option<usize>> {
        let mut return_vec: Vec<Option<usize>> = Vec::new();
        let this_point_option: Option<(isize, isize, isize)> = self.ix_to_cart(index);
        match this_point_option {
            Some((this_x, this_y, this_z)) => {
                // cycle through neighbors
                for delta_z in -1..2 {
                    for delta_y in -1..2 {
                        for delta_x in -1..2 {
                            // get the neighbor at this delta, but skip the 0,0,0 case
                            if (delta_x == 0) && (delta_y == 0) && (delta_z == 0) {
                                // do nothing
                            } else {                                
                                // return the location (which is an option)
                                return_vec.push(self.cart_to_ix(this_x+(delta_x as isize), this_y+(delta_y as isize), this_z+(delta_z as isize)))
                            }
                        }
                    }
                }

            },
            None => println!("PANIK"),
        }

        return return_vec;
    }

    // check just the perimeter for 1 values. if there are none, then we know we do not need to extend the grid based on the rules
    fn check_perimiter(&self) -> bool {

        // get the expansion as an int
        let search_min = -(self.expansion as isize);
        let search_max = (self.grid_width-self.expansion) as isize;

        // search each face. This is going to contain overlap in the search space, but for now that's fine.
        // x faces
        for &ix in [search_min, search_max-1].iter() {
            for iy in search_min..search_max {
                for iz in search_min..search_max {
                    let this_coord_as_ix = self.cart_to_ix(ix, iy, iz);
                    match this_coord_as_ix {
                        Some(index) => {                            
                            if self.flat_grid[index] == 1 {
                                return true
                            }
                        },
                        None => println!("PANIK while checking perimiter")
                    }
                }
            }
        }
        // y faces
        for &iy in [search_min, search_max-1].iter() {
            for ix in search_min..search_max {
                for iz in search_min..search_max {
                    let this_coord_as_ix = self.cart_to_ix(ix, iy, iz);
                    match this_coord_as_ix {
                        Some(index) => {                            
                            if self.flat_grid[index] == 1 {
                                return true
                            }
                        },
                        None => println!("PANIK while checking perimiter")
                    }
                }
            }
        }
        // z faces
        for &iz in [search_min, search_max-1].iter() {
            for iy in search_min..search_max {
                for ix in search_min..search_max {
                    let this_coord_as_ix = self.cart_to_ix(ix, iy, iz);
                    match this_coord_as_ix {
                        Some(index) => {                            
                            if self.flat_grid[index] == 1 {
                                return true
                            }
                        },
                        None => println!("PANIK while checking perimiter")
                    }
                }
            }
        }
        // if we checked everything and found nothing, we're done here
        return false;        
    }
}

fn build_grid(grid_width: usize) -> Grid {
    // grid initializer
    Grid {
        flat_grid: vec![0; grid_width.pow(3)],
        grid_width: grid_width,
        total_grid_cells: grid_width.pow(3),
        expansion: 0
    }
}

fn main() -> Result<(), Error> {
    // seeing as pretty much every puzzle is going to be reading in a file and then manipulating the input, 
    // I might as well just build main out to be a template

    // read the input for today's puzzle
    let filepath = "input.txt";
    // let filepath = "test_input.txt";
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
    
    // trying to make it a dynamically expanding grid
    // grab the first line as the width of the grid
    let initial_grid_width = splits[0].len();
    let mut my_grid = build_grid(initial_grid_width);
    println!("my_grid {:?}", my_grid);

    // insert the inital data into the grid
    for (iy, this_str) in splits.iter().enumerate() {
        let this_string = String::from(*this_str);
        for (ix, this_char) in this_string.chars().enumerate() {
            let mut this_cell: usize = 0;
            match this_char {
                '.' => this_cell = 0,
                '#' => this_cell = 1,
                _ => ()
            }
            let this_ix = my_grid.cart_to_ix(ix as isize, iy as isize, 0 as isize).unwrap();
            my_grid.flat_grid[this_ix] = this_cell;
            println!("cart_to_ix {:?}", my_grid.cart_to_ix(ix as isize, iy as isize, 0));
        }
    }

    println!("cart_to_ix {:?}", my_grid.cart_to_ix(0 as isize, 4 as isize, 1 as isize));
    println!("ix_to_cart {:?}", my_grid.ix_to_cart(0));
    println!("ix_to_cart {:?}", my_grid.ix_to_cart(17));

    // my_grid.expand_grid();
    // println!("my_grid {:?}", my_grid);
    // expand_grid(&mut grid_len, &mut total_grid_cells, &mut expansion);
    // println!("{}, {}, {}", grid_len, total_grid_cells, expansion);
    // expand_grid(&mut grid_len, &mut total_grid_cells, &mut expansion);
    // println!("{}, {}, {}", grid_len, total_grid_cells, expansion);

    // time iteration
    for _ in 0..6 {
        // based on the rules, if the outermost shell of the space is empty we dont need to expand.
        // Otherwise, we might, so go ahead and expand
        if my_grid.check_perimiter() {
            my_grid.expand_grid();
        }

        // make a copy of the grid. Yeah, it sucks, but we can't overwrite in place
        let mut temp_grid = my_grid.flat_grid.clone();
    
        // now enforce the rules
        // If a cube is active and exactly 2 or 3 of its neighbors are also active, the cube remains active. Otherwise, the cube becomes inactive.
        // If a cube is inactive but exactly 3 of its neighbors are active, the cube becomes active. Otherwise, the cube remains inactive.
        // go through each cell (using the main) and overwrite into the copy
        for index in 0..my_grid.total_grid_cells {
            // need the neighbors so we can check them
            let these_neighbors: Vec<Option<usize>> = my_grid.get_neighbors_ix(index);
            // need to track the neighbor conditions (as per da rulez)
            let mut neighbor_check = 0;
            match my_grid.flat_grid[index] {
                0 => { // cube is inactive
                    for this_neighbor in these_neighbors {
                        // check the neighbor for a valid cell (some may not exist)
                        match this_neighbor {
                            Some(neighbor_ix) => {
                                // we found a valid cell. add the value (beacuse we need some "exactly"s in the rules)
                                neighbor_check += my_grid.flat_grid[neighbor_ix];                                
                            },
                            None => (),
                        }
                    }
                    // we have now checked every neighbor, so now check da rulez
                    if neighbor_check == 3 {
                        // we have met the condition wherein we want to activate the cell
                        temp_grid[index] = 1;
                    } else {
                        // the cell should remain inactive, however we want to assure that state is set in the copy.
                        temp_grid[index] = 0;
                    }
                },
                1 => { // cube is active
                    for this_neighbor in these_neighbors {
                        // check the neighbor for a valid cell (some may not exist)
                        match this_neighbor {
                            Some(neighbor_ix) => {
                                // we found a valid cell. add the value (beacuse we need some "exactly"s in the rules)
                                neighbor_check += my_grid.flat_grid[neighbor_ix];                                
                            },
                            None => (),
                        }
                    }
                    // we have now checked every neighbor, so now check da rulez
                    if (neighbor_check == 3) || (neighbor_check == 2) {
                        // we have met the condition wherein we want the cell to remain activate
                        temp_grid[index] = 1;
                    } else {
                        // the cell should become inactive
                        temp_grid[index] = 0;
                    }
                },
                _ => println!("PANIK in update loop")
            }
        }

        // the temp grid should now be completely updated. copy it back in
        my_grid.flat_grid = temp_grid.clone();

    }

    // now count everything
    let mut count = 0;
    for index in 0..my_grid.total_grid_cells {
        count += my_grid.flat_grid[index];
    }

    println!("count {:?}", count);

}

fn part_2(file_data: &String) {
    
    let mut splits: Vec<&str> = file_data.split("\n").collect(); // this will split each line seperately

}