// Filename : actor.rs
// Author   : CJ McAllister
// Created  : 22-11-2017
// License  : TODO: Add license info

use ::ability::Ability;

///////////////////////////////////////////////////////////////////////////////
//  Public Data Members
///////////////////////////////////////////////////////////////////////////////

// Struct containing state information for the Actor
#[allow(dead_code)]
pub struct Actor {
    position: [u32; 3],         // Actor's 3D position in the environment
    cur_fatigue: u32,           // Actor's current fatigue level
    abilities: Vec<Ability>,    // List of Actor's Abilities
}

///////////////////////////////////////////////////////////////////////////////
//  Functions and Methods
///////////////////////////////////////////////////////////////////////////////
impl Actor {
    // Constructor
    pub fn new() -> Actor {
        Actor {
            position: [0,0,0],
            cur_fatigue: 0,
            abilities: Vec::new(),
        }
    }

    // Add ability to actor's ability list
    pub fn add_ability(&mut self, ability: ::ability::Ability) {
        self.abilities.push(ability);
    }
}