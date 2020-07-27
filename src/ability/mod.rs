/* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *\
Filename : ability/mod.rs

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
    This module defines, and provides interfaces for, CastIron abilities.

    Abilities encompass all actions (spells, melee/ranged attacks, etc.) that
    one actor can take on another actor or group of actors.

\* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * */

use std::{
    fmt,
    str::FromStr
};

use crate::environment::element::Element;

use uuid::Uuid;


///////////////////////////////////////////////////////////////////////////////
//  Module Declarations
///////////////////////////////////////////////////////////////////////////////

pub mod aspect;
use self::aspect::*;


///////////////////////////////////////////////////////////////////////////////
// Data Structures
///////////////////////////////////////////////////////////////////////////////

/// Struct containing all necessary data fields to define an ability for use in CastIron
pub struct Ability {
    uid:        Uuid,
    name:       String,
    aspects:    Aspects,
    potency:    usize,
}


///////////////////////////////////////////////////////////////////////////////
// Object Implementation
///////////////////////////////////////////////////////////////////////////////

impl Ability {
    /// Fully-qualified constructor
    pub fn new(name: &'static str, potency: usize, aspects: Aspects) -> Self {
        Self {
            uid:        Uuid::new_v4(),
            name:       name.to_string(),
            potency:    potency,
            aspects:    aspects,
        }
    }
    /// Name-only constructor
    pub fn new_name_only(name: &'static str) -> Self {
        Self {
            uid:        Uuid::new_v4(),
            name:       name.to_string(),
            potency:    0,
            aspects:    Aspects::default(),
        }
    }


    ///
    // Mutator Methods
    ///

    // Name the ability
    pub fn set_name (&mut self, _name: &'static str) {
        self.name.clear();
        self.name.push_str(_name);
    }

    pub fn set_potency (&mut self, _potency: usize) {
        self.potency = _potency;
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


    ///
    // Accessor Methods
    ///

    // Returns a reference to the name of the ability
    pub fn get_uid (&self) -> &Uuid {
        &self.uid
    }

    // Returns a reference to the name of the ability
    pub fn get_name (&self) -> &String {
        &self.name
    }

    // Returns potency of the ability
    pub fn get_potency (&self) -> usize {
        self.potency
    }

    // Returns a reference to the Aspects of the ability
    pub fn get_aspects (&self) -> &Aspects {
        &self.aspects
    }

    // Returns a reference to the ability's aesthetics
    pub fn get_aesthetics (&self) -> &Aesthetics {
        &self.aspects.aesthetics
    }

    // Returns a reference to the ability's element
    pub fn get_element (&self) -> &Element {
        &self.aspects.element
    }

    // Returns a reference to the ability's method
    pub fn get_method (&self) -> &Method {
        &self.aspects.method
    }

    // Returns a reference to the ability's morality
    pub fn get_morality (&self) -> &Morality {
        &self.aspects.morality
    }

    // Returns a reference to the ability's school
    pub fn get_school (&self) -> &School {
        &self.aspects.school
    }
}


///////////////////////////////////////////////////////////////////////////////
//  Trait Implementations
///////////////////////////////////////////////////////////////////////////////

/* NOTE: Default trait intentionally omitted for this object */

impl From<&String> for Ability {
    fn from (src: &String) -> Self {
        // Tokenize on ":"
        let data_vec: Vec<&str> = src.split(':').collect();

        let uid = match Uuid::from_str(data_vec[0]) {
            Ok(uid)     => uid,
            Err(_err)   => panic!("actor::from: Invalid uuid input string."),
        };

        let name = data_vec[1].to_string();

        let potency = data_vec[2].parse::<usize>().unwrap();

        let aspects = Aspects::from(&data_vec[3].to_string());

        Self {
            uid:        uid,
            name:       name,
            potency:    potency,
            aspects:    aspects,
        }

    }
}
impl PartialEq for Ability {
    fn eq(&self, other: &Ability) -> bool {
        self.uid == other.uid
    }
}
impl fmt::Display for Ability {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}:{}:{}", self.get_uid(), self.get_name(), self.get_potency(), self.get_aspects())
    }
}
