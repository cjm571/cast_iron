/* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *\
Filename : resource.rs

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
    //TODO: purpose writeup for resource

Changelog:

\* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * */

use uuid::Uuid;

use super::Element;
use super::coords::Coords;

///////////////////////////////////////////////////////////////////////////////
//  Data Structures
///////////////////////////////////////////////////////////////////////////////

pub struct Resource {
    uid:    Uuid,
    kind:   Element,
    state:  State,
    pos:    Coords,
    radius: u8,
}

#[derive(Copy, Clone)]
pub enum State {
    Depleted    = 0,
    Low         = 1,
    Partial     = 2,
    High        = 3,
    Full        = 4,
    Overflow    = 5,
}


///////////////////////////////////////////////////////////////////////////////
//  Functions and Methods
///////////////////////////////////////////////////////////////////////////////

impl Resource {

    // Creates and returns a new Resource object
    pub fn new() -> Resource {
        Resource {
            uid:    Uuid::new_v4(),
            kind:   Element::Unset,
            state:  State::Depleted,
            pos:    Coords::new(),
            radius: 0,
        }
    }

    // Creates and returns a new Resource object with the given parameters
    pub fn from(_kind: Element, _state: State, _pos: Coords, _radius: u8) -> Resource {
        Resource {
            uid:    Uuid::new_v4(),
            kind:   _kind,
            state:  _state,
            pos:    _pos,
            radius: _radius,
        }
    }

    ///////////////////////////////////////////////////////////////////////////
    //  Mutator Methods
    ///////////////////////////////////////////////////////////////////////////

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
    pub fn replenish(&mut self, _mag: u8) {

        // set the state to the initial + given magnitude
        let state_val = (self.state as u8) + _mag;
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
    pub fn intensify(&mut self, _mag: u8) {
        if (self.radius as u32) + (_mag as u32) < u8::max_value() as u32 {
            self.radius += _mag;
        } else {
            self.radius = u8::max_value();
        }

    }

    // Decreases the radius of the resource
    pub fn weaken(&mut self, _mag: u8) {
        if (self.radius as i32) - (_mag as i32) > u8::min_value() as i32 {
            self.radius -= _mag;
        } else {
            self.radius = u8::min_value();
        }
    }

    ///////////////////////////////////////////////////////////////////////////
    //  Accessor Methods
    ///////////////////////////////////////////////////////////////////////////

    pub fn get_uid(self) -> Uuid {
        self.uid
    }

    pub fn get_kind(self) -> Element {
        self.kind
    }

    pub fn get_state(self) -> State {
        self.state
    }

    pub fn get_position(self) -> Coords {
        self.pos
    }

    pub fn get_radius(self) -> u8 {
        self.radius
    }
}

