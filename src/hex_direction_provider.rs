/* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *\
Filename : hex_direction_provider.rs

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
    This module will provide data structures and helper functions for
    convenience in working with the hex grid

\* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * */

use std::f32::consts::PI;

use rand::{
    Rng,
    distributions::{
        Distribution,
        Standard
    }
};


///////////////////////////////////////////////////////////////////////////////
//  Constants
///////////////////////////////////////////////////////////////////////////////

const NUM_HEX_DIRECTIONS: usize = 6;


///////////////////////////////////////////////////////////////////////////////
//  Trait Definitions
///////////////////////////////////////////////////////////////////////////////

pub trait HexDirection:
    Copy + Clone + From<f32> + Into<f32> + Eq + PartialEq {
    fn count() -> usize {
        NUM_HEX_DIRECTIONS
    }
}


///////////////////////////////////////////////////////////////////////////////
//  Data structures
///////////////////////////////////////////////////////////////////////////////

//OPT: Derive the HexDirection trait if it has no required methods
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum HexSides {
    NORTHEAST,
    NORTH,
    NORTHWEST,
    SOUTHWEST,
    SOUTH,
    SOUTHEAST
}

//OPT: Derive the HexDirection trait if it has no required methods
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum HexVertices {
    EAST,
    NORTHEAST,
    NORTHWEST,
    WEST,
    SOUTHWEST,
    SOUTHEAST
}


///////////////////////////////////////////////////////////////////////////////
//  Class Implementation
///////////////////////////////////////////////////////////////////////////////

pub struct HexDirectionProvider<T: HexDirection > {
    cur_direction:  T,
    idx:            usize
}

impl<T: HexDirection> HexDirectionProvider<T> {
    pub fn new(start_direction: T) -> Self {
        Self {
            cur_direction:  start_direction,
            idx:            0
        }
    }

    pub fn count(&self) -> usize {
        T::count()
    }
}



///////////////////////////////////////////////////////////////////////////////
//  Trait Implementations
///////////////////////////////////////////////////////////////////////////////

///
// HexDirectionProvider
///

impl<T: HexDirection> Iterator for HexDirectionProvider<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        // Increment direction by PI/3 to get the next side/vertex, and iteration idx by 1
        let next_theta: f32 = self.cur_direction.into() + PI/3.0;
        self.idx = self.idx + 1;

        // Convert the new direction back into a side/vertex
        self.cur_direction = T::from(next_theta);

        // If we're back at the start, stop iterating
        if self.idx > T::count() {
            None
        }
        else {
            Some(self.cur_direction)
        }

    }
}

impl<T: HexDirection> Distribution<HexDirectionProvider<T>> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> HexDirectionProvider<T> {
        let rand_num: f32 = rng.gen();
        let rand_dir: T = T::from(rand_num);

        HexDirectionProvider::new(rand_dir)
    }
}

///
// HexSides
///

impl HexDirection for HexSides {}

impl From<HexSides> for f32 {
    fn from(src: HexSides) -> f32 {
        match src {
            HexSides::NORTHEAST    => PI/6.0,
            HexSides::NORTH        => PI/2.0,
            HexSides::NORTHWEST    => 5.0*PI/6.0,
            HexSides::SOUTHWEST    => 7.0*PI/6.0,
            HexSides::SOUTH        => 3.0*PI/2.0,
            HexSides::SOUTHEAST    => 11.0*PI/6.0
        }
    }
}
impl From<f32> for HexSides {
    fn from(src: f32) -> Self {
        // Clamp value to 2*PI before comparison
        let clamped_val = src % (2.0*PI);

        match clamped_val {
            x if x < PI/3.0         => HexSides::NORTHEAST,
            x if x < 2.0*PI/3.0     => HexSides::NORTH,
            x if x < PI             => HexSides::NORTHWEST,
            x if x < 4.0*PI/3.0     => HexSides::SOUTHWEST,
            x if x < 5.0*PI/3.0     => HexSides::SOUTH,
            x if x < 2.0*PI         => HexSides::SOUTHEAST,
            _ => panic!("Invalid value for f32->HexSides conversion")
        }
    }
}

impl From<HexSides> for usize {
    fn from(src: HexSides) -> usize {
        match src {
            HexSides::NORTHEAST    => 0,
            HexSides::NORTH        => 1,
            HexSides::NORTHWEST    => 2,
            HexSides::SOUTHWEST    => 3,
            HexSides::SOUTH        => 4,
            HexSides::SOUTHEAST    => 5
        }
    }
}
impl From<usize> for HexSides {
    fn from(src: usize) -> Self {
        match src {
            0 => HexSides::NORTHEAST,
            1 => HexSides::NORTH,
            2 => HexSides::NORTHWEST,
            3 => HexSides::SOUTHWEST,
            4 => HexSides::SOUTH,
            5 => HexSides::SOUTHEAST,
            _ => panic!("Invalid value for usize->HexSides conversion")
        }
    }
}


// Distribution trait provides randomization for this module
impl Distribution<HexSides> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> HexSides {
        let rand_num: f32 = rng.gen();
        HexSides::from(rand_num)
    }
}


///
// HexVertices
///

impl HexDirection for HexVertices {}

impl From<HexVertices> for f32 {
    fn from(src: HexVertices) -> f32 {
        match src {
            HexVertices::EAST       => 0.0,
            HexVertices::NORTHEAST  => PI/3.0,
            HexVertices::NORTHWEST  => 2.0*PI/3.0,
            HexVertices::WEST       => PI,
            HexVertices::SOUTHWEST  => 4.0*PI/3.0,
            HexVertices::SOUTHEAST  => 5.0*PI/3.0
        }
    }
}
impl From<f32> for HexVertices {
    fn from(src: f32) -> Self {
        // Clamp value to 2*PI before comparison
        let clamped_val = src % (2.0*PI);

        match clamped_val {
            x if x < PI/6.0         => HexVertices::EAST,
            x if x < PI/2.0         => HexVertices::NORTHEAST,
            x if x < 5.0*PI/6.0     => HexVertices::NORTHWEST,
            x if x < 7.0*PI/6.0     => HexVertices::WEST,
            x if x < 3.0*PI/2.0     => HexVertices::SOUTHWEST,
            x if x < 11.0*PI/6.0    => HexVertices::SOUTHEAST,
            x if x < 2.0*PI         => HexVertices::EAST,
            _ => panic!("Invalid value for HexVertices conversion")
        }
    }
}

impl From<HexVertices> for usize {
    fn from(src: HexVertices) -> usize {
        match src {
            HexVertices::EAST       => 0,
            HexVertices::NORTHEAST  => 1,
            HexVertices::NORTHWEST  => 2,
            HexVertices::WEST       => 3,
            HexVertices::SOUTHWEST  => 4,
            HexVertices::SOUTHEAST  => 5
        }
    }
}
impl From<usize> for HexVertices {
    fn from(src: usize) -> Self {
        match src {
            0 => HexVertices::EAST,
            1 => HexVertices::NORTHEAST,
            2 => HexVertices::NORTHWEST,
            3 => HexVertices::WEST,
            4 => HexVertices::SOUTHWEST,
            5 => HexVertices::SOUTHEAST,
            _ => panic!("Invalid value for usize->HexVertices conversion")
        }
    }
}


// Distribution trait provides randomization for this module
impl Distribution<HexVertices> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> HexVertices {
        let rand_num: f32 = rng.gen();
        HexVertices::from(rand_num)
    }
}
