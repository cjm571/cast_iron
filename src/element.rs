/* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *\
Filename : element.rs

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
    This package enumerates available elements and provides Utility Methods
    for convenience.

\* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * */

use rand::{
    Rng,
    distributions::{
        Distribution,
        Standard
    }
};


//////////////////////////////////////////////////////////////////////////////
//  Data Structures
//////////////////////////////////////////////////////////////////////////////

// Enumeration of all element types
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Element {
    Unset       = 0,
    Fire        = 1,
    Ice         = 2,
    Wind        = 3,
    Water       = 4,
    Electric    = 5,
    Earth       = 6,
    Light       = 7,
    Dark        = 8
    // NOTE: Do not add elements to the end! Light and Dark kindof naturally fit at
    // the end, and using Dark as a marker for "last valid" is very useful due to
    // the weirdness of Rust's "full-fledged data type" enums
}


///////////////////////////////////////////////////////////////////////////////
//  Trait Declaration
///////////////////////////////////////////////////////////////////////////////

pub trait Elemental {
    fn element(&self) -> Element;
} 


///////////////////////////////////////////////////////////////////////////////
//  Trait Implementations
///////////////////////////////////////////////////////////////////////////////

impl Default for Element {
    fn default() -> Self {
        Self::Unset
    }
}
impl From<Element> for String {
    fn from(src: Element) -> Self {
        match src {
            Element::Unset      => String::from("Unset"),
            Element::Fire       => String::from("Fire"),
            Element::Ice        => String::from("Ice"),
            Element::Wind       => String::from("Wind"),
            Element::Water      => String::from("Water"),
            Element::Electric   => String::from("Electric"),
            Element::Earth      => String::from("Earth"),
            Element::Light      => String::from("Light"),
            Element::Dark       => String::from("Dark"),
        }
    }
}
impl From<usize> for Element {
    fn from(src: usize) -> Self {
        match src {
            1 => Element::Fire,
            2 => Element::Ice,
            3 => Element::Wind,
            4 => Element::Water,
            5 => Element::Electric,
            6 => Element::Earth,
            7 => Element::Light,
            8 => Element::Dark,
            _ => panic!("environment::Element::from: Element value ({}) out of range", src)
        }
    }
}

// Distribution trait provides randomization for this module
impl Distribution<Element> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Element {
        let rand_num: usize = rng.gen();
        Element::from((rand_num % Element::Dark as usize) + 1)
    }
}
