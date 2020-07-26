/* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *\
Filename : environment\coords.rs

Copyright (C) 2017 CJ McAllister
    This program is free software; you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation; either version 3 of the License, or
    (at your option) any later version.
    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.
    You should have received a copy of the GNU General Public License
    along with this program; if not, write to the Free Software Foundation,
    Inc., 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301  USA

Purpose:
    This module defines the Coords object, which denotes an actors location in
    CastIron's hexagonal grid.

    (0, 0, 0) is the centermost hexagon in the world grid. 
    
    X, Y, and Z coordinates correspond to "East", "Northwest", and "SouthWest" directions,
    respectively, and must always add up to 0. See diagram below for clarity:

              _______
             /       \
     _______/ 0, 1,-1 \_______
    /       \         /       \  
   /-1, 1, 0 \_______/ 1, 0,-1 \
   \         /       \         /
    \_______/ 0, 0, 0 \_______/
    /       \         /       \
   /-1, 0, 1 \_______/ 1,-1, 0 \
   \         /       \         /
    \_______/ 0,-1, 1 \_______/
            \         /
             \_______/

    See World Grid module for the maximum values of the axes.

\* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * */

use crate::context::Context;

use std::fmt;
use std::error::Error;

use rand::Rng;


///////////////////////////////////////////////////////////////////////////////
//  Data structures
///////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy)]
pub struct Coords {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Debug)]
pub struct CoordsError;

// Defines the limit of a negligible fractional movement
const MIN_FRACTIONAL_MOVE: f32 = 0.01;


///////////////////////////////////////////////////////////////////////////////
//  Functions and Methods
///////////////////////////////////////////////////////////////////////////////
 
impl Coords {
    //FIXME: Should implement Default trait instead
    /// Creates a coordinates object at (0, 0, 0)
    pub fn default() -> Self {
        Self {
            x: 0,
            y: 0,
            z: 0,
        }
    }
    
    /// Creates a new coordinates object at the given (sanity-checked) coordinates
    pub fn new(x: i32, y: i32, z: i32) -> Result<Self, CoordsError> {
        if  x + y + z == 0 {
            Ok(Self {
                x: x,
                y: y,
                z: z,
            })
        } else {
            Err(CoordsError)
        }
    }

    /// Creates a random, valid Coords object within the constraints of the game Context
    pub fn rand(ctx: &Context) -> Self {
        let mut rng = rand::thread_rng();
        
        //OPT: Will always calculating the z-value lead to non-random coordinates?
        let rand_x: i32 = rng.gen_range(-1*(ctx.get_grid_radius() as i32), ctx.get_grid_radius() as i32 + 1);
        let rand_y: i32 = rng.gen_range(-1*(ctx.get_grid_radius() as i32), ctx.get_grid_radius() as i32 + 1);
        let calc_z: i32 = 0 - rand_x - rand_y; // Coords must meet the x + y + z == 0 requirement

        Self::new(rand_x, rand_y, calc_z).unwrap()
    }

    /// Creates a random, valid Coords object within the constraints fo the game Context AND
    /// constrained the given number cells away from the edge of the hex grid
    pub fn rand_constrained(ctx: &Context, dist_from_edge: u8) -> Result<Self, CoordsError> {
        // Ensure that the distance from the edge is less than the Context's grid radius
        if dist_from_edge >= ctx.get_grid_radius() {
            return Err(CoordsError)
        }

        let max_dist = (ctx.get_grid_radius() - dist_from_edge) as i32;
        
        let mut rng = rand::thread_rng();
        
        //OPT: Will always calculating the y & z values lead to non-random coordinates?
        // debug_println!("max_dist: {}", max_dist);
        let rand_x: i32 = rng.gen_range(-1 * max_dist, max_dist);
        // debug_println!("rand_x: {}", rand_x);
        // Need to base the next coordinate off of x. Can't be higher than X unless X is 0
        let calc_rand_y = match rand_x {
            i32::MIN..=-1   => rng.gen_range(0,             rand_x.abs()),  // X is negative, generate a bounded-positive Y
            0               => rng.gen_range(-1 * max_dist, max_dist),      // X is 0, generate an unbounded Y
            1..=i32::MAX    => rng.gen_range(-1 * rand_x,   0)              // X is positive, generate a bounded-negative Y
        };
        // debug_println!("calc_rand_y: {}", calc_rand_y);
        let calc_z: i32 = 0 - rand_x - calc_rand_y; // Coords must meet the x + y + z == 0 requirement
        // debug_println!("calc_z: {}", calc_z);



        Ok(Self::new(rand_x, calc_rand_y, calc_z).unwrap())
    }

    ///////////////////////////////////////////////////////////////////////////
    //  Mutator Methods
    ///////////////////////////////////////////////////////////////////////////
    
    // Moves the object by vector
    //  mag: number of "straightline" cells to move
    //  dir: direction of movement in radians
    pub fn move_vec(&mut self, mag: i32, dir: f32) {
        debug_println!("START coord.move_vec()");
        debug_println!("mag: {}, dir: {:.4}", mag, dir);
        
        let flt_mag: f32 = mag as f32;

        // Determine lateral movement
        if dir.cos().abs() > MIN_FRACTIONAL_MOVE {
            let mut lat_mag: f32 = flt_mag * dir.cos();
            debug_println!("Lat mag: {:.2}", lat_mag);

            // Adjust such that non-negligible fractional movements round to next larger integer
            if lat_mag.fract().abs() > MIN_FRACTIONAL_MOVE {
                lat_mag = lat_mag.trunc() + (1.0 * lat_mag.signum());
            }

            //TODO:Check for overflow
            //let tempX = match (self.x += lat_mag as i32) {
            //    Ok( )
            //}
            // Set movement
            self.x += lat_mag as i32;
            self.y -= lat_mag as i32;
            debug_println!("move_east by: {}", lat_mag as i32);
        }

        // Approximate vertical movement with partial NE/NW movement
        if dir.sin().abs() > MIN_FRACTIONAL_MOVE {
            let mut vert_mag: f32 = flt_mag * dir.sin();
            debug_println!("Vert mag: {:.2}", vert_mag);

            // Adjust such that non-negligible fractional movements round to next larger integer
            if vert_mag.fract().abs() > MIN_FRACTIONAL_MOVE {
                vert_mag = vert_mag.trunc() + (1.0 * vert_mag.signum());
            }

            // move "NE" for half and "NW" for half of vert_mag, adding any odd moves to "NE"
            let ne_mag = ((vert_mag as i32) / 2) + ((vert_mag as i32) % 2);
            let nw_mag = (vert_mag as i32) / 2;

            // Set movement
            self.x += ne_mag;
            self.y += nw_mag;
            self.z -= ne_mag + nw_mag;
            debug_println!("move_ne by: {}", ne_mag);
            debug_println!("move_nw by: {}", nw_mag);
        }
        
        debug_println!("END coord.move_vec()");
    }

    ///////////////////////////////////////////////////////////////////////////
    //  Accessor Methods
    ///////////////////////////////////////////////////////////////////////////
    
    pub fn get_x(&self) -> i32
    {
        self.x
    }
    pub fn get_y(&self) -> i32
    {
        self.y
    }
    pub fn get_z(&self) -> i32
    {
        self.z
    }
}

//FIXME: Some kind of distribution implementation

// Debug output format for coordinates
impl fmt::Debug for Coords {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Coords: {{X: {} Y: {} Z: {}}}", self.x, self.y, self.z)
    }
}

// Display output format for coordinates
impl fmt::Display for Coords {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{},{})", self.x, self.y, self.z)
    }
} 

// Equivalence comparison 
impl PartialEq for Coords {
    fn eq(&self, other: &Coords) -> bool {
        (self.x == other.x) && (self.y == other.y) && (self.z == other.z)
    }
}
impl Eq for Coords {}

// Denote coordinate error info
impl fmt::Display for CoordsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid Coordinates. Sum must equal 0.")
    }
}
impl Error for CoordsError {
    fn description(&self) -> &str {
        "Invalid Coordinates. Sum must equal 0."
    }
}


///////////////////////////////////////////////////////////////////////////////
//  Unit Tests
///////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use std::f32::consts::PI;
    use super::*;

    // Define cardinal and sub-cardinal directions for ease-of-use
    pub static EAST:        f32 = 0.0;
    pub static NORTH:       f32 = PI/2.0;
    pub static WEST:        f32 = PI;
    pub static SOUTH:       f32 = 3.0 * PI/4.0;

    #[test]
    fn square_hemisphere() {
        // Initialize values
        let mut actual = Coords::default();
        let mut expected = Coords::default();

        // Initial position
        assert_eq!(actual, expected);
        
        // Move 3 cells East
        actual.move_vec(3, EAST);
        if let Ok(expected) = Coords::new(3, -3, 0) {
            assert_eq!(actual, expected);
        }

        // Move 4 cells North
        actual.move_vec(4, NORTH);
        if let Ok(expected) = Coords::new(5, -1, -4) {
            assert_eq!(actual, expected);
        }
        
        // Move 6 cells West
        actual.move_vec(6, WEST);
        if let Ok(expected) = Coords::new(-1, 5, -4) {
            assert_eq!(actual, expected);
        }
        
        // Move 4 cells South
        actual.move_vec(4, SOUTH);
        if let Ok(expected) = Coords::new(-3, 3, 0) {
            assert_eq!(actual, expected);
        }
        
        // Move 3 cells East
        actual.move_vec(3, EAST);
        expected = Coords::default();
        assert_eq!(actual, expected);
    }

    #[test]
    fn move_max_cardinal_dirs() {
        // Initialize values
        let mut actual = Coords::default();

        // Move East by INT_MAX
        actual.move_vec(i32::max_value(), EAST);

        // Move one more unit East
        actual.move_vec(1, EAST);
    }
}