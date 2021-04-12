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

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Side {
    NorthEast,
    North,
    NorthWest,
    SouthWest,
    South,
    SouthEast,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Vertex {
    East,
    NorthEast,
    NorthWest,
    West,
    SouthWest,
    SouthEast,
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
            Self::NorthEast   => (Vertex::East,        Vertex::NorthEast),
            Self::North       => (Vertex::NorthEast,   Vertex::NorthWest),
            Self::NorthWest   => (Vertex::NorthWest,   Vertex::West),
            Self::SouthWest   => (Vertex::West,        Vertex::SouthWest),
            Self::South       => (Vertex::SouthWest,   Vertex::SouthEast),
            Self::SouthEast   => (Vertex::SouthEast,   Vertex::East),
        }
    }
}

impl Vertex {
    /// Returns the adjacent sides as a tuple in counter-clockwise order
    pub fn get_adjacent_sides(vertex: Self) -> (Side, Side) {
        match vertex {
            Self::East      => (Side::SouthEast,    Side::NorthEast),
            Self::NorthEast => (Side::NorthEast,    Side::North),
            Self::NorthWest => (Side::North,        Side::NorthWest),
            Self::West      => (Side::NorthWest,    Side::SouthWest),
            Self::SouthWest => (Side::SouthWest,    Side::South),
            Self::SouthEast => (Side::South,        Side::SouthEast),
        }
    }
}


///////////////////////////////////////////////////////////////////////////////
//  Trait Implementations
///////////////////////////////////////////////////////////////////////////////

/*  *  *  *  *  *  *  *\
 *      Provider      *
\*  *  *  *  *  *  *  */
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


/*  *  *  *  *  *  *  *\
 *        Side        *
\*  *  *  *  *  *  *  */
impl HexDirection for Side {}
impl From<Side> for f32 {
    fn from(src: Side) -> f32 {
        match src {
            Side::NorthEast    => PI/6.0,
            Side::North        => PI/2.0,
            Side::NorthWest    => 5.0*PI/6.0,
            Side::SouthWest    => 7.0*PI/6.0,
            Side::South        => 3.0*PI/2.0,
            Side::SouthEast    => 11.0*PI/6.0
        }
    }
}
impl From<f32> for Side {
    fn from(src: f32) -> Self {
        // Clamp value to 2*PI before comparison
        let clamped_val = src % (2.0*PI);

        match clamped_val {
            x if x < PI/3.0         => Side::NorthEast,
            x if x < 2.0*PI/3.0     => Side::North,
            x if x < PI             => Side::NorthWest,
            x if x < 4.0*PI/3.0     => Side::SouthWest,
            x if x < 5.0*PI/3.0     => Side::South,
            x if x < 2.0*PI         => Side::SouthEast,
            _ => panic!("Invalid value for f32->Side conversion")
        }
    }
}
impl From<Side> for usize {
    fn from(src: Side) -> usize {
        match src {
            Side::NorthEast    => 0,
            Side::North        => 1,
            Side::NorthWest    => 2,
            Side::SouthWest    => 3,
            Side::South        => 4,
            Side::SouthEast    => 5
        }
    }
}
impl From<usize> for Side {
    fn from(src: usize) -> Self {
        match src {
            0 => Side::NorthEast,
            1 => Side::North,
            2 => Side::NorthWest,
            3 => Side::SouthWest,
            4 => Side::South,
            5 => Side::SouthEast,
            _ => panic!("Invalid value for usize->Side conversion")
        }
    }
}
impl From<coords::Translation> for Side {
    fn from(src: coords::Translation) -> Self {
        match (src.x(), src.y(), src.z()) {
            (1, 0, -1)  => Side::NorthEast,
            (0, 1, -1)  => Side::North,
            (-1, 1, 0)  => Side::NorthWest,
            (-1, 0, 1)  => Side::SouthWest,
            (0, -1, 1)  => Side::South,
            (1, -1, 0)  => Side::SouthEast,
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
        Self::NorthEast
    }
}


/*  *  *  *  *  *  *  *\
 *       Vertex       *
\*  *  *  *  *  *  *  */
impl HexDirection for Vertex {}
impl From<Vertex> for f32 {
    fn from(src: Vertex) -> f32 {
        match src {
            Vertex::East       => 0.0,
            Vertex::NorthEast  => PI/3.0,
            Vertex::NorthWest  => 2.0*PI/3.0,
            Vertex::West       => PI,
            Vertex::SouthWest  => 4.0*PI/3.0,
            Vertex::SouthEast  => 5.0*PI/3.0
        }
    }
}
impl From<f32> for Vertex {
    fn from(src: f32) -> Self {
        // Clamp value to 2*PI before comparison
        let clamped_val = src % (2.0*PI);

        match clamped_val {
            x if x < PI/6.0         => Vertex::East,
            x if x < PI/2.0         => Vertex::NorthEast,
            x if x < 5.0*PI/6.0     => Vertex::NorthWest,
            x if x < 7.0*PI/6.0     => Vertex::West,
            x if x < 3.0*PI/2.0     => Vertex::SouthWest,
            x if x < 11.0*PI/6.0    => Vertex::SouthEast,
            x if x < 2.0*PI         => Vertex::East,
            _ => panic!("Invalid value for Vertex conversion")
        }
    }
}
impl From<Vertex> for usize {
    fn from(src: Vertex) -> usize {
        match src {
            Vertex::East       => 0,
            Vertex::NorthEast  => 1,
            Vertex::NorthWest  => 2,
            Vertex::West       => 3,
            Vertex::SouthWest  => 4,
            Vertex::SouthEast  => 5
        }
    }
}
impl From<usize> for Vertex {
    fn from(src: usize) -> Self {
        match src {
            0 => Vertex::East,
            1 => Vertex::NorthEast,
            2 => Vertex::NorthWest,
            3 => Vertex::West,
            4 => Vertex::SouthWest,
            5 => Vertex::SouthEast,
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
        Self::East
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
            Side::NorthEast,
            Side::North,
            Side::NorthWest,
            Side::SouthWest,
            Side::South,
            Side::SouthEast
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
        side_provider = Provider::new(Side::SouthEast);
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
            Vertex::East,
            Vertex::NorthEast,
            Vertex::NorthWest,
            Vertex::West,
            Vertex::SouthWest,
            Vertex::SouthEast
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
        side_provider = Provider::new(Vertex::SouthEast);
        for j in 0..NUM_HEX_DIRECTIONS {
            let side = side_provider.next().unwrap();
            assert_eq!(Vertex::from(correct_usize[j]), side);
            assert_eq!(Vertex::from(correct_f32[j]), side);
        }
    }
}