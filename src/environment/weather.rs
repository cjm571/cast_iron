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

    Efefects follow a unique polynomial curve in severity.

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
const MILD_INT: i32 = 63;
const STRONG_INT: i32 = 127;
const SEVERE_INT: i32 = 191;
const MAX_INT: i32 = 255;

#[repr(u8)]
#[derive(Debug)]
pub enum Intensity {
    None    = 0,
    Mild    = MILD_INT as u8,
    Strong  = STRONG_INT as u8,
    Severe  = SEVERE_INT as u8,
    Max     = MAX_INT as u8,
}


///////////////////////////////////////////////////////////////////////////////
//  Functions and Methods
///////////////////////////////////////////////////////////////////////////////

impl Weather {

    // Creates and returns a new Weather object
    pub fn new() -> Weather {
        Weather {
            kind:       Element::Unset,
            function:   PolyFunc::new(),
        }
    }

    // Creates and returns a new Weather object from the given parameters
    pub fn from(kind: Element, function: PolyFunc) -> Weather {
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

    pub fn kind(self) -> Element {
        self.kind
    }

    pub fn intensity(&self, tick: u32) -> Intensity {
        let intensity = self.function.solve(tick);

        match intensity {
            MIN ... -1  =>                  Intensity::None,
            0 ... MILD_INT  =>              Intensity::None,
            MILD_INT ... STRONG_INT  =>     Intensity::Mild,
            STRONG_INT ... SEVERE_INT =>    Intensity::Strong,
            SEVERE_INT ... MAX_INT =>       Intensity::Severe,
            MAX_INT ... MAX =>              Intensity::Max,
            _           =>                  {
                                            debug_println!("Invalid weather intensity!");
                                            Intensity::None
                                            }
        }
    }
}