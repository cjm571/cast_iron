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
    //TODO: purpose writeup for aspect

Changelog:

\* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * */

use std::fmt;
use ::environment::Element;

///////////////////////////////////////////////////////////////////////////////
// Data Structures
///////////////////////////////////////////////////////////////////////////////

// Enumeration of the aesthetics (coolness) of an ability
#[allow(dead_code)]
#[derive(Debug)]
pub enum Aesthetics {
    Unset       = 0,
    Beautiful   = 1,
    Impressive  = 2,
    Erotic      = 3,
    Ugly        = 4,
    Subtle      = 5,
}
// Enumeration of method by which an ability is performed
#[allow(dead_code)]
#[derive(Debug)]
pub enum Method {
    Unset   = 0,
    Staff   = 1,
    Wand    = 2,
    Manual  = 3,
    Vocal   = 4,
}
// Enumeration of morality aspect of an ability
#[allow(dead_code)]
#[derive(Debug)]
pub enum Morality {
    Unset   = 0,
    Good    = 1,
    Neutral = 2,
    Evil    = 3,
}
// Enumeration of all schools of an ability
#[allow(dead_code)]
#[derive(Debug)]
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
pub struct Aspects {
    pub aesthetics: Aesthetics,
    pub element: Element,
    pub method: Method,
    pub morality: Morality,
    pub school: School,
}


///////////////////////////////////////////////////////////////////////////////
//  Functions and Methods
///////////////////////////////////////////////////////////////////////////////

impl Aspects {
    // Constructor
    pub fn new() -> Aspects {
        Aspects {
            aesthetics: Aesthetics::Unset,
            element: Element::Unset,
            method: Method::Unset,
            morality: Morality::Unset,
            school: School::Unset,
        }
    }
}

impl fmt::Debug for Aspects {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Aspects: {{{:?}, {:?}, {:?}, {:?}, {:?}}}", self.aesthetics, self.element, self.method, self.morality, self.school)
    }
}