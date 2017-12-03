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
    name: String,               // Actor's name
    //TODO: consider a hashmap or something for more readable x, y, z keying
    pos: [u32; 3],              // Actor's 3D position in the environment
    cur_fatigue: u8,           // Actor's current fatigue level
    abilities: Vec<Ability>,    // List of Actor's Abilities
}

///////////////////////////////////////////////////////////////////////////////
//  Functions and Methods
///////////////////////////////////////////////////////////////////////////////
impl Actor {
    // Constructor
    pub fn new(_name: &'static str) -> Actor {
        Actor {
            name: _name.to_string(),
            pos: [0,0,0],
            cur_fatigue: 0,
            abilities: Vec::new(),
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

    // Adds ability to actor's ability list
    pub fn add_ability(&mut self, ability: ::ability::Ability) {
        self.abilities.push(ability);
    }


    ///////////////////////////////////////////////////////////////////////////
    //  Accessor Methods
    ///////////////////////////////////////////////////////////////////////////

    // Returns a reference for the actor's name
    pub fn get_name(&self) -> &String {
        &self.name
    }

    // Returns a reference for the actor's position
    pub fn get_pos(&self) -> &[u32] {
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
}