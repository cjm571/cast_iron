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

use rand::Rng;


///////////////////////////////////////////////////////////////////////////////
//  Named Constants
///////////////////////////////////////////////////////////////////////////////

// Intensity limits
const NONE_INTENSITY_RANGE_MIN:     i32 = 0;
const NONE_INTENSITY_RANGE_MAX:     i32 = 63;
const MILD_INTENSITY_RANGE_MIN:     i32 = 64;
const MILD_INTENSITY_RANGE_MAX:     i32 = 127;
const STRONG_INTENSITY_RANGE_MIN:   i32 = 128;
const STRONG_INTENSITY_RANGE_MAX:   i32 = 191;
const SEVERE_INTENSITY_RANGE_MIN:   i32 = 192;
const SEVERE_INTENSITY_RANGE_MAX:   i32 = 255;

//OPT: *DESIGN* This should be configurable, and also need to consider if we're tied to framerate
/// Maximum duration for a weather event
pub const MAX_DURATION:     usize   = 10_000;
/// Maximum intensity of a weather event
pub const MAX_INTENSITY:    i32     = 256;


///////////////////////////////////////////////////////////////////////////////
//  Data Structures
///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Default)]
pub struct Event {
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
//  Object Implementations
///////////////////////////////////////////////////////////////////////////////

impl Event {
    /// Fully-qualified constructor. You probably don't want to use this.
    pub fn new(element: Element, function: PolyFunc) -> Self {
        Self {element, function}
    }

    /// Generate a random weather effect at the given tick
    pub fn rand_starting_at(tick: usize) -> Self {
        let mut rng = rand::thread_rng();

        let element: Element = rng.gen();
        let function = PolyFunc::rand_constrained(MAX_INTENSITY as usize, MAX_DURATION, tick);

        Self {element, function}
    }


    ///
    // Mutator Methods
    ///

    /// Changes the kind of weather to the given Element
    pub fn change(&mut self, element: Element) {
        self.element = element;
    }


    ///
    // Accessor Methods
    ///

    pub fn intensity(&self, tick: usize) -> Intensity {
        Intensity::from(self.function.solve(tick))
    }

    pub fn intensity_exact(&self, tick: usize) -> i32 {
        self.function.solve(tick)
    }

    pub fn duration(&self) -> usize {
        self.function.duration()
    }
}

impl Intensity {
    /// Provides the appropriate alpha level for the weather's intensity
    pub fn to_alpha(&self) -> f32 {
        match self {
            Intensity::None     => 0.000,
            Intensity::Mild     => 0.250,
            Intensity::Strong   => 0.500,
            Intensity::Severe   => 0.750,
            Intensity::Max      => 1.000,
        }
    }
}


///////////////////////////////////////////////////////////////////////////////
//  Trait Implementations
///////////////////////////////////////////////////////////////////////////////

/* Intensity */
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
impl From<Intensity> for String {
    fn from(src: Intensity) -> Self {
        match src {
            Intensity::None     => Self::from("None"),
            Intensity::Mild     => Self::from("Mild"),
            Intensity::Strong   => Self::from("Strong"),
            Intensity::Severe   => Self::from("Severe"),
            Intensity::Max      => Self::from("Max"),
        }
    }
}


/* Event */
impl Elemental for Event {
    fn element(&self) -> Element {
        self.element
    }
}
