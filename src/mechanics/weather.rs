/* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *\
Filename : mechanics/weather.rs

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

use std::time::Duration;

use crate::{
    context::Context,
    element::{
        Element,
        Elemental,
    },
    polyfunc::PolyFunc,
    Randomizable,
};

use rand::Rng;


///////////////////////////////////////////////////////////////////////////////
//  Named Constants
///////////////////////////////////////////////////////////////////////////////

// Intensity limits
const NONE_INTENSITY_RANGE_MIN:     i64 = 0;
const NONE_INTENSITY_RANGE_MAX:     i64 = 63;
const MILD_INTENSITY_RANGE_MIN:     i64 = 64;
const MILD_INTENSITY_RANGE_MAX:     i64 = 127;
const STRONG_INTENSITY_RANGE_MIN:   i64 = 128;
const STRONG_INTENSITY_RANGE_MAX:   i64 = 191;
const SEVERE_INTENSITY_RANGE_MIN:   i64 = 192;
const SEVERE_INTENSITY_RANGE_MAX:   i64 = 255;


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


    /*  *  *  *  *  *  *  *\
     *  Builder Methods   *
    \*  *  *  *  *  *  *  */
    
    pub fn starting_at(mut self, start_time: Duration) -> Self {
        self.function = self.function.starting_at(start_time.as_secs_f64());
        
        self
    }


    /*  *  *  *  *  *  *  *\
     *  Mutator Methods   *
    \*  *  *  *  *  *  *  */

    /// Changes the kind of weather to the given Element
    pub fn change(&mut self, element: Element) {
        self.element = element;
    }


    /*  *  *  *  *  *  *  *\
     *  Accessor Methods  *
    \*  *  *  *  *  *  *  */

    pub fn intensity(&self, tick: f64) -> Intensity {
        Intensity::from(self.function.solve(tick) as i64)
    }

    pub fn intensity_exact(&self, tick: f64) -> f64 {
        self.function.solve(tick)
    }

    pub fn duration(&self) -> Duration {
        Duration::from_secs_f64(self.function.duration())
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

/*  *  *  *  *  *  *  *\
 *     Intensity      *
\*  *  *  *  *  *  *  */
impl Default for Intensity {
    fn default() -> Self {
        Self::None
    }
}
impl From<i64> for Intensity {
    fn from(src: i64) -> Self {
        match src {
            std::i64::MIN               ..= -1                          => Intensity::None,
            NONE_INTENSITY_RANGE_MIN    ..= NONE_INTENSITY_RANGE_MAX    => Intensity::None,
            MILD_INTENSITY_RANGE_MIN    ..= MILD_INTENSITY_RANGE_MAX    => Intensity::Mild,
            STRONG_INTENSITY_RANGE_MIN  ..= STRONG_INTENSITY_RANGE_MAX  => Intensity::Strong,
            SEVERE_INTENSITY_RANGE_MIN  ..= SEVERE_INTENSITY_RANGE_MAX  => Intensity::Severe,
            _                                                           => Intensity::Max
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


/*  *  *  *  *  *  *  *\
 *       Event        *
\*  *  *  *  *  *  *  */
impl Elemental for Event {
    fn element(&self) -> Element {
        self.element
    }
}
impl Randomizable for Event {
    fn rand(ctx: &Context) -> Self {
        let mut rng = rand::thread_rng();

        let element: Element = rng.gen();
        let function = PolyFunc::rand_constrained(ctx.max_weather_intensity(), ctx.max_weather_duration());

        Self {element, function}
    }
}
