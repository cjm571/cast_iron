/* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *\
Filename : coords.rs

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
    This module defines the objects for use within the CastIron hexagonal coordinates
    system.

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
    ops::Neg,
};

use crate::{
    context::Context,
    hex_directions,
    Randomizable,
};

use rand::Rng;
use serde::{Serialize, Deserialize};


///////////////////////////////////////////////////////////////////////////////
//  Data structures
///////////////////////////////////////////////////////////////////////////////

#[derive(
    Default,
    Copy, Clone,
    Eq, PartialEq,
    Hash,
    Serialize, Deserialize
)]
pub struct Position {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(
    Default,
    Copy, Clone,
    Eq, PartialEq,
    Hash,
    Serialize, Deserialize
)]
pub struct Translation {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Debug)]
pub enum CoordsError {
    InvalidComponents(i32, i32, i32),
    InvalidParam(String),
    OutOfBounds,
}


///////////////////////////////////////////////////////////////////////////////
//  Object Implementations
///////////////////////////////////////////////////////////////////////////////

impl Position {
    /// Fully-qualified constructor
    pub fn new(x: i32, y: i32, z: i32, ctx: &Context) -> Result<Self, CoordsError> {
        let pos = Self {x, y, z};

        // Sanity check
        match pos.is_sane(ctx) {
            Ok(()) => Ok(pos),
            Err(e) => Err(e),
        }
    }

    /// Constructs a random, valid Position object within the constraints fo the game Context AND
    /// constrained the given number cells away from the edge of the hex grid
    pub fn rand_constrained(ctx: &Context, dist_from_edge: usize) -> Result<Self, CoordsError> {
        // Ensure that the distance from the edge is less than the Context's grid radius
        if dist_from_edge >= ctx.grid_radius() {
            return Err(CoordsError::InvalidParam(String::from("dist_from_edge")))
        }

        let max_dist = (ctx.grid_radius() - dist_from_edge) as i32;

        let mut rng = rand::thread_rng();

        let rand_x: i32 = rng.gen_range(- max_dist, max_dist);
        let calc_rand_y = match rand_x {
            i32::MIN..=-1   => rng.gen_range(0,         rand_x.abs()),  // X is negative, generate a bounded-positive Y
            0               => rng.gen_range(-max_dist, max_dist),      // X is 0, generate an unbounded Y
            1..=i32::MAX    => rng.gen_range(-rand_x,   0)              // X is positive, generate a bounded-negative Y
        };
        let calc_z: i32 = 0 - rand_x - calc_rand_y; // Position must meet the x + y + z == 0 requirement

        Ok(Self::new(rand_x, calc_rand_y, calc_z, ctx).unwrap())
    }


    /*  *  *  *  *  *  *  *\
     *  Accessor Methods  *
    \*  *  *  *  *  *  *  */

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


    /*  *  *  *  *  *  *  *\
     *  Utility Methods   *
    \*  *  *  *  *  *  *  */

    /// Determine the translation required to move from the given position to the current position.
    pub fn delta_from(&self, other: &Self) -> Translation {
        let x_delta = self.x - other.x();
        let y_delta = self.y - other.y();
        let z_delta = self.z - other.z();

        Translation {
            x: x_delta,
            y: y_delta,
            z: z_delta,
        }
    }
    
    /// Determine the translation required to move from the current position to the given position.
    pub fn delta_to(&self, other: &Self) -> Translation {
        let x_delta = other.x() - self.x;
        let y_delta = other.y() - self.y;
        let z_delta = other.z() - self.z;

        Translation {
            x: x_delta,
            y: y_delta,
            z: z_delta,
        }
    }

    /// Attempts to move the position by the given translation, returning an error if the translation
    /// or the resulting position is invalid.
    pub fn translate(&mut self, trans: &Translation, ctx: &Context) -> Result<(), CoordsError>  {
        // Sanity check, propagate on failure
        self.can_translate(trans, ctx)?;

        // Check passed, perform translation
        self.blind_translate(trans);
        Ok(())
    }
    /// Determines if the given position is a neighbor of this position
    pub fn is_neighbor(&self, other: &Self) -> bool {
        let translation = self.delta_from(other);

        // A translation magnitude of one means the other position is adjacent to this one
        translation.magnitude() == 1
    }


    /*  *  *  *  *  *  *  *\
     *  Helper Methods    *
    \*  *  *  *  *  *  *  */

    /// Sanity check.
    fn is_sane(&self, ctx: &Context) -> Result<(), CoordsError> {
        // Coordinate validity check
        if self.x + self.y + self.z != 0 {
            return Err(CoordsError::InvalidComponents(self.x, self.y, self.z));
        }

        // Bounds check
        if i32::abs(self.x) > ctx.grid_radius() as i32 ||
           i32::abs(self.y) > ctx.grid_radius() as i32 ||
           i32::abs(self.z) > ctx.grid_radius() as i32 {
            return Err(CoordsError::OutOfBounds)
        }

        Ok(())
    }

    /// Blindly translates the position without sanity checking.
    fn blind_translate(&mut self, trans: &Translation) {
        self.x += trans.x();
        self.y += trans.y();
        self.z += trans.z();
    }
    
    /// Determines if the given translation is valid
    fn can_translate(&self, trans: &Translation, ctx: &Context) -> Result<(), CoordsError> {
        // Simulate the translation and return sanity check result
        let mut pos_clone = *self;
        pos_clone.blind_translate(trans);
        
        //FEAT: Need to do a global collision check here?
        pos_clone.is_sane(ctx)
    }
}


