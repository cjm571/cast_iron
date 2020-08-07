/* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *\
Filename : environment/weather.rs

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

\* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * */

use crate::{
    environment::element::{
        Element,
        Elemental
    },
    polyfunc::PolyFunc
};

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

// Intensity limits
const NONE_INTENSITY_RANGE_MIN: i32     = 0;
const NONE_INTENSITY_RANGE_MAX: i32     = 63;
const MILD_INTENSITY_RANGE_MIN: i32     = 64;
const MILD_INTENSITY_RANGE_MAX: i32     = 127;
const STRONG_INTENSITY_RANGE_MIN: i32   = 128;
const STRONG_INTENSITY_RANGE_MAX: i32   = 191;
const SEVERE_INTENSITY_RANGE_MIN: i32   = 192;
const SEVERE_INTENSITY_RANGE_MAX: i32   = 255;
const MAX_INTENSITY: i32                = 256;


///////////////////////////////////////////////////////////////////////////////
//  Data Structures
///////////////////////////////////////////////////////////////////////////////

#[derive(Default)]
pub struct Weather {
    element:    Element,
    function:   PolyFunc,
}

#[derive(Debug, PartialEq)]
pub enum Intensity {
    None,
    Mild,
    Strong,
    Severe,
    Max
}


///////////////////////////////////////////////////////////////////////////////
//  Object Implementation
///////////////////////////////////////////////////////////////////////////////

impl Weather {
    /// Fully-qualified constructor
    pub fn new(element: Element, function: PolyFunc) -> Self {
        Self {
            element:    element,
            function:   function,
        }
    }


    ///
    // Mutator Methods
    ///

    // Changes the kind of weather to the given Element
    pub fn change(&mut self, element: Element) {
        self.element = element;
    }


    ///
    // Accessor Methods
    ///

    pub fn get_intensity(&self, tick: usize) -> Intensity {
        Intensity::from(self.function.solve(tick))
    }
}


///////////////////////////////////////////////////////////////////////////////
//  Trait Implementations
///////////////////////////////////////////////////////////////////////////////

///
// Intensity
///
impl Default for Intensity {
    fn default() -> Self {
        Self::None
    }
}
impl From<i32> for Intensity {
    fn from(src: i32) -> Self {
        match src {
            std::i32::MIN               ..= -1                          => Intensity::None,
            NONE_INTENSITY_RANGE_MIN    ..= NONE_INTENSITY_RANGE_MAX    => Intensity::None,
            MILD_INTENSITY_RANGE_MIN    ..= MILD_INTENSITY_RANGE_MAX    => Intensity::Mild,
            STRONG_INTENSITY_RANGE_MIN  ..= STRONG_INTENSITY_RANGE_MAX  => Intensity::Strong,
            SEVERE_INTENSITY_RANGE_MIN  ..= SEVERE_INTENSITY_RANGE_MAX  => Intensity::Severe,
            MAX_INTENSITY               ..= std::i32::MAX               => Intensity::Max
        }
    }
}

///
// Weather
///
// Distribution trait provides randomnization for this module
impl Distribution<Weather> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Weather {
        let rand_elem: Element = rng.gen();
        let rand_polyfunc: PolyFunc = rng.gen();

        Weather {
            element:  rand_elem,
            function: rand_polyfunc
        }
    }
}
impl Elemental for Weather {
    fn get_element(&self) -> Element {
        self.element
    }
}
