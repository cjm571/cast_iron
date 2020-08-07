/* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *\
Filename : environment/resource.rs

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
    This package defines the status of an actor-usable resource. A reource
    can be things like campfires, ponds, etc. that enhance elementally-aligned
    abilities for actors within its radius.

\* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * */

use crate::{
    context::Context,
    environment::{
        coords::Coords,
        element::{
            Element,
            Elemental
        }
    }
};

use uuid::Uuid;
use rand::{
    Rng,
    distributions::{
        Distribution,
        Standard
    }
};


///////////////////////////////////////////////////////////////////////////////
//  Data Structures
///////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct Resource {
    uid:        Uuid,
    element:    Element,
    state:      State,
    coords:     Coords,
    radius:     usize,
}

// OPT: *PERFORMANCE* Do custom enums end up on the stack? if not, remove the Copy/Clone derivations
#[derive(Debug, Copy, Clone)]
pub enum State {
    Depleted    = 0,
    Low         = 1,
    Partial     = 2,
    High        = 3,
    Full        = 4,
    Overflow    = 5,
}


///////////////////////////////////////////////////////////////////////////////
//  Object Implementation
///////////////////////////////////////////////////////////////////////////////

impl Resource {
    /// Fully-qualified constructor
    pub fn new(element: Element, state: State, coords: Coords, radius: usize) -> Self {
        Self {
            uid:        Uuid::new_v4(),
            element:    element,
            state:      state,
            coords:     coords,
            radius:     radius,
        }
    }

    // Creates a random, valid Resource object within the constraints of the game Context
    pub fn rand(ctx: &Context) -> Self {
        // Set UID
        let uid = Uuid::new_v4();

        //  Get RNG thread handle and generate random centerpoint
        let mut rng = rand::thread_rng();

        // Generate random properties
        let rand_elem: Element = rng.gen();
        let rand_state: State = rng.gen();

        // Constrain max resource radius to 1/4 of the total grid
        let rand_radius: usize = rng.gen_range(0, ctx.get_max_resource_radius());

        // Generate a random Coords object that won't spill outside the grid
        let rand_center_coords = Coords::rand_constrained(ctx, rand_radius).unwrap();

        Self {
            uid:        uid,
            element:    rand_elem,
            state:      rand_state,
            coords:     rand_center_coords,
            radius:     rand_radius
        }
    }

    ///
    // Mutator Methods
    ///

    // Attempt to make use of the resource
    // Returns magnitude of potency boost, or None if already depleted
    pub fn consume(&mut self) -> Option<u8> {
        let initial_state = self.state;

        // draw down the resource one tick
        match initial_state {
            State::Depleted => {},
            State::Low      => self.state = State::Depleted,
            State::Partial  => self.state = State::Low,
            State::High     => self.state = State::Partial,
            State::Full     => self.state = State::High,
            State::Overflow => self.state = State::Full,
        }

        // return the magnitude based on the initial state
        match initial_state {
            State::Depleted => None,
            _ => Some(initial_state as u8),
        }
    }

    // Replenish the state of the resource by the given magnitude
    pub fn replenish(&mut self, mag: u8) {

        // set the state to the initial + given magnitude
        let state_val = (self.state as u8) + mag;
        match state_val {
            0 => self.state = State::Depleted,
            1 => self.state = State::Low,
            2 => self.state = State::Partial,
            3 => self.state = State::High,
            4 => self.state = State::Full,
            _ => self.state = State::Overflow,
        }
    }

    // Increases the radius of the resource
    pub fn intensify(&mut self, mag: usize) {
        self.radius = self.radius + mag;
    }

    // Decreases the radius of the resource
    pub fn weaken(&mut self, mag: usize) {
        self.radius = self.radius - mag;
    }


    ///
    // Accessor Methods
    ///

    pub fn get_uid(&self) -> Uuid {
        self.uid
    }

    pub fn get_state(&self) -> State {
        self.state
    }

    pub fn get_coords(&self) -> &Coords {
        &self.coords
    }

    pub fn get_radius(&self) -> usize {
        self.radius
    }
}


///////////////////////////////////////////////////////////////////////////////
//  Trait Implementations
///////////////////////////////////////////////////////////////////////////////

///
// Resource
///
impl Default for Resource {
    fn default() -> Self {
        Self {
            uid:        Uuid::new_v4(),
            element:    Element::default(),
            state:      State::default(),
            coords:     Coords::default(),
            radius:     0,
        }
    }
}
impl Elemental for Resource {
    fn get_element(&self) -> Element {
        self.element
    }
}


///
// State
///
impl Default for State {
    fn default() -> Self {
        Self::Depleted
    }
}
impl From<u8> for State {
    fn from(val: u8) -> Self {
        match val {
            0 => State::Depleted,
            1 => State::Low,
            2 => State::Partial,
            3 => State::High,
            4 => State::Full,
            5 => State::Overflow,
            _ => panic!("State value out of range")
        }
    }
}
// Distribution trait provides randomnization for this module
impl Distribution<State> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> State {
        let rand_num: u8 = rng.gen();
        State::from((rand_num % State::Overflow as u8) + 1)
    }
}

