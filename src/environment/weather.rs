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

///////////////////////////////////////////////////////////////////////////////
//  Data Structures
///////////////////////////////////////////////////////////////////////////////

pub struct Weather {
    kind:       Element,
    intensity:  Intensity,
}

//TODO: Cool feature: Intensity as a polynomial function of time?
pub enum Intensity {
    None    = 0,
    Mild    = 1,
    Weak    = 2,
    Medium  = 3,
    Strong  = 4,
    Severe  = 5,
}


///////////////////////////////////////////////////////////////////////////////
//  Functions and Methods
///////////////////////////////////////////////////////////////////////////////

impl Weather {

    // Creates and returns a new Weather object
    pub fn new() -> Weather {
        Weather {
            kind:       Element::Unset,
            intensity:  Intensity::None,
        }
    }

    // Creates and returns a new Weather object from the given parameters
    pub fn from(_kind: Element, _intensity: Intensity) -> Weather {
        Weather {
            kind:       _kind,
            intensity:  _intensity,
        }
    }

    ///////////////////////////////////////////////////////////////////////////
    //  Mutator Methods
    ///////////////////////////////////////////////////////////////////////////
    
    // Changes the kind of weather to the given Element
    pub fn change(&mut self, _kind: Element) {
        self.kind = _kind;
    }

    // Makes the weather more intense, unless it is already Severe
    pub fn intensify(&mut self) {
        match self.intensity {
            Intensity::None     => self.intensity = Intensity::Mild,
            Intensity::Mild     => self.intensity = Intensity::Weak,
            Intensity::Weak     => self.intensity = Intensity::Medium,
            Intensity::Medium   => self.intensity = Intensity::Strong,
            Intensity::Strong   => self.intensity = Intensity::Severe,
            Intensity::Severe   => debug_println!("Attempted to intensify Severe weather!"),
        }
    }

    // Makes the weather less intense, unless it is already None
    pub fn weaken(&mut self) {
        match self.intensity {
            Intensity::None     => debug_println!("Attempted to weaken None weather!"),
            Intensity::Mild     => self.intensity = Intensity::None,
            Intensity::Weak     => self.intensity = Intensity::Mild,
            Intensity::Medium   => self.intensity = Intensity::Weak,
            Intensity::Strong   => self.intensity = Intensity::Medium,
            Intensity::Severe   => self.intensity = Intensity::Strong,
        }
    }

    ///////////////////////////////////////////////////////////////////////////
    //  Accessor Methods
    ///////////////////////////////////////////////////////////////////////////

    pub fn get_kind(self) -> Element {
        self.kind
    }

    pub fn get_intensity(self) -> Intensity {
        self.intensity
    }
}