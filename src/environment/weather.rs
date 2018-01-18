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
    //TODO: purpose writeup for weather

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

#[repr(u8)]
#[derive(Debug)]
pub enum Intensity {
    None    = 0,
    Mild    = 63,
    Strong  = 127,
    Severe  = 191,
    Max     = 255,
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
    pub fn from(_kind: Element, _function: PolyFunc) -> Weather {
        Weather {
            kind:       _kind,
            function:   _function,
        }
    }

    ///////////////////////////////////////////////////////////////////////////
    //  Mutator Methods
    ///////////////////////////////////////////////////////////////////////////
    
    // Changes the kind of weather to the given Element
    pub fn change(&mut self, _kind: Element) {
        self.kind = _kind;
    }

    ///////////////////////////////////////////////////////////////////////////
    //  Accessor Methods
    ///////////////////////////////////////////////////////////////////////////

    pub fn get_kind(self) -> Element {
        self.kind
    }

    pub fn get_intensity(&self, _tick: u32) -> Intensity {
        let intensity = self.function.solve(_tick);

        //TODO: Magic numbers. Ugh.
        match intensity {
            MIN ... -1  => Intensity::None,
            0 ... 62    => Intensity::None,
            63 ... 126  => Intensity::Mild,
            127 ... 190 => Intensity::Strong,
            191 ... 254 => Intensity::Severe,
            255 ... MAX => Intensity::Max,
            _           => {
                            debug_println!("Invalid weather intensity!");
                            Intensity::None
                           }
        }
    }
}