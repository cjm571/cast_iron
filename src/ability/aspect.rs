// Filename : ability\aspect.rs
// Author   : CJ McAllister
// Created  : 22-11-2017
// License  : TODO: Add license info
// Purpose  : TODO: this

///////////////////////////////////////////////////////////////////////////////
// Data Structures
///////////////////////////////////////////////////////////////////////////////

// Enumeration of the aesthetics (coolness) of an ability
#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub enum Aesthetics {
    Unset,
    Beautiful,
    Impressive,
    Erotic,
    Ugly,
    Subtle,
    COUNT,
}
// Enumeration of all element types
#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub enum Element {
    Unset,
    Fire,
    Ice,
    Wind,
    Water,
    Electric,
    Earth,
    COUNT,
}
// Enumeration of method by which an ability is performed
#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub enum Method {
    Unset,
    Staff,
    Wand,
    Manual,
    Vocal,
    COUNT,
}
// Enumeration of morality aspect of an ability
#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub enum Morality {
    Unset,
    Good,
    Neutral,
    Evil,
    COUNT,
}
// Enumeration of all schools of an ability
#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub enum School {
    Unset,
    Destruction,
    Restoration,
    Conjuration,
    Alteration,
    Illusion,
    Nature,
    Song,
    COUNT,
}


// Enumeration of aspects for tagging Aspect union
#[derive(PartialEq, Eq, Hash, Debug)]
pub enum Tag {
    Aesthetics,
    Element,
    Method,
    Morality,
    School,
    COUNT,
}
// Union for holding one aspect at a time
pub union Value {
    pub aesthetics: Aesthetics,
    pub element: Element,
    pub method: Method,
    pub morality: Morality,
    pub school: School,
}
// Struct for public access of single aspects
pub struct Aspect {
    pub tag: Tag,
    pub val: Value,
}

//TODO: "pub"s above may be better designed by using accessor methods