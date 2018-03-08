/* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *\
Filename : ability\mod.rs

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
    //TODO: purpose writeup for ability

Changelog:
    CJ McAllister   22 Nov 2017     File created
    CJ McAllister   31 Jan 2018     Added UUID
\* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * */

pub mod aspect;

use std::fmt;

use uuid::Uuid;

use self::aspect::*;
use super::environment::Element;

///////////////////////////////////////////////////////////////////////////////
// Data Structures
///////////////////////////////////////////////////////////////////////////////

// Struct containing all necessary data fields to define an ability for use in CastIron
pub struct Ability {
    uid:        Uuid,
    name:       String,
    aspects:    Aspects,
    potency:    u8,
}


///////////////////////////////////////////////////////////////////////////////
//  Functions and Methods
///////////////////////////////////////////////////////////////////////////////

// TODO: Figure out how to comment this shit well...
impl Ability {
    // Constructor
    pub fn new(_name: &'static str) -> Ability {
        Ability {
            uid:        Uuid::new_v4(),
            name:       _name.to_string(),
            aspects:    Aspects::new(),
            potency:    0,
        }
    }

    ///////////////////////////////////////////////////////////////////////////
    //  Mutator Methods
    ///////////////////////////////////////////////////////////////////////////

    // Name the ability
    pub fn set_name (&mut self, _name: &'static str) {
        self.name.clear();
        self.name.push_str(_name);
    }

    pub fn set_aspects(&mut self, _aspects: Aspects) {
        self.aspects = _aspects;
    }

    pub fn set_aesthetics (&mut self, _aesthetics: Aesthetics) {
        self.aspects.aesthetics = _aesthetics;
    }

    pub fn set_element (&mut self, _element: Element) {
        self.aspects.element = _element;
    }

    pub fn set_method (&mut self, _method: Method) {
        self.aspects.method = _method;
    }

    pub fn set_morality (&mut self, _morality: Morality) {
        self.aspects.morality = _morality;
    }

    pub fn set_school (&mut self, _school: School) {
        self.aspects.school = _school;
    }

    pub fn set_potency (&mut self, _potency: u8) {
        self.potency = _potency;
    }


    ///////////////////////////////////////////////////////////////////////////
    //  Accessor Methods
    ///////////////////////////////////////////////////////////////////////////

    // Returns a reference to the name of the ability
    pub fn uid (&self) -> &Uuid {
        &self.uid
    }

    // Returns a reference to the name of the ability
    pub fn name (&self) -> &String {
        &self.name
    }

    // Returns a reference to the Aspects of the ability
    pub fn aspects (&self) -> &Aspects {
        &self.aspects
    }
    
    // Returns a reference to the ability's aesthetics
    pub fn aesthetics (&self) -> &Aesthetics {
        &self.aspects.aesthetics
    }
    
    // Returns a reference to the ability's element
    pub fn element (&self) -> &Element {
        &self.aspects.element
    }
    
    // Returns a reference to the ability's method
    pub fn method (&self) -> &Method {
        &self.aspects.method
    }
    
    // Returns a reference to the ability's morality
    pub fn morality (&self) -> &Morality {
        &self.aspects.morality
    }
    
    // Returns a reference to the ability's school
    pub fn school (&self) -> &School {
        &self.aspects.school
    }

    // Returns potency of the ability
    pub fn potency (&self) -> u8 {
        self.potency
    }
}

// Equivalence comparison 
impl PartialEq for Ability {
    fn eq(&self, other: &Ability) -> bool {
        self.uid == other.uid
    }
}
impl Eq for Ability {}

// Display output format for abilities
// [UID]:[Name]:[Potency]:[Aspects (ordered CSV)]
impl fmt::Display for Ability {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}:{}:{}", self.uid(), self.name(), self.potency(), self.aspects())
    }
}