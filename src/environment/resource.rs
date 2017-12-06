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
    state:  ResourceState,
    pos:    Coords,
    radius: u8,
}

pub enum ResourceState {
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

pub fn new() -> Resource {
    Resource {
        uid:    Uuid::new_v4(),
        kind:   Element::Unset,
        state:  ResourceState::Depleted,
        pos:    Coords::new(),
        radius: 0,
    }
}
