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

use std::{
    fmt,
    str::FromStr
};

use crate::{
    ability::Ability,
    context::Context,
    coords,
    hex_directions,
};

use uuid::Uuid;


///////////////////////////////////////////////////////////////////////////////
//  Data Structures
///////////////////////////////////////////////////////////////////////////////

// Struct containing state information for the Actor
pub struct Actor {
    uid:            Uuid,
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
    pub fn new(
        name:           &'static str,
        pos:            coords::Position,
        cur_fatigue:    u8,
        abilities:      Vec<Ability>,
    ) -> Self {
        Self {
            uid:    Uuid::new_v4(),
            name:   name.to_string(),
            pos,
            cur_fatigue,
            abilities,
        }
    }

    /// Name-only constructor
    pub fn new_name_only(name: &'static str) -> Self {
        Self {
            uid:            Uuid::new_v4(),
            name:           name.to_string(),
            pos:            coords::Position::default(),
            cur_fatigue:    0,
            abilities:      Vec::new(),
        }
    }

    pub fn from_string(src: &str, ctx: &Context) -> Self {
        // Tokenize on "|" to separate actor from abil list
        let split_vec: Vec<&str> = src.split('|').collect();

        let actor_str = split_vec[0];
        let abil_str = split_vec[1];

        // Tokenize string on ":"
        let data_vec: Vec<&str> = actor_str.split(':').collect();

        let uid = match Uuid::from_str(data_vec[0]) {
            Ok(uid)     => uid,
            Err(_err)   => panic!("actor::from: Invalid uuid input string."),
        };

        let name = data_vec[1];

        // trim parentheses and tokenize on ','
        let parens: &[_] = &['(', ')']; //WTF: is this type?
        let coord_vec: Vec<&str> = data_vec[2].trim_matches(parens).split(',').collect();
        let pos = match coords::Position::new(
            coord_vec[0].parse::<i32>().unwrap(),
            coord_vec[1].parse::<i32>().unwrap(),
            coord_vec[2].parse::<i32>().unwrap(),
            ctx
        ) {
            Ok(pos)     => pos,
            Err(_err)   => panic!("actor::from: Invalid coords::Position input string."),
        };

        let cur_fatigue = data_vec[3].parse::<u8>().unwrap();

        // Check for empty ability list
        let mut abil_vec: Vec<Ability> = Vec::new();
        if abil_str != "\n" {
            // Tokenize abil list on ";"
            let abil_str_vec: Vec<&str> = abil_str.split(';').collect();

            // Iterate through abil list, creating ability objects and dropping them in the vector
            for abil_str in abil_str_vec {
                abil_vec.push(Ability::from(&abil_str.to_string()));
            }
        }

        Self {
            uid,
            name:       name.to_string(),
            pos,
            cur_fatigue,
            abilities:  abil_vec,
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
    pub fn move_one_cell(&mut self, dir: hex_directions::Side, ctx: &Context) {
        self.pos.move_one_cell(dir, ctx).unwrap();
    }

    // Adds ability to actor's ability list
    pub fn add_ability(&mut self, ability: Ability) {
        self.abilities.push(ability);
    }


    ///
    // Accessor Methods
    ///

    // Returns a reference for the actor's unique ID
    pub fn uid(&self) -> Uuid {
        self.uid
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

// Display output format for actors
// [UID]:[Name]:[Position]:[Fatigue]:[Abilities (CSV)]
impl fmt::Display for Actor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut res = write!(f, "{}:{}:{}:{}:|", self.uid, self.name, self.pos, self.cur_fatigue);

        for abil in &self.abilities {
            res = write!(f, "{}", abil.to_string());

            // Avoid adding a trailing semicolon
            if abil != self.abilities.last().unwrap() {
                res = write!(f, ";");
            }
        }

        res
    }
}

///////////////////////////////////////////////////////////////////////////////
//  Unit Tests
///////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn output_single() {
        // set up an actor object with one ability
        let mut player_one = Actor::new_name_only("CJ McAllister");
        let null_abil = Ability::new_name_only("Null");
        player_one.add_ability(null_abil);

        // output the actor as a string
        println!("{}", player_one.to_string());

        assert_eq!(1,1);
    }

    #[test]
    fn output_multi() {
        // set up an actor object with one ability
        let mut player_one = Actor::new_name_only("CJ McAllister");
        let tbolt = Ability::new_name_only("Thunderbolt");
        let fstorm = Ability::new_name_only("Fire Storm");
        player_one.add_ability(tbolt);
        player_one.add_ability(fstorm);

        // output the actor as a string
        println!("{}", player_one.to_string());

        assert_eq!(1,1);
    }

    #[test]
    fn input() {
        // Create a default game context for the test
        let test_ctx = Context::default();

        // set up an actor object with one ability
        let mut player_one = Actor::new_name_only("CJ McAllister");
        let null_abil = Ability::new_name_only("Null");
        player_one.add_ability(null_abil);

        // feed the actor string into from() to "clone" the actor
        let clone_one = Actor::from_string(&player_one.to_string(), &test_ctx);

        // output both actor strings for comparison
        println!("{}\n{}", player_one.to_string(), clone_one.to_string());

        // assert that the strings are identical
        assert_eq!(player_one.to_string(), clone_one.to_string());
    }
}