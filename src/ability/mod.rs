// Filename : ability\mod.rs
// Author   : CJ McAllister
// Created  : 22-11-2017
// License  : TODO: Add license info

pub mod aspect;

///////////////////////////////////////////////////////////////////////////////
// Data members
///////////////////////////////////////////////////////////////////////////////
// Struct containing all necessary data fields to define an ability for use in CastIron
#[allow(dead_code)]
pub struct Ability {
    aesthetics: aspect::Aesthetics,
    element: aspect::Element,
    method: aspect::Method,
    morality: aspect::Morality,
    school: aspect::School,
}

// TODO: Figure out how to comment this shit well...
impl Ability {
    // Constructor
    pub fn new() -> Ability {
        Ability {
            aesthetics: aspect::Aesthetics::Unset,
            element: aspect::Element::Unset,
            method: aspect::Method::Unset,
            morality: aspect::Morality::Unset,
            school: aspect::School::Unset,
        }
    }

    //TODO: See if aspects can be placed under some sort of "parent class". Maybe traits can help?
    
    // Set aspect to given value
    pub fn set_aesthetics (&mut self, value: aspect::Aesthetics) {
        self.aesthetics = value;
    }
    pub fn set_element (&mut self, value: aspect::Element) {
        self.element = value;
    }
    pub fn set_method (&mut self, value: aspect::Method) {
        self.method = value;
    }
    pub fn set_morality (&mut self, value: aspect::Morality) {
        self.morality = value;
    }
    pub fn set_school (&mut self, value: aspect::School) {
        self.school = value;
    }
        
}