impl Translation {
    /// Fully-qualified constructor
    pub fn new(x: i32, y: i32, z: i32, ctx: &Context) -> Result<Self, CoordsError> {
        let translation = Self {x, y, z};

        // Sanity check
        match translation.is_sane(ctx) {
            Ok(()) => Ok(translation),
            Err(e) => Err(e),
        }
    }


    /*  *  *  *  *  *  *  *\
     *  Accessor Methods  *
    \*  *  *  *  *  *  *  */

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

    
    /*  *  *  *  *  *  *  *\
     *  Utility Methods   *
    \*  *  *  *  *  *  *  */

    /// Calculates and returns the magnitude of the translation i.e., the minimum number of hops required to accomplish it.
    pub fn magnitude(&self) -> u32 {
        let x_abs_mag = i32::abs(self.x) as u32;
        let y_abs_mag = i32::abs(self.y) as u32;
        let z_abs_mag = i32::abs(self.z) as u32;

        // Return the coord component with the largest absolute value
        if x_abs_mag > y_abs_mag && x_abs_mag > z_abs_mag {
            x_abs_mag
        }
        else if y_abs_mag > z_abs_mag {
            y_abs_mag
        }
        else {
            z_abs_mag
        }
    }

    
    /*  *  *  *  *  *  *  *\
     *  Helper Methods    *
    \*  *  *  *  *  *  *  */

    /// Sanity check
    fn is_sane(&self, ctx: &Context) -> Result<(), CoordsError> {
        // Coordinate validity check
        if self.x + self.y + self.z != 0 {
            return Err(CoordsError::InvalidComponents(self.x, self.y, self.z));
        }

        // Bounds check
        if i32::abs(self.x) > ctx.grid_radius() as i32 ||
           i32::abs(self.y) > ctx.grid_radius() as i32 ||
           i32::abs(self.z) > ctx.grid_radius() as i32 {
            return Err(CoordsError::OutOfBounds)
        }

        Ok(())
    }
}


///////////////////////////////////////////////////////////////////////////////
//  Trait Implementations
///////////////////////////////////////////////////////////////////////////////

/*  *  *  *  *  *  *  *\
 *  Position          *
\*  *  *  *  *  *  *  */
impl fmt::Debug for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Position: {{X: {} Y: {} Z: {}}}", self.x, self.y, self.z)
    }
}
impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{},{})", self.x, self.y, self.z)
    }
}
impl Randomizable for Position {
    fn rand(ctx: &Context) -> Self {
        let max_dist = ctx.grid_radius() as i32;

        let mut rng = rand::thread_rng();

        let rand_x: i32 = rng.gen_range(-max_dist, max_dist);
        let calc_rand_y = match rand_x {
            i32::MIN..=-1   => rng.gen_range(0,         rand_x.abs()),  // X is negative, generate a bounded-positive Y
            0               => rng.gen_range(-max_dist, max_dist),      // X is 0, generate an unbounded Y
            1..=i32::MAX    => rng.gen_range(-rand_x,   0)              // X is positive, generate a bounded-negative Y
        };
        let calc_z: i32 = 0 - rand_x - calc_rand_y; // Position must meet the x + y + z == 0 requirement

        Self::new(rand_x, calc_rand_y, calc_z, ctx).unwrap()
    }
}


/*  *  *  *  *  *  *  *\
 *  Translation       *
\*  *  *  *  *  *  *  */
//OPT: *DESIGN* Would be better if this took an angle and a magnitude (what would the units of magnitude be though?)
impl From<hex_directions::Side> for Translation {
    fn from(src: hex_directions::Side) -> Self {
        match src {
            hex_directions::Side::NORTHEAST =>  Self {x: 1,     y: 0,   z: -1},
            hex_directions::Side::NORTH     =>  Self {x: 0,     y: 1,   z: -1},
            hex_directions::Side::NORTHWEST =>  Self {x: -1,    y: 1,   z: 0},
            hex_directions::Side::SOUTHWEST =>  Self {x: -1,    y: 0,   z: 1},
            hex_directions::Side::SOUTH     =>  Self {x: 0,     y: -1,  z: 1},
            hex_directions::Side::SOUTHEAST =>  Self {x: 1,     y: -1,  z: 0},
        }
    }
}
impl Neg for Translation {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}


/*  *  *  *  *  *  *  *\
 *  CoordsError       *
\*  *  *  *  *  *  *  */
impl Error for CoordsError {}
impl fmt::Display for CoordsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CoordsError::InvalidComponents(x, y, z) => {
                write!(f, "Position components ({}, {}, {}) do not add up to 0.", x, y, z)
            }
            CoordsError::InvalidParam(param_name)   => {
                write!(f, "Invalid Parameter: {}", param_name)
            },
            CoordsError::OutOfBounds                => {
                write!(f, "Position out of bounds")
            }
        }
    }
}
