/* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *\
Filename : environment\mod.rs

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
    Defines the structures and interactions that make up the environment
    in which the actors perform their actions. Provides functions pertaining
    to both immediate and atmospheric conditions.

Changelog:

\* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * */

pub mod coords;
pub mod resource;
pub mod weather;

///////////////////////////////////////////////////////////////////////////////
//  Data Structures
///////////////////////////////////////////////////////////////////////////////

// Enumeration of all element types
#[allow(dead_code)]
#[derive(Debug)]
pub enum Element {
    Unset       = 0,
    Fire        = 1,
    Ice         = 2,
    Wind        = 3,
    Water       = 4,
    Electric    = 5,
    Earth       = 6,
    Light       = 7,
    Dark        = 8,
}

// Defines the global characteristics and enumerates the objects present within
// the game environment
pub struct Environment {
    size:       u32, // world size, as a radius measured in hexgrid units
    weather:    weather::Weather,
}


///////////////////////////////////////////////////////////////////////////////
//  Functions and Methods
///////////////////////////////////////////////////////////////////////////////

impl Environment {

    // Creates and returns a new Environment object
    pub fn new() -> Environment {
        Environment {
            size:       0,
            weather:    weather::Weather::new(),
        }
    }

    ///////////////////////////////////////////////////////////////////////////
    //  Mutator Methods
    ///////////////////////////////////////////////////////////////////////////

    pub fn set_size(&mut self, _size: u32) {
        self.size = _size;
    }

    pub fn change_weather(&mut self, _weather: weather::Weather) {
        self.weather = _weather
    }

    ///////////////////////////////////////////////////////////////////////////
    //  Accessor Methods
    ///////////////////////////////////////////////////////////////////////////

    pub fn size(self) -> u32 {
        self.size
    }

    pub fn weather(self) -> weather::Weather {
        self.weather
    }
}