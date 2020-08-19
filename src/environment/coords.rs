/* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *\
Filename : environment/coords.rs

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

\* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * */

use std::{
    error::Error,
    fmt,
    ops::{
        Add,
        Sub,
    },
};

use crate::{
    context::Context,
    hex_direction_provider::HexSides,
    logger,
    ci_log
};

use rand::Rng;


///////////////////////////////////////////////////////////////////////////////
//  Named Constants
///////////////////////////////////////////////////////////////////////////////

// Defines the limit of a negligible fractional movement
const MIN_FRACTIONAL_MOVE: f32 = 0.01;


///////////////////////////////////////////////////////////////////////////////
//  Data structures
///////////////////////////////////////////////////////////////////////////////

#[derive(Default, Clone, Copy, PartialEq)]
pub struct Coords {
    x: i32,
    y: i32,
    z: i32,
}

//OPT: *DESIGN* Needs more specificity - i.e, can be due to invalid composition, out of bounds, etc
#[derive(Debug, Clone)]
pub struct ValidityError;

#[derive(Debug, Clone)]
pub struct ParamError;


///////////////////////////////////////////////////////////////////////////////
//  Object Implementation
///////////////////////////////////////////////////////////////////////////////

impl Coords {
    /// Fully-qualified constructor
    pub fn new(x: i32, y: i32, z: i32, ctx: &Context) -> Result<Self, ValidityError> {
        // Check coords composition
        if x + y + z != 0 {
            return Err(ValidityError)
        }

        // Check for out-of-bounds
        if i32::abs(x) > ctx.grid_radius() as i32 ||
           i32::abs(y) > ctx.grid_radius() as i32 ||
           i32::abs(z) > ctx.grid_radius() as i32 {
            return Err(ValidityError)
        }

        Ok(Self {x, y, z})
    }

    /// Constructs a random, valid Coords object within the constraints of the game Context
    pub fn rand(ctx: &Context) -> Self {
        let max_dist = ctx.grid_radius() as i32;

        let mut rng = rand::thread_rng();

        let rand_x: i32 = rng.gen_range(-max_dist, max_dist);
        let calc_rand_y = match rand_x {
            i32::MIN..=-1   => rng.gen_range(0,             rand_x.abs()),  // X is negative, generate a bounded-positive Y
            0               => rng.gen_range(-max_dist, max_dist),      // X is 0, generate an unbounded Y
            1..=i32::MAX    => rng.gen_range(-rand_x,   0)              // X is positive, generate a bounded-negative Y
        };
        let calc_z: i32 = 0 - rand_x - calc_rand_y; // Coords must meet the x + y + z == 0 requirement

        Self::new(rand_x, calc_rand_y, calc_z, ctx).unwrap()
    }

    /// Constructs a random, valid Coords object within the constraints fo the game Context AND
    /// constrained the given number cells away from the edge of the hex grid
    pub fn rand_constrained(ctx: &Context, dist_from_edge: usize) -> Result<Self, ParamError> {
        // Ensure that the distance from the edge is less than the Context's grid radius
        if dist_from_edge >= ctx.grid_radius() {
            return Err(ParamError)
        }

        let max_dist = (ctx.grid_radius() - dist_from_edge) as i32;

        let mut rng = rand::thread_rng();

        let rand_x: i32 = rng.gen_range(- max_dist, max_dist);
        let calc_rand_y = match rand_x {
            i32::MIN..=-1   => rng.gen_range(0,             rand_x.abs()),  // X is negative, generate a bounded-positive Y
            0               => rng.gen_range(-max_dist, max_dist),      // X is 0, generate an unbounded Y
            1..=i32::MAX    => rng.gen_range(-rand_x,   0)              // X is positive, generate a bounded-negative Y
        };
        let calc_z: i32 = 0 - rand_x - calc_rand_y; // Coords must meet the x + y + z == 0 requirement

        Ok(Self::new(rand_x, calc_rand_y, calc_z, ctx).unwrap())
    }


    ///
    // Mutator Methods
    ///

    // Moves the object by vector
    //  mag: number of "straightline" cells to move
    //  dir: direction of movement in radians
    pub fn move_vec(&mut self, mag: i32, dir: f32, logger: &logger::Instance, ctx: &Context) -> Result<(), ValidityError>{
        ci_log!(logger, logger::FilterLevel::Trace, "START coord.move_vec()");
        ci_log!(logger, logger::FilterLevel::Trace, "mag: {}, dir: {:.4}", mag, dir);

        let mut new_x = self.x;
        let mut new_y = self.y;
        let mut new_z = self.z;

        let flt_mag: f32 = mag as f32;

        // Determine lateral movement
        if dir.cos().abs() > MIN_FRACTIONAL_MOVE {
            let mut lat_mag: f32 = flt_mag * dir.cos();
            ci_log!(logger, logger::FilterLevel::Trace, "Lat mag: {:.2}", lat_mag);

            // Adjust such that non-negligible fractional movements round to next larger integer
            if lat_mag.fract().abs() > MIN_FRACTIONAL_MOVE {
                lat_mag = lat_mag.trunc() + lat_mag.signum();
            }

            //TODO:Check for overflow
            //let tempX = match (self.x += lat_mag as i32) {
            //    Ok( )
            //}
            // Set movement
            new_x += lat_mag as i32;
            new_y -= lat_mag as i32;
            ci_log!(logger, logger::FilterLevel::Trace, "move_east by: {}", lat_mag as i32);
        }

        // Approximate vertical movement with partial NE/NW movement
        if dir.sin().abs() > MIN_FRACTIONAL_MOVE {
            let mut vert_mag: f32 = flt_mag * dir.sin();
            ci_log!(logger, logger::FilterLevel::Trace, "Vert mag: {:.2}", vert_mag);

            // Adjust such that non-negligible fractional movements round to next larger integer
            if vert_mag.fract().abs() > MIN_FRACTIONAL_MOVE {
                vert_mag = vert_mag.trunc() + vert_mag.signum();
            }

            // move "NE" for half and "NW" for half of vert_mag, adding any odd moves to "NE"
            let northeast_magnitude = ((vert_mag as i32) / 2) + ((vert_mag as i32) % 2);
            let northwest_magnitude = (vert_mag as i32) / 2;

            // Set movement
            new_x += northeast_magnitude;
            new_y += northwest_magnitude;
            new_z -= northeast_magnitude + northwest_magnitude;
            ci_log!(logger, logger::FilterLevel::Trace, "move_ne by: {}", northeast_magnitude);
            ci_log!(logger, logger::FilterLevel::Trace, "move_nw by: {}", northwest_magnitude);
        }

        //OPT: *DESIGN* This is a dumb way to sanity-check
        // Sanity check
        match Coords::new(new_x, new_y, new_z, ctx) {
            Ok(_coords) => {
                self.x = new_x;
                self.y = new_y;
                self.z = new_z;

                ci_log!(logger, logger::FilterLevel::Trace, "END coord.move_vec()");
                Ok(())
            }
            Err(e)  => {
                ci_log!(logger, logger::FilterLevel::Trace, "FAILED coord.move_vec()");
                Err(e)
            }
        }
    }

