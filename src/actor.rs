/* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *\
Filename : actor.rs

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
    This module defines the Actor object. All PCs and NPCs will have an
    associated Actor object to record their abilities and game state.

\* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * */

use std::fmt;

use crate::{
    ability::Ability,
    context::Context,
    coords,
    hex_directions,
    Locatable,
    Randomizable,
};

use rand::{
    Rng,
    distributions::Alphanumeric,
};
use serde::{Serialize, Deserialize};
use uuid::Uuid;


//FEAT: Full phylogeny for actor? could be cool way to procedurally generate enemies using features of KPCOFGS hierarchy
///////////////////////////////////////////////////////////////////////////////
//  Data Structures
///////////////////////////////////////////////////////////////////////////////

/// Struct containing state information for the Actor
#[derive(Serialize, Deserialize)]
pub struct Actor {
    uid:            [u8; 16],
    name:           String,             // Actor's name
    pos:            coords::Position,   // Actor's 3D position in the environment
    cur_fatigue:    u8,                 // Actor's current fatigue level
    abilities:      Vec<Ability>,       // List of Actor's Abilities
}


///////////////////////////////////////////////////////////////////////////////
//  Object Implementatoin
///////////////////////////////////////////////////////////////////////////////

impl Actor {
    /// Fully-qualified constructor
    pub fn new(name:        &'static str,
               pos:         coords::Position,
               cur_fatigue: u8,
               abilities:   Vec<Ability>) -> Self {
        Self {
            uid:    *Uuid::new_v4().as_bytes(),
            name:   name.to_string(),
            pos,
            cur_fatigue,
            abilities,
        }
    }

    /// Name-only constructor
    pub fn new_name_only(name: &'static str) -> Self {
        Self {
            uid:            *Uuid::new_v4().as_bytes(),
            name:           name.to_string(),
            pos:            coords::Position::default(),
            cur_fatigue:    0,
            abilities:      Vec::new(),
        }
    }


    ///
    // Mutator Methods
    ///

    // Names the actor
    pub fn set_name(&mut self, _name: &'static str) {
        self.name.clear();
        self.name.push_str(_name);
    }

    /// Moves actor one cell in the given direction
    pub fn move_one_cell(&mut self, dir: hex_directions::Side, ctx: &Context) -> Result<(), coords::CoordsError> {
        let trans = coords::Translation::from(dir);
        
        self.pos.translate(&trans, ctx)
    }

    // Adds ability to actor's ability list
    pub fn add_ability(&mut self, ability: Ability) {
        self.abilities.push(ability);
    }


    ///
    // Accessor Methods
    ///

    // Returns a reference for the actor's unique ID
    pub fn uid(&self) -> &[u8; 16] {
        &self.uid
    }

    // Returns a reference for the actor's name
    pub fn name(&self) -> &String {
        &self.name
    }

    // Returns a reference for the actor's position
    pub fn pos(&self) -> &coords::Position {
        &self.pos
    }

    // Returns a reference for the actor's current fatigue
    pub fn cur_fatigue(&self) -> &u8 {
        &self.cur_fatigue
    }

    // Returns a refernce to the vector of the actor's abilities
    pub fn abilities(&self) -> &Vec<Ability>{
        &self.abilities
    }
}

///////////////////////////////////////////////////////////////////////////////
//  Trait Implementations
///////////////////////////////////////////////////////////////////////////////

// Display output format for actors
// [UID]:[Name]:[Position]:[Fatigue]:[Abilities (CSV)]
impl fmt::Display for Actor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut res = write!(f, "{}:{}:{}:{}:|", Uuid::from_bytes(self.uid), self.name, self.pos, self.cur_fatigue);

        for (i, abil) in self.abilities.iter().enumerate() {
            res = write!(f, "{}", abil.to_string());

            // Avoid adding a trailing semicolon
            if i == self.abilities.len()-1 {
                res = write!(f, ";");
            }
        }

        res
    }
}
impl Locatable for Actor {
    fn origin(&self) -> &coords::Position {
        &self.pos
    }
}
impl Randomizable for Actor {
    fn rand(ctx: &Context) -> Self {
        // Generate UUID
        let uid = *Uuid::new_v4().as_bytes();

        //FEAT: Pull from list of actual names or something
        // Generate random name
        let name: String = rand::thread_rng().sample_iter(&Alphanumeric)
                                             .take(10)
                                             .collect();

        // Generate a random position
        let pos: coords::Position = coords::Position::rand(ctx);

        // New actor, so fatigue should be 0
        let cur_fatigue = 0;

        //OPT: *DESIGN* Make the count random as well
        // Generate random abilities
        let mut abilities: Vec<Ability> = Vec::new();
        for _i in 0 .. 5 {
            abilities.push(Ability::rand(ctx));
        }

        Self {
            uid,
            name,
            pos,
            cur_fatigue,
            abilities,
        }
    }
}
