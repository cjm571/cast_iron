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
#[derive(Debug, Copy, Clone)]
pub enum Aesthetics {
    Unset       = 0,
    Beautiful   = 1,
    Impressive  = 2,
    Erotic      = 3,
    Ugly        = 4,
    Subtle      = 5,
}
impl From<u8> for Aesthetics {
    fn from(val: u8) -> Self {
        match val {
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

// Enumeration of method by which an ability is performed
#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
pub enum Method {
    Unset   = 0,
    Staff   = 1,
    Wand    = 2,
    Manual  = 3,
    Vocal   = 4,
}
impl From<u8> for Method {
    fn from(val: u8) -> Self {
        match val {
            0 => Method::Unset,
            1 => Method::Staff,
            2 => Method::Wand,
            3 => Method::Manual,
            4 => Method::Vocal,
            _ => panic!("aspect::Method::from: Aspect value out of range")
        }
    }
}

// Enumeration of morality aspect of an ability
#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
pub enum Morality {
    Unset   = 0,
    Good    = 1,
    Neutral = 2,
    Evil    = 3,
}
impl From<u8> for Morality {
    fn from(val: u8) -> Self {
        match val {
            0 => Morality::Unset,
            1 => Morality::Good,
            2 => Morality::Neutral,
            3 => Morality::Evil,
            _ => panic!("aspect::Morality::from: Aspect value out of range")
        }
    }
}

// Enumeration of all schools of an ability
#[allow(dead_code)]
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
impl From<u8> for School {
    fn from(val: u8) -> Self {
        match val {
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
            element:    Element::Unset,
            method:     Method::Unset,
            morality:   Morality::Unset,
            school:     School::Unset,
        }
    }

    // Constructor
    // See Display formatter for expected string format
    pub fn from(data_str: &String) -> Aspects {
        let mut data_chars = data_str.chars();

        // Subtract 48 to extract int value from ascii value
        Aspects {
            aesthetics: Aesthetics::from(data_chars.next().unwrap() as u8 - 48),
            element:    Element::from(data_chars.next().unwrap() as u8 - 48),
            method:     Method::from(data_chars.next().unwrap() as u8 - 48),
            morality:   Morality::from(data_chars.next().unwrap() as u8 - 48),
            school:     School::from(data_chars.next().unwrap() as u8 - 48),
        }
    }
}

// Display output format for aspects
impl fmt::Display for Aspects {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.aesthetics as u8)?;
        write!(f, "{}", self.element as u8)?;
        write!(f, "{}", self.method as u8)?;
        write!(f, "{}", self.morality as u8)?;
        write!(f, "{}", self.school as u8)
    }
}
impl fmt::Debug for Aspects {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Aspects: {{{:?}, {:?}, {:?}, {:?}, {:?}}}", self.aesthetics, self.element, self.method, self.morality, self.school)
    }
}