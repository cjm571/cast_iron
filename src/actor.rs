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
    //TODO: purpose writeup for actor

Changelog:
    CJ McAllister   21 Nov 2017     File created
    CJ McAllister   31 Jan 2018     Added UUID

\* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * */

use uuid::Uuid;

use ::ability::Ability;
use ::environment::coords::Coords;

///////////////////////////////////////////////////////////////////////////////
//  Data Structures
///////////////////////////////////////////////////////////////////////////////

// Struct containing state information for the Actor
pub struct Actor {
    uid:            Uuid,
    name:           String,               // Actor's name
    pos:            Coords,                // Actor's 3D position in the environment
    cur_fatigue:    u8,            // Actor's current fatigue level
    abilities:      Vec<Ability>,    // List of Actor's Abilities
}

///////////////////////////////////////////////////////////////////////////////
//  Functions and Methods
///////////////////////////////////////////////////////////////////////////////
impl Actor {
    // Constructor
    pub fn new(_name: &'static str) -> Actor {
        Actor {
            uid:            Uuid::new_v4(),
            name:           _name.to_string(),
            pos:            Coords::new(),
            cur_fatigue:    0,
            abilities:      Vec::new(),
        }
    }

    ///////////////////////////////////////////////////////////////////////////
    //  Mutator Methods
    ///////////////////////////////////////////////////////////////////////////

    // Names the actor
    pub fn set_name(&mut self, _name: &'static str) {
        self.name.clear();
        self.name.push_str(_name);
    }

    // Moves the object by vector
    //  _mag: number of "straightline" cells to move
    //  _dir: direction of movement in radians
    pub fn move_vec(&mut self, _mag: i32, _dir: f64) {
        self.pos.move_vec(_mag, _dir);
    }

    // Adds ability to actor's ability list
    pub fn add_ability(&mut self, ability: ::ability::Ability) {
        self.abilities.push(ability);
    }


    ///////////////////////////////////////////////////////////////////////////
    //  Accessor Methods
    ///////////////////////////////////////////////////////////////////////////

    // Returns a reference for the actor's unique ID
    pub fn get_uid(&self) -> &Uuid {
        &self.uid
    }
    
    // Returns a reference for the actor's name
    pub fn get_name(&self) -> &String {
        &self.name
    }

    // Returns a reference for the actor's position
    pub fn get_pos(&self) -> &Coords {
        &self.pos
    }

    // Returns a reference for the actor's current fatigue
    pub fn get_cur_fatigue(&self) -> &u8 {
        &self.cur_fatigue
    }

    // Returns a refernce to the vector of the actor's abilities
    pub fn get_abilities(&self) -> &Vec<Ability>{
        &self.abilities
    }

    
    ///////////////////////////////////////////////////////////////////////////
    //  Other Methods
    ///////////////////////////////////////////////////////////////////////////
     
    // Returns a string representing the actor in the format:
    // [UID]:[Name]:[Position]:[Fatigue]
    pub fn to_string(&self) -> String {
        let mut actor_string = String::new();

        actor_string.push_str(&self.get_uid().to_string());
        actor_string.push_str(":");

        actor_string.push_str(&self.get_name());
        actor_string.push_str(":");

        actor_string.push_str(&self.get_pos().to_string());
        actor_string.push_str(":");

        actor_string.push_str(&self.get_cur_fatigue().to_string());

        actor_string
    }
}