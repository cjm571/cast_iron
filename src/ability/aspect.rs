/* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *\
Filename : ability\aspect.rs

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
    This module defines the aspects (i.e characteristics) of CastIron abilities.

    All aspects are implemented as enumerations, as mixed-aspect abilities are not
    planned at the time of this writing.

\* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * */

use std::fmt;

use crate::{
    context::Context,
    element::Element,
    Randomizable,
};

use rand::Rng;


///////////////////////////////////////////////////////////////////////////////
//  Named Constants
///////////////////////////////////////////////////////////////////////////////

// Difference between numerical and ASCII value of a number character
const ASCII_TO_VAL_CONVERSION_VAL: usize = 48;

//OPT: *DESIGN* Could use an explicit declaration of enum min/maxes


///////////////////////////////////////////////////////////////////////////////
//  Data Structures
///////////////////////////////////////////////////////////////////////////////

// Enumeration of the aesthetics (coolness) of an ability
#[derive(Debug, Copy, Clone)]
pub enum Aesthetics {
    Unset       = 0,
    Beautiful   = 1,
    Impressive  = 2,
    Erotic      = 3,
    Ugly        = 4,
    Subtle      = 5,
}

// Enumeration of method by which an ability is performed
#[derive(Debug, Copy, Clone)]
pub enum Method {
    Unset       = 0,
    Staff       = 1,
    Wand        = 2,
    Manual      = 3,
    Vocal       = 4,
}

// Enumeration of morality aspect of an ability
#[derive(Debug, Copy, Clone)]
pub enum Morality {
    Unset       = 0,
    Good        = 1,
    Neutral     = 2,
    Evil        = 3,
}

// Enumeration of all schools of an ability
#[derive(Debug, Copy, Clone)]
pub enum School {
    Unset       = 0,
    Destruction = 1,
    Restoration = 2,
    Conjuration = 3,
    Alteration  = 4,
    Illusion    = 5,
    Nature      = 6,
    Song        = 7,
}

// Structure containing all aspect classifications
#[derive(Default)]
pub struct Aspects {
    pub aesthetics: Aesthetics,
    pub element:    Element,
    pub method:     Method,
    pub morality:   Morality,
    pub school:     School,
}


///////////////////////////////////////////////////////////////////////////////
//  Object Implementation
///////////////////////////////////////////////////////////////////////////////

impl Aspects {
    /// Fully-qualified constructor
    pub fn new(aesthetics: Aesthetics,
               element:    Element,
               method:     Method,
               morality:   Morality,
               school:     School) -> Self {
        Self {
            aesthetics,
            element,
            method,
            morality,
            school,
        }
    }
}


///////////////////////////////////////////////////////////////////////////////
//  Trait Implementations
///////////////////////////////////////////////////////////////////////////////

///
// Aesthetics
///
impl Default for Aesthetics {
    fn default() -> Self {
        Aesthetics::Unset
    }
}
impl From<usize> for Aesthetics {
    fn from(src: usize) -> Self {
        match src {
            0 => Aesthetics::Unset,
            1 => Aesthetics::Beautiful,
            2 => Aesthetics::Impressive,
            3 => Aesthetics::Erotic,
            4 => Aesthetics::Ugly,
            5 => Aesthetics::Subtle,
            _ => panic!("aspect::Aesthetics::from: Aspect value out of range")
        }
    }
}

///
// Method
///
impl Default for Method {
    fn default() -> Self {
        Method::Unset
    }
}
impl From<usize> for Method {
    fn from(src: usize) -> Self {
        match src {
            0 => Method::Unset,
            1 => Method::Staff,
            2 => Method::Wand,
            3 => Method::Manual,
            4 => Method::Vocal,
            _ => panic!("aspect::Method::from: Aspect value out of range")
        }
    }
}

///
// Morality
///
impl Default for Morality {
    fn default() -> Self {
        Morality::Unset
    }
}
impl From<usize> for Morality {
    fn from(src: usize) -> Self {
        match src {
            0 => Morality::Unset,
            1 => Morality::Good,
            2 => Morality::Neutral,
            3 => Morality::Evil,
            _ => panic!("aspect::Morality::from: Aspect value out of range")
        }
    }
}

///
// School
///
impl Default for School {
    fn default() -> Self {
        School::Unset
    }
}
impl From<usize> for School {
    fn from(src: usize) -> Self {
        match src {
            0 => School::Unset,
            1 => School::Destruction,
            2 => School::Restoration,
            3 => School::Conjuration,
            4 => School::Alteration,
            5 => School::Illusion,
            6 => School::Nature,
            7 => School::Song,
            _ => panic!("aspect::School::from: Aspect value out of range")
        }
    }
}

///
// Aspects
///
impl From<&String> for Aspects {
    fn from(src: &String) -> Self {
        let mut data_chars = src.chars();

        // Subtract conversion value to extract int value from ascii value
        Self {
            aesthetics: Aesthetics::from(data_chars.next().unwrap() as usize - ASCII_TO_VAL_CONVERSION_VAL),
            element:    Element::from(data_chars.next().unwrap() as usize - ASCII_TO_VAL_CONVERSION_VAL),
            method:     Method::from(data_chars.next().unwrap() as usize - ASCII_TO_VAL_CONVERSION_VAL),
            morality:   Morality::from(data_chars.next().unwrap() as usize - ASCII_TO_VAL_CONVERSION_VAL),
            school:     School::from(data_chars.next().unwrap() as usize - ASCII_TO_VAL_CONVERSION_VAL),
        }
    }
}
impl fmt::Display for Aspects {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.aesthetics as usize)?;
        write!(f, "{}", self.element as usize)?;
        write!(f, "{}", self.method as usize)?;
        write!(f, "{}", self.morality as usize)?;
        write!(f, "{}", self.school as usize)
    }
}
impl fmt::Debug for Aspects {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Aspects: {{{:?}, {:?}, {:?}, {:?}, {:?}}}", self.aesthetics, self.element, self.method, self.morality, self.school)
    }
}
impl Randomizable for Aspects {
    fn rand(_ctx: &Context) -> Self {
        // Start randomization thread
        let mut rng = rand::thread_rng();
        
        //OPT: *STYLE* magic numbers! and these should just be rand() calls
        let aesthetics  = Aesthetics::from(rng.gen_range(0, 6) as usize);
        let element     = Element::from(rng.gen_range(1, 9) as usize);
        let method      = Method::from(rng.gen_range(0, 5) as usize);
        let morality    = Morality::from(rng.gen_range(0, 4) as usize);
        let school      = School::from(rng.gen_range(0, 8) as usize);

        Self {
            aesthetics,
            element,
            method,
            morality,
            school,
        }
    }
}