    pub fn move_one(&mut self, dir: HexSides, logger: &logger::Instance, ctx: &Context) -> Result<(), ValidityError> {
        ci_log!(logger, logger::FilterLevel::Debug, "Attempting to move item at {:?} 1 cell {:?}", self, dir);
        
        //OPT: *DESIGN* Do some smart math here probably
        // Decompose movement into x, y, and z components
        let (x_delta, y_delta, z_delta) = match dir {
            HexSides::NORTHEAST =>  (1, 0, -1),
            HexSides::NORTH     =>  (0, 1, -1),
            HexSides::NORTHWEST =>  (-1, 1, 0),
            HexSides::SOUTHWEST =>  (-1, 0, 1),
            HexSides::SOUTH     =>  (0, -1, 1),
            HexSides::SOUTHEAST =>  (1, -1, 0)
        };

        //OPT: *DESIGN* This is a dumb way to sanity-check
        // Sanity check
        match Coords::new(self.x + x_delta, self.y + y_delta, self.z + z_delta, ctx) {
            Ok(_coords) => {
                self.x += x_delta;
                self.y += y_delta;
                self.z += z_delta;

                ci_log!(logger, logger::FilterLevel::Debug, "Item successfully moved 1 cell {:?} to {:?}", dir, self);
                Ok(())
            }
            Err(e)  => {
                ci_log!(logger, logger::FilterLevel::Debug, "Failed to move item at {:?} 1 cell {:?}", self, dir);
                Err(e)
            }
        }

    }

    ///
    // Accessor Methods
    ///

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


///////////////////////////////////////////////////////////////////////////////
//  Trait Implementations
///////////////////////////////////////////////////////////////////////////////

///
// Coords
///
impl fmt::Debug for Coords {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Coords: {{X: {} Y: {} Z: {}}}", self.x, self.y, self.z)
    }
}
impl fmt::Display for Coords {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{},{})", self.x, self.y, self.z)
    }
}
impl Add for Coords {
    type Output = Self;
    
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}
impl Sub for Coords {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

///
// ValidityError
///
impl fmt::Display for ValidityError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid Coordinates. Sum must equal 0.")
    }
}
impl Error for ValidityError {}

//OPT: *STYLE* Should be more general
///
// ValidityError
///
impl fmt::Display for ParamError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid Param. dist_from_edge >= ctx.grid_radius")
    }
}
impl Error for ParamError {}


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
        // Create a default game context and logger for the test
        let test_ctx = Context::default();
        let logger = logger::Instance::default();

        // Initialize values
        let mut actual = Coords::default();
        let mut expected = Coords::default();

        // Initial position
        assert_eq!(actual, expected);

        // Move 3 cells East
        actual.move_vec(3, EAST, &logger, &test_ctx).unwrap();
        if let Ok(expected) = Coords::new(3, -3, 0, &test_ctx) {
            assert_eq!(actual, expected);
        }

        // Move 4 cells North
        actual.move_vec(4, NORTH, &logger, &test_ctx).unwrap();
        if let Ok(expected) = Coords::new(5, -1, -4, &test_ctx) {
            assert_eq!(actual, expected);
        }

        // Move 6 cells West
        actual.move_vec(6, WEST, &logger, &test_ctx).unwrap();
        if let Ok(expected) = Coords::new(-1, 5, -4, &test_ctx) {
            assert_eq!(actual, expected);
        }

        // Move 4 cells South
        actual.move_vec(4, SOUTH, &logger, &test_ctx).unwrap();
        if let Ok(expected) = Coords::new(-3, 3, 0, &test_ctx) {
            assert_eq!(actual, expected);
        }

        // Move 3 cells East
        actual.move_vec(3, EAST, &logger, &test_ctx).unwrap();
        expected = Coords::default();
        assert_eq!(actual, expected);
    }

    #[test]
    fn move_max_cardinal_dirs() {
        // Create a default game context and logger for the test
        let test_ctx = Context::default();
        let logger = logger::Instance::default();

        // Initialize values
        let mut actual = Coords::default();

        // Move East by INT_MAX
        actual.move_vec(i32::max_value(), EAST, &logger, &test_ctx).unwrap();

        // Move one more unit East
        actual.move_vec(1, EAST, &logger, &test_ctx).unwrap();
    }
}