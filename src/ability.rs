// Filename : ability.rs
// Author   : CJ McAllister
// Created  : 22-11-2017
// License  : TODO: Add license info

//
// Data members
//

// Enumeration of all element types
#[allow(dead_code)]
enum Element {
    Fire,
    Ice,
    Wind,
    Water,
    Electric,
    Earth,
}
// Enumeration of all schools of an ability
#[allow(dead_code)]
enum School {
    Destruction,
    Restoration,
    Conjuration,
    Alteration,
    Illusion,
    Nature,
    Song,
}
// Enumeration of morality aspect of an ability
#[allow(dead_code)]
enum Morality {
    Good,
    Neutral,
    Evil,
}
// Enumeration of method by which an ability is performed
#[allow(dead_code)]
enum Method {
    Staff,
    Wand,
    Manual,
    Vocal,
}
// Enumeration of the aesthetics (coolness) of an ability
#[allow(dead_code)]
enum Aesthetics {
    Beautiful,
    Impressive,
    Erotic,
    Ugly,
    Subtle,
}

// Struct encapsulating all aspects of an ability
#[allow(dead_code)]
pub struct Ability {
    element: Element,
    school: School,
    morality: Morality,
    method: Method,
    aesthetics: Aesthetics,
}
