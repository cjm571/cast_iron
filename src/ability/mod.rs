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

use std::fmt;

use crate::{
    context::Context,
    element::Element,
    Randomizable,
};

use rand::{
    Rng,
    distributions::Alphanumeric,
};
use serde::{Serialize, Deserialize};
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
#[derive(Serialize, Deserialize)]
pub struct Ability {
    uid:        [u8; 16],
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
            uid:        *Uuid::new_v4().as_bytes(),
            name:       name.to_string(),
            potency,
            aspects,
        }
    }
    /// Name-only constructor
    pub fn new_name_only(name: &'static str) -> Self {
        Self {
            uid:        *Uuid::new_v4().as_bytes(),
            name:       name.to_string(),
            potency:    0,
            aspects:    Aspects::default(),
        }
    }


    /*  *  *  *  *  *  *  *\
     *  Mutator Methods   *
    \*  *  *  *  *  *  *  */

    // Name the ability
    pub fn set_name (&mut self, name: &'static str) {
        self.name.clear();
        self.name.push_str(name);
    }

    pub fn set_potency (&mut self, potency: usize) {
        self.potency = potency;
    }

    pub fn set_aspects(&mut self, aspects: Aspects) {
        self.aspects = aspects;
    }

    pub fn set_aesthetics (&mut self, aesthetics: Aesthetics) {
        self.aspects.aesthetics = aesthetics;
    }

    pub fn set_element (&mut self, element: Element) {
        self.aspects.element = element;
    }

    pub fn set_method (&mut self, method: Method) {
        self.aspects.method = method;
    }

    pub fn set_morality (&mut self, morality: Morality) {
        self.aspects.morality = morality;
    }

    pub fn set_school (&mut self, school: School) {
        self.aspects.school = school;
    }


    /*  *  *  *  *  *  *  *\
     *  Accessor Methods  *
    \*  *  *  *  *  *  *  */

    // Returns a reference to the name of the ability
    pub fn uid (&self) -> &[u8; 16] {
        &self.uid
    }

    // Returns a reference to the name of the ability
    pub fn name (&self) -> &String {
        &self.name
    }

    // Returns potency of the ability
    pub fn potency (&self) -> usize {
        self.potency
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
}


///////////////////////////////////////////////////////////////////////////////
//  Trait Implementations
///////////////////////////////////////////////////////////////////////////////

/* NOTE: Default trait intentionally omitted for this object */

impl fmt::Display for Ability {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}:{}:{}", Uuid::from_bytes(self.uid), self.name, self.potency, self.aspects)
    }
}
impl PartialEq for Ability {
    fn eq(&self, other: &Ability) -> bool {
        self.uid == other.uid
    }
}
impl Randomizable for Ability {
    fn rand(ctx: &Context) -> Self {
        // Generate UUID
        let uid = *Uuid::new_v4().as_bytes();

        //OPT: *DESIGN* Pull from list of actual names or something
        // Generate random name
        let mut rng = rand::thread_rng();
        let name: String = rng.sample_iter(&Alphanumeric).take(10).collect();

        // Generate random potency
        let potency: usize = rng.gen();

        // Generate random aspects
        let aspects = Aspects::rand(ctx);

        Self {
            uid,
            name,
            potency,
            aspects,
        }
    }
}
