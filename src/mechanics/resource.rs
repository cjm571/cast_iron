/* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *\
Filename : mechanics/resource.rs

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
    This package defines the status of an actor-usable resource. A resource
    can be things like campfires, ponds, etc. that enhance elementally-aligned
    abilities for actors within its radius.

\* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * */

use crate::{
    context::Context,
    coords,
    element::{
        Element,
        Elemental
    },
    Plottable,
    Randomizable,
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
    uid:        [u8; 16],
    element:    Element,
    state:      State,
    origin:     coords::Position,
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
    pub fn new(element: Element, state: State, origin: coords::Position, radius: usize) -> Self {
        Self {
            uid:        *Uuid::new_v4().as_bytes(),
            element,
            state,
            origin,
            radius,
        }
    }


    /*  *  *  *  *  *  *  *\
     *  Accessor Methods  *
    \*  *  *  *  *  *  *  */

    pub fn uid(&self) -> &[u8; 16] {
        &self.uid
    }

    pub fn state(&self) -> State {
        self.state
    }

    pub fn radius(&self) -> usize {
        self.radius
    }

    
    /*  *  *  *  *  *  *  *\
     *  Mutator Methods   *
    \*  *  *  *  *  *  *  */

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
        self.radius += mag;
    }

    // Decreases the radius of the resource
    pub fn weaken(&mut self, mag: usize) {
        self.radius -= mag;
    }
}


///////////////////////////////////////////////////////////////////////////////
//  Trait Implementations
///////////////////////////////////////////////////////////////////////////////

/*  *  *  *  *  *  *  *\
 *      Resource      *
\*  *  *  *  *  *  *  */
impl Default for Resource {
    fn default() -> Self {
        Self {
            uid:        *Uuid::new_v4().as_bytes(),
            element:    Element::default(),
            state:      State::default(),
            origin:     coords::Position::default(),
            radius:     0,
        }
    }
}
impl Elemental for Resource {
    fn element(&self) -> Element {
        self.element
    }
}
impl Plottable for Resource {
    fn origin(&self) -> &coords::Position {
        &self.origin
    }
}
impl Randomizable for Resource {
    fn rand(ctx: &Context) -> Self {
        // Set UID
        let uid = *Uuid::new_v4().as_bytes();

        //  Get RNG thread handle and generate random centerpoint
        let mut rng = rand::thread_rng();

        // Generate random properties
        let element: Element = rng.gen();
        let state: State = rng.gen();

        // Constrain max resource radius to 1/4 of the total grid
        let radius: usize = rng.gen_range(0, ctx.max_resource_radius());

        // Generate a random coords::Position object that won't spill outside the grid
        let origin = coords::Position::rand_constrained(ctx, radius).unwrap();

        Self {
            uid,
            element,
            state,
            origin,
            radius,
        }
    }
}


/*  *  *  *  *  *  *  *\
 *       State        *
\*  *  *  *  *  *  *  */
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
// Distribution trait provides randomization for this module
impl Distribution<State> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> State {
        let rand_num: u8 = rng.gen();
        State::from((rand_num % State::Overflow as u8) + 1)
    }
}
