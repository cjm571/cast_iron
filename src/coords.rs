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
    ops::{
        Add,
        Sub,
    },
};

use crate::{
    context::Context,
    hex_directions,
    Randomizable,
};

use rand::Rng;


///////////////////////////////////////////////////////////////////////////////
//  Data structures
///////////////////////////////////////////////////////////////////////////////

#[derive(Default, Copy, Clone, Hash, Eq, PartialEq)]
pub struct Position {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq)]
pub struct Translation {
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
//  Object Implementations
///////////////////////////////////////////////////////////////////////////////

impl Position {
    /// Fully-qualified constructor
    pub fn new(x: i32, y: i32, z: i32, ctx: &Context) -> Result<Self, ValidityError> {
        let pos = Self {x, y, z};

        // Sanity check
        match pos.is_sane(ctx) {
            Ok(()) => Ok(pos),
            Err(e) => Err(e),
        }
    }

    /// Constructs a random, valid Position object within the constraints fo the game Context AND
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

    /// Determines if the given translation is valid
    pub fn can_translate(&self, trans: &Translation, ctx: &Context) -> Result<(), ValidityError> {
        // Simulate the translation and return sanity check result
        (self + trans).is_sane(ctx)
    }

    /// Determines if the given position is a neighbor of this position
    pub fn is_neighbor(&self, other: &Self) -> bool {
        let translation = self - other;

        // A translation magnitude of one means the other position is adjacent to this one
        translation.magnitude() == 1
    }


    /*  *  *  *  *  *  *  *\
     *  Helper Methods    *
    \*  *  *  *  *  *  *  */

    /// Sanity check
    fn is_sane(&self, ctx: &Context) -> Result<(), ValidityError> {
        // Coordinate validity check
        if self.x + self.y + self.z != 0 {
            //TODO: Needs to be a different kind of error than below
            return Err(ValidityError);
        }

        // Bounds check
        if i32::abs(self.x) > ctx.grid_radius() as i32 ||
           i32::abs(self.y) > ctx.grid_radius() as i32 ||
           i32::abs(self.z) > ctx.grid_radius() as i32 {
            return Err(ValidityError)
        }

        Ok(())
    }
}


impl Translation {
    /// Fully-qualified constructor
    pub fn new(x: i32, y: i32, z: i32, ctx: &Context) -> Result<Self, ValidityError> {
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
    fn is_sane(&self, ctx: &Context) -> Result<(), ValidityError> {
        // Coordinate validity check
        if self.x + self.y + self.z != 0 {
            return Err(ValidityError);
        }

        // Bounds check
        if i32::abs(self.x) > ctx.grid_radius() as i32 ||
           i32::abs(self.y) > ctx.grid_radius() as i32 ||
           i32::abs(self.z) > ctx.grid_radius() as i32 {
            return Err(ValidityError)
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
//OPT: *DESIGN* These should sanity-check the result, but that would require access to the context from within these functions...
impl Add<Translation> for Position {
    type Output = Self;

    fn add(self, other: Translation) -> Self::Output {
        Self::Output {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}
impl Add<Translation> for &Position {
    type Output = Position;

    fn add(self, other: Translation) -> Self::Output {
        Self::Output {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}
impl Add<&Translation> for &Position {
    type Output = Position;

    fn add(self, other: &Translation) -> Self::Output {
        Self::Output {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}
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
impl Sub<Translation> for Position {
    type Output = Self;

    fn sub(self, other: Translation) -> Self::Output {
        Self::Output {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}
impl Sub for Position {
    type Output = Translation;

    fn sub(self, other: Self) -> Self::Output {
        Self::Output {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}
impl Sub for &Position {
    type Output = Translation;

    fn sub(self, other: Self) -> Self::Output {
        Self::Output {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
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

///
// ValidityError
///
impl Error for ValidityError {}
impl fmt::Display for ValidityError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid Coordinates. Sum must equal 0.")
    }
}

//OPT: *STYLE* Should be more general
///
// ParamError
///
impl Error for ParamError {}
impl fmt::Display for ParamError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid Param. dist_from_edge >= ctx.grid_radius")
    }
}
