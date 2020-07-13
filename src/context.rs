/* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *\
Filename : context.rs

Copyright (C) 2020 CJ McAllister
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
    Context module for tracking high-level state for the CastIron engine.

\* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * */



///////////////////////////////////////////////////////////////////////////////
//  Constants
///////////////////////////////////////////////////////////////////////////////

/* CastIron Game Defaults */ 
const DEFAULT_GRID_RADIUS:          u8 = 10;
const DEFAULT_MAX_OBSTACLE_LENGTH:  u8 = 5;


///////////////////////////////////////////////////////////////////////////////
//  Data structures
///////////////////////////////////////////////////////////////////////////////

pub struct Context {
    grid_radius:        u8,
    max_obstacle_len:   u8
}


///////////////////////////////////////////////////////////////////////////////
//  Functions and Methods
///////////////////////////////////////////////////////////////////////////////

impl Context {
    /// Default constructor
    pub fn default() -> Self {
        Self {
            grid_radius:        DEFAULT_GRID_RADIUS,
            max_obstacle_len:  DEFAULT_MAX_OBSTACLE_LENGTH
        }
    }

    //OPT: If this ends up having more than 4 params, make a builder class
    /// Generic Constructor
    pub fn new(grid_radius: u8, max_obstacle_len: u8) -> Self {
        Self {
            grid_radius:        grid_radius,
            max_obstacle_len:   max_obstacle_len
        }
    }

    
    ///////////////////////////////////////////////////////////////////////////
    //  Accessor Methods
    ///////////////////////////////////////////////////////////////////////////

    pub fn get_grid_radius(&self) -> u8 {
        self.grid_radius
    }
    
    pub fn get_max_obstacle_len(&self) -> u8 {
        self.max_obstacle_len
    }
}
