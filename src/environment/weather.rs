/* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *\
Filename : weather.rs

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
    This module defines CastIron weather effects.

    Weather can enhance or impede actors in various ways e.g. reducing visibility
    and lowering accuracy of ranged attacks, enhancing wind-elemental damage, etc.

    Effects follow a defined polynomial curve in severity.

Changelog:

\* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * */

use super::Element;
use ::polyfunc::PolyFunc;
use std::i32::{MIN, MAX};

///////////////////////////////////////////////////////////////////////////////
//  Data Structures
///////////////////////////////////////////////////////////////////////////////

pub struct Weather {
    kind:       Element,
    function:   PolyFunc,
}

// Define intensity limits
const MIN_INT: i32 = 0;
const MAX_NO_INT: i32 = 63;
const MIN_MILD_INT: i32 = 64;
const MAX_MILD_INT: i32 = 127;
const MIN_STRONG_INT: i32 = 128;
const MAX_STRONG_INT: i32 = 191;
const MIN_SEVERE_INT: i32 = 192;
const MAX_SEVERE_INT: i32 = 255;
const MAX_INT: i32 = 255;

#[repr(u8)]
#[derive(Debug, PartialEq)]
pub enum Intensity {
    None    = MIN_INT as u8,
    Mild    = MIN_MILD_INT as u8,
    Strong  = MIN_STRONG_INT as u8,
    Severe  = MIN_SEVERE_INT as u8,
    Max     = MAX_INT as u8,
}


///////////////////////////////////////////////////////////////////////////////
//  Functions and Methods
///////////////////////////////////////////////////////////////////////////////

impl Weather {

    /// Constructor
    /// Creates and returns a new Weather object from the given parameters
    pub fn new(kind: Element, function: PolyFunc) -> Weather {
        Weather {
            kind:       kind,
            function:   function,
        }
    }

    ///////////////////////////////////////////////////////////////////////////
    //  Mutator Methods
    ///////////////////////////////////////////////////////////////////////////
    
    // Changes the kind of weather to the given Element
    pub fn change(&mut self, kind: Element) {
        self.kind = kind;
    }

    ///////////////////////////////////////////////////////////////////////////
    //  Accessor Methods
    ///////////////////////////////////////////////////////////////////////////

    pub fn kind(&self) -> Element {
        self.kind
    }

    pub fn intensity(&self, tick: u32) -> Intensity {
        let intensity = self.function.solve(tick);

        match intensity {
            MIN             ..= -1              => Intensity::None,
            MIN_INT         ..= MAX_NO_INT      => Intensity::None,
            MIN_MILD_INT    ..= MAX_MILD_INT    => Intensity::Mild,
            MIN_STRONG_INT  ..= MAX_STRONG_INT  => Intensity::Strong,
            MIN_SEVERE_INT  ..= MAX_SEVERE_INT  => Intensity::Severe,
            256             ..= MAX             => Intensity::Max
        }
    }
}