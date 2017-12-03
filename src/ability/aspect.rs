// Filename : ability\aspect.rs
// Author   : CJ McAllister
// Created  : 22-11-2017
// License  : TODO: Add license info
// Purpose  : TODO: this

use std::fmt;

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
// Enumeration of all element types
#[allow(dead_code)]
#[derive(Debug)]
pub enum Element {
    Unset       = 0,
    Fire        = 1,
    Ice         = 2,
    Wind        = 3,
    Water       = 4,
    Electric    = 5,
    Earth       = 6,
    Light       = 7,
    Dark        = 8,
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