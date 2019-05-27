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
    
    X, Y, and Z coordinates correspond to the "NE", "NW", and "S" directions,
    respectively, and must always add up to 0. See diagram below for clarity:

              _______
             /       \
     _______/ 1, 1,-2 \_______
    /       \         /       \  
   / 0, 1,-1 \_______/ 1, 0,-1 \
   \         /       \         /
    \_______/ 0, 0, 0 \_______/
    /       \         /       \
   /-1, 0, 1 \_______/ 0,-1, 1 \
   \         /       \         /
    \_______/ 0, 0, 2 \_______/
            \         /
             \_______/

    See World Grid module for the maximum values of the axes.

Changelog:

\* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * */

use std::fmt;
use std::error::Error;
use std::f64::consts::*;

///////////////////////////////////////////////////////////////////////////////
//  Data structures
///////////////////////////////////////////////////////////////////////////////

// Define cardinal and sub-cardinal directions for ease-of-use
pub static EAST:        f64 = 0.0;
pub static NORTHEAST:   f64 = FRAC_PI_4; 
pub static NORTH:       f64 = FRAC_PI_2;
pub static NORTHWEST:   f64 = 3.0 * FRAC_PI_4;
pub static WEST:        f64 = PI;
pub static SOUTHWEST:   f64 = 5.0 * FRAC_PI_4;
pub static SOUTH:       f64 = 3.0 * FRAC_PI_2;
pub static SOUTHEAST:   f64 = 7.0 * FRAC_PI_4;

pub struct Coords {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Debug)]
pub struct CoordsError;

// Defines the limit of a negligible fractional movement
const MIN_FRACTIONAL_MOVE: f64 = 0.01;

///////////////////////////////////////////////////////////////////////////////
//  Functions and Methods
///////////////////////////////////////////////////////////////////////////////
 
impl Coords {
    // Creates a new coordinates object
    pub fn new() -> Coords {
        Coords{
            x: 0,
            y: 0,
            z: 0,
        }
    }

    // Creates a new coordinates object at the given (sanity-checked) coordinates
    pub fn new_at( _x: i32, _y: i32, _z: i32) -> Result<Coords, CoordsError> {
        if  _x + _y + _z == 0 {
            Ok(Coords {
                x: _x,
                y: _y,
                z: _z,
            })
        } else {
            Err(CoordsError)
        }
    }

    ///////////////////////////////////////////////////////////////////////////
    //  Mutator Methods
    ///////////////////////////////////////////////////////////////////////////
    
    // Moves the object by vector
    //  _mag: number of "straightline" cells to move
    //  _dir: direction of movement in radians
    pub fn move_vec(&mut self, _mag: i32, _dir: f64) {
        debug_println!("START coord.move_vec()");
        debug_println!("_mag: {}, _dir: {:.4}", _mag, _dir);
        
        let flt_mag: f64 = _mag as f64;

        // Determine lateral movement
        if _dir.cos().abs() > MIN_FRACTIONAL_MOVE {
            let mut lat_mag: f64 = flt_mag * _dir.cos();
            debug_println!("Lat mag: {:.2}", lat_mag);

            // Adjust such that non-negligible fractional movements round to next larger integer
            if lat_mag.fract().abs() > MIN_FRACTIONAL_MOVE {
                lat_mag = lat_mag.trunc() + (1.0 * lat_mag.signum());
            }

            //TODO:Check for overflow
            //let _tempX = match (self.x += lat_mag as i32) {
            //    Ok( )
            //}
            // Set movement
            self.x += lat_mag as i32;
            self.y -= lat_mag as i32;
            debug_println!("move_east by: {}", lat_mag as i32);
        }

        // Approximate vertical movement with partial NE/NW movement
        if _dir.sin().abs() > MIN_FRACTIONAL_MOVE {
            let mut vert_mag: f64 = flt_mag * _dir.sin();
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
    
    pub fn x(&self) -> i32
    {
        self.x
    }
    pub fn y(&self) -> i32
    {
        self.y
    }
    pub fn z(&self) -> i32
    {
        self.z
    }
}

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
    use super::*;

    #[test]
    fn square_hemisphere() {
        // Initialize values
        let mut actual = Coords::new();
        let mut expected = Coords::new();

        // Initial position
        assert_eq!(actual, expected);
        
        // Move 3 cells East
        actual.move_vec(3, EAST);
        if let Ok(expected) = Coords::new_at(3, -3, 0) {
            assert_eq!(actual, expected);
        }

        // Move 4 cells North
        actual.move_vec(4, NORTH);
        if let Ok(expected) = Coords::new_at(5, -1, -4) {
            assert_eq!(actual, expected);
        }
        
        // Move 6 cells West
        actual.move_vec(6, WEST);
        if let Ok(expected) = Coords::new_at(-1, 5, -4) {
            assert_eq!(actual, expected);
        }
        
        // Move 4 cells South
        actual.move_vec(4, SOUTH);
        if let Ok(expected) = Coords::new_at(-3, 3, 0) {
            assert_eq!(actual, expected);
        }
        
        // Move 3 cells East
        actual.move_vec(3, EAST);
        expected = Coords::new();
        assert_eq!(actual, expected);
    }

    #[test]
    fn move_max_cardinal_dirs() {
        // Initialize values
        let mut actual = Coords::new();

        // Move East by INT_MAX
        actual.move_vec(i32::max_value(), EAST);

        // Move one more unit East
        actual.move_vec(1, EAST);
    }
}