// Filename : ability\aspect.rs
// Author   : CJ McAllister
// Created  : 22-11-2017
// License  : TODO: Add license info

///////////////////////////////////////////////////////////////////////////////
// Data members
///////////////////////////////////////////////////////////////////////////////
// Enumeration of the aesthetics (coolness) of an ability
#[allow(dead_code)]
pub enum Aesthetics {
    Unset,
    Beautiful,
    Impressive,
    Erotic,
    Ugly,
    Subtle,
}
// Enumeration of all element types
#[allow(dead_code)]
pub enum Element {
    Unset,
    Fire,
    Ice,
    Wind,
    Water,
    Electric,
    Earth,
}
// Enumeration of method by which an ability is performed
#[allow(dead_code)]
pub enum Method {
    Unset,
    Staff,
    Wand,
    Manual,
    Vocal,
}
// Enumeration of morality aspect of an ability
#[allow(dead_code)]
pub enum Morality {
    Unset,
    Good,
    Neutral,
    Evil,
}
// Enumeration of all schools of an ability
#[allow(dead_code)]
pub enum School {
    Unset,
    Destruction,
    Restoration,
    Conjuration,
    Alteration,
    Illusion,
    Nature,
    Song,
}