/* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *\
Filename : hex_directions.rs

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

use crate::coords;

use rand::{
    Rng,
    distributions::{
        Distribution,
        Standard
    }
};


///////////////////////////////////////////////////////////////////////////////
//  Named Constants
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

//OPT: *STYLE* Derive the HexDirection trait if it has no required methods
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Side {
    NORTHEAST,
    NORTH,
    NORTHWEST,
    SOUTHWEST,
    SOUTH,
    SOUTHEAST,
}

//OPT: *STYLE* Derive the HexDirection trait if it has no required methods
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Vertex {
    EAST,
    NORTHEAST,
    NORTHWEST,
    WEST,
    SOUTHWEST,
    SOUTHEAST,
}


///////////////////////////////////////////////////////////////////////////////
//  Object Implementation
///////////////////////////////////////////////////////////////////////////////

#[derive(Default, Debug)]
pub struct Provider<T: HexDirection > {
    cur_direction:  T,
    idx:            usize
}

impl<T: HexDirection> Provider<T> {
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

impl Side {
    /// Returns the adjacent vertices as a tuple in counter-clockwise order
    pub fn get_adjacent_vertices(side: Self) -> (Vertex, Vertex) {
        match side {
            Self::NORTHEAST   => (Vertex::EAST,        Vertex::NORTHEAST),
            Self::NORTH       => (Vertex::NORTHEAST,   Vertex::NORTHWEST),
            Self::NORTHWEST   => (Vertex::NORTHWEST,   Vertex::WEST),
            Self::SOUTHWEST   => (Vertex::WEST,        Vertex::SOUTHWEST),
            Self::SOUTH       => (Vertex::SOUTHWEST,   Vertex::SOUTHEAST),
            Self::SOUTHEAST   => (Vertex::SOUTHEAST,   Vertex::EAST),
        }
    }
}

impl Vertex {
    /// Returns the adjacent sides as a tuple in counter-clockwise order
    pub fn get_adjacent_sides(vertex: Self) -> (Side, Side) {
        match vertex {
            Self::EAST      => (Side::SOUTHEAST,    Side::NORTHEAST),
            Self::NORTHEAST => (Side::NORTHEAST,    Side::NORTH),
            Self::NORTHWEST => (Side::NORTH,        Side::NORTHWEST),
            Self::WEST      => (Side::NORTHWEST,    Side::SOUTHWEST),
            Self::SOUTHWEST => (Side::SOUTHWEST,    Side::SOUTH),
            Self::SOUTHEAST => (Side::SOUTH,        Side::SOUTHEAST),
        }
    }
}


///////////////////////////////////////////////////////////////////////////////
//  Trait Implementations
///////////////////////////////////////////////////////////////////////////////

///
// Provider
///
impl<T: HexDirection> Iterator for Provider<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        // Increment direction by PI/3 to get the next side/vertex, and iteration idx by 1
        let next_theta: f32 = self.cur_direction.into() + PI/3.0;
        self.idx += 1;

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
impl<T: HexDirection> Distribution<Provider<T>> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Provider<T> {
        let rand_num: f32 = rng.gen();
        let rand_dir: T = T::from(rand_num * (2.0*PI));

        Provider::new(rand_dir)
    }
}


///
// Side
///
impl HexDirection for Side {}
impl From<Side> for f32 {
    fn from(src: Side) -> f32 {
        match src {
            Side::NORTHEAST    => PI/6.0,
            Side::NORTH        => PI/2.0,
            Side::NORTHWEST    => 5.0*PI/6.0,
            Side::SOUTHWEST    => 7.0*PI/6.0,
            Side::SOUTH        => 3.0*PI/2.0,
            Side::SOUTHEAST    => 11.0*PI/6.0
        }
    }
}
impl From<f32> for Side {
    fn from(src: f32) -> Self {
        // Clamp value to 2*PI before comparison
        let clamped_val = src % (2.0*PI);

        match clamped_val {
            x if x < PI/3.0         => Side::NORTHEAST,
            x if x < 2.0*PI/3.0     => Side::NORTH,
            x if x < PI             => Side::NORTHWEST,
            x if x < 4.0*PI/3.0     => Side::SOUTHWEST,
            x if x < 5.0*PI/3.0     => Side::SOUTH,
            x if x < 2.0*PI         => Side::SOUTHEAST,
            _ => panic!("Invalid value for f32->Side conversion")
        }
    }
}
impl From<Side> for usize {
    fn from(src: Side) -> usize {
        match src {
            Side::NORTHEAST    => 0,
            Side::NORTH        => 1,
            Side::NORTHWEST    => 2,
            Side::SOUTHWEST    => 3,
            Side::SOUTH        => 4,
            Side::SOUTHEAST    => 5
        }
    }
}
impl From<usize> for Side {
    fn from(src: usize) -> Self {
        match src {
            0 => Side::NORTHEAST,
            1 => Side::NORTH,
            2 => Side::NORTHWEST,
            3 => Side::SOUTHWEST,
            4 => Side::SOUTH,
            5 => Side::SOUTHEAST,
            _ => panic!("Invalid value for usize->Side conversion")
        }
    }
}
impl From<coords::Translation> for Side {
    fn from(src: coords::Translation) -> Self {
        match (src.x(), src.y(), src.z()) {
            (1, 0, -1)  => Side::NORTHEAST,
            (0, 1, -1)  => Side::NORTH,
            (-1, 1, 0)  => Side::NORTHWEST,
            (-1, 0, 1)  => Side::SOUTHWEST,
            (0, -1, 1)  => Side::SOUTH,
            (1, -1, 0)  => Side::SOUTHEAST,
            _           => panic!("Invalid Coords Translation for conversion to HexSide"),
        }
    }
}
// Distribution trait provides randomization for this module
impl Distribution<Side> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Side {
        let rand_num: f32 = rng.gen();
        Side::from(rand_num)
    }
}
impl Default for Side {
    fn default() -> Self {
        Self::NORTHEAST
    }
}


///
// Vertex
///
impl HexDirection for Vertex {}
impl From<Vertex> for f32 {
    fn from(src: Vertex) -> f32 {
        match src {
            Vertex::EAST       => 0.0,
            Vertex::NORTHEAST  => PI/3.0,
            Vertex::NORTHWEST  => 2.0*PI/3.0,
            Vertex::WEST       => PI,
            Vertex::SOUTHWEST  => 4.0*PI/3.0,
            Vertex::SOUTHEAST  => 5.0*PI/3.0
        }
    }
}
impl From<f32> for Vertex {
    fn from(src: f32) -> Self {
        // Clamp value to 2*PI before comparison
        let clamped_val = src % (2.0*PI);

        match clamped_val {
            x if x < PI/6.0         => Vertex::EAST,
            x if x < PI/2.0         => Vertex::NORTHEAST,
            x if x < 5.0*PI/6.0     => Vertex::NORTHWEST,
            x if x < 7.0*PI/6.0     => Vertex::WEST,
            x if x < 3.0*PI/2.0     => Vertex::SOUTHWEST,
            x if x < 11.0*PI/6.0    => Vertex::SOUTHEAST,
            x if x < 2.0*PI         => Vertex::EAST,
            _ => panic!("Invalid value for Vertex conversion")
        }
    }
}
impl From<Vertex> for usize {
    fn from(src: Vertex) -> usize {
        match src {
            Vertex::EAST       => 0,
            Vertex::NORTHEAST  => 1,
            Vertex::NORTHWEST  => 2,
            Vertex::WEST       => 3,
            Vertex::SOUTHWEST  => 4,
            Vertex::SOUTHEAST  => 5
        }
    }
}
impl From<usize> for Vertex {
    fn from(src: usize) -> Self {
        match src {
            0 => Vertex::EAST,
            1 => Vertex::NORTHEAST,
            2 => Vertex::NORTHWEST,
            3 => Vertex::WEST,
            4 => Vertex::SOUTHWEST,
            5 => Vertex::SOUTHEAST,
            _ => panic!("Invalid value for usize->Vertex conversion")
        }
    }
}
// Distribution trait provides randomization for this module
impl Distribution<Vertex> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Vertex {
        let rand_num: f32 = rng.gen();
        Vertex::from(rand_num)
    }
}
impl Default for Vertex {
    fn default() -> Self {
        Self::EAST
    }
}


///////////////////////////////////////////////////////////////////////////////
//  Unit Tests
///////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hex_sides() {
        // Setup correct value arrays
        let correct_enum: [Side;NUM_HEX_DIRECTIONS] = [
            Side::NORTHEAST,
            Side::NORTH,
            Side::NORTHWEST,
            Side::SOUTHWEST,
            Side::SOUTH,
            Side::SOUTHEAST
        ];
        let correct_usize: [usize;NUM_HEX_DIRECTIONS] = [
            0,
            1,
            2,
            3,
            4,
            5
        ];
        let correct_f32: [f32;NUM_HEX_DIRECTIONS] = [
            PI/6.0,
            PI/2.0,
            5.0*PI/6.0,
            7.0*PI/6.0,
            3.0*PI/2.0,
            11.0*PI/6.0
        ];


        // Verify default
        let mut side_provider: Provider<Side> = Provider::default();
        assert_eq!(side_provider.cur_direction, correct_enum[0]);
        assert_eq!(usize::from(side_provider.cur_direction), correct_usize[0]);
        assert_eq!(f32::from(side_provider.cur_direction), correct_f32[0]);

        let mut i = 1;
        for hex_side in side_provider {
            assert_eq!(hex_side, correct_enum[i]);
            assert_eq!(usize::from(hex_side), correct_usize[i]);
            assert_eq!(f32::from(hex_side), correct_f32[i]);
            i = (i + 1) % NUM_HEX_DIRECTIONS;
        }

        // Verify we iterated through all 6 sides
        assert_eq!(i, 1);

        // Verify conversions
        side_provider = Provider::new(Side::SOUTHEAST);
        for j in 0..NUM_HEX_DIRECTIONS {
            let side = side_provider.next().unwrap();
            assert_eq!(Side::from(correct_usize[j]), side);
            assert_eq!(Side::from(correct_f32[j]), side);
        }
    }
    

    #[test]
    fn hex_vertices() {
        // Setup correct value arrays
        let correct_enum: [Vertex;NUM_HEX_DIRECTIONS] = [
            Vertex::EAST,
            Vertex::NORTHEAST,
            Vertex::NORTHWEST,
            Vertex::WEST,
            Vertex::SOUTHWEST,
            Vertex::SOUTHEAST
        ];
        let correct_usize: [usize;NUM_HEX_DIRECTIONS] = [
            0,
            1,
            2,
            3,
            4,
            5
        ];
        let correct_f32: [f32;NUM_HEX_DIRECTIONS] = [
            0.0,
            PI/3.0,
            2.0*PI/3.0,
            PI,
            4.0*PI/3.0,
            5.0*PI/3.0
        ];


        // Verify default
        let mut side_provider: Provider<Vertex> = Provider::default();
        assert_eq!(side_provider.cur_direction, correct_enum[0]);
        assert_eq!(usize::from(side_provider.cur_direction), correct_usize[0]);
        assert_eq!(f32::from(side_provider.cur_direction), correct_f32[0]);

        let mut i = 1;
        for hex_side in side_provider {
            assert_eq!(hex_side, correct_enum[i]);
            assert_eq!(usize::from(hex_side), correct_usize[i]);
            assert_eq!(f32::from(hex_side), correct_f32[i]);
            i = (i + 1) % NUM_HEX_DIRECTIONS;
        }

        // Verify we iterated through all 6 sides
        assert_eq!(i, 1);

        // Verify conversions
        side_provider = Provider::new(Vertex::SOUTHEAST);
        for j in 0..NUM_HEX_DIRECTIONS {
            let side = side_provider.next().unwrap();
            assert_eq!(Vertex::from(correct_usize[j]), side);
            assert_eq!(Vertex::from(correct_f32[j]), side);
        }
    }
}