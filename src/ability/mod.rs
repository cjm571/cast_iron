// Filename : ability\mod.rs
// Author   : CJ McAllister
// Created  : 22-11-2017
// License  : TODO: Add license info

pub mod aspect;

use self::aspect::*;
use std::collections::HashMap;

///////////////////////////////////////////////////////////////////////////////
// Data Structures
///////////////////////////////////////////////////////////////////////////////


// Struct containing all necessary data fields to define an ability for use in CastIron
#[allow(dead_code)]
pub struct Ability {
    name: String,
    aspects: HashMap<Tag, Aspect>,
    potency: u8,
}

///////////////////////////////////////////////////////////////////////////////
//  Functions and Methods
///////////////////////////////////////////////////////////////////////////////

// TODO: Figure out how to comment this shit well...
impl Ability {
    // Constructor
    pub fn new() -> Ability {
        // Allocate local aspects map
        let mut _aspects: HashMap<Tag, Aspect> = HashMap::new();

        // Populate the map with Unset
        _aspects.insert(Tag::Aesthetics, 
            Aspect { 
                tag: Tag::Aesthetics,
                val: Value{ aesthetics: Aesthetics::Unset}
            });
        _aspects.insert(Tag::Element, 
            Aspect { 
                tag: Tag::Element,
                val: Value{ element: Element::Unset}
            });
        _aspects.insert(Tag::Method, 
            Aspect { 
                tag: Tag::Method,
                val: Value{ method: Method::Unset}
            });
        _aspects.insert(Tag::Morality, 
            Aspect { 
                tag: Tag::Morality,
                val: Value{ morality: Morality::Unset}
            });
        _aspects.insert(Tag::School, 
            Aspect { 
                tag: Tag::School,
                val: Value{ school: School::Unset}
            });

        // Return Ability with ownership of _aspects' data
        Ability {
            name: "Unset".to_string(),
            aspects: _aspects,
            potency: 0,
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

    // Set the aspect of the given tag to the given value
    pub fn set_aspect (&mut self, _tag: Tag, _val: Value) {
        if let Some(_aspect) = self.aspects.get_mut(&_tag){
            *_aspect = Aspect {tag: _tag, val: _val};
        }
    }

    ///////////////////////////////////////////////////////////////////////////
    //  Accessor Methods
    ///////////////////////////////////////////////////////////////////////////

    // Returns a reference to the name of the ability
    pub fn get_name (&self) -> &String {
        &self.name
    }

    // Returns a reference to the given aspect
    pub fn get_aspect (&self, _tag: &Tag) -> &Aspect {
        if let Some(_aspect) = self.aspects.get(_tag){
            _aspect
        } else {
            //TODO: graceful handling
            panic!("Invalid aspect tag requested");
        }
    }

    // Returns a reference to the HashMap that denotes to all ability aspects
    pub fn get_aspects (&self) -> &HashMap<Tag, Aspect> {
        &self.aspects
    }
    // Returns potency of the ability
    pub fn get_potency (&self) -> u8 {
        self.potency
    }
}
