/* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *\
Filename : lib.rs

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
    Top-level module for the CastIron engine.
    
Changelog:

\* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * */

extern crate uuid;

///////////////////////////////////////////////////////////////////////////////
//  Global Data Structures
///////////////////////////////////////////////////////////////////////////////


///////////////////////////////////////////////////////////////////////////////
//  Attribute Definitions
///////////////////////////////////////////////////////////////////////////////

// Non-interruptive debug output
#[cfg(debug_assertions)]
macro_rules! debug_println {
    ($( $args:expr ),*) => {
        println!( $( $args ),* );        
    };
}
#[cfg(not(debug_assertions))]
macro_rules! debug_println {
    ($( $args:expr ),*) => {}
}


///////////////////////////////////////////////////////////////////////////////
//  Module declarations
///////////////////////////////////////////////////////////////////////////////

#[macro_use]
pub mod environment;
pub mod actor;
pub mod ability;
pub mod polyfunc;
pub mod fileops;