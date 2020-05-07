/* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *\
Filename : environment\world_grid.rs

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
    This module rovides functions to determine interactions between various objects
    in the world grid.
    
Changelog:

\* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * */

use std::f64::consts::PI;
use std::collections::HashMap;

///////////////////////////////////////////////////////////////////////////////
// Data structures
///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Hash)]
pub enum Direction {
    EAST,
    NORTHEAST,
    NORTH,
    NORTHWEST,
    WEST,
    SOUTHWEST,
    SOUTH,
    SOUTHEAST
}
// Equivalence comparison
impl PartialEq for Direction {
    fn eq(&self, other: &Direction) -> bool {
        self == other
    }
}
impl Eq for Direction {}

lazy_static! {
    pub static ref HEX_SIDES: HashMap<Direction, f64> = {
        let mut m = HashMap::new();

        m.insert(Direction::NORTHEAST, PI/6.0);
        m.insert(Direction::NORTH,     PI/2.0);
        m.insert(Direction::NORTHWEST, 5.0*PI/6.0);
        m.insert(Direction::SOUTHWEST, 7.0*PI/6.0);
        m.insert(Direction::SOUTH,     3.0*PI/2.0);
        m.insert(Direction::SOUTHEAST, 11.0*PI/6.0);

        m
    };
}

lazy_static! {
    pub static ref HEX_VERTICES: HashMap<Direction, f64> = {
        let mut m = HashMap::new();

        m.insert(Direction::EAST,       0.0);
        m.insert(Direction::NORTHEAST,  PI/3.0);
        m.insert(Direction::NORTHWEST,  2.0*PI/3.0);
        m.insert(Direction::WEST,       PI);
        m.insert(Direction::SOUTHWEST,  4.0*PI/3.0);
        m.insert(Direction::SOUTHEAST,  5.0*PI/3.0);

        m
    };
}

pub struct WorldGrid {
    pub size: u32, // Maximum value for an axis of the hex grid
}


///////////////////////////////////////////////////////////////////////////////
//  Functions and Methods
///////////////////////////////////////////////////////////////////////////////

impl WorldGrid {
    pub fn new(size: u32) -> WorldGrid {
        WorldGrid {
            size: size,
        }
    }    

    ///////////////////////////////////////////////////////////////////////////
    //  Accessor Methods
    ///////////////////////////////////////////////////////////////////////////
     
    pub fn get_size(self) -> u32 {
        self.size
    }
}