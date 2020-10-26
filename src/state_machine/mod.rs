/* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *\
Filename : state_machine/mod.rs

Copyright (C) 2020 CJ McAllister
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
    Defines a semi-generic (still RPG-based) state machine to be populated by
    users of the CastIron game engine.

    It is a "data-driven" state machine, in that States and Transitions are
    generic constructs that the user defines the parameters of. States are
    represented as structs with a user-defined set of entry and exit conditions,
    and Transitions are essentially messages carrying the relevant data from 
    the previous State.

\* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * */


///////////////////////////////////////////////////////////////////////////////
//  Module Declarations
///////////////////////////////////////////////////////////////////////////////

pub mod state;
pub mod transition;
