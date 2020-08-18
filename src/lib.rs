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

\* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * */

extern crate chrono;
extern crate rand;
extern crate uuid;


///////////////////////////////////////////////////////////////////////////////
//  Macro Definitions
///////////////////////////////////////////////////////////////////////////////

// Macro for retrieving the function name
#[macro_export]
macro_rules! function_name {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        &name[..name.len() - 3]
    }}
}


// Non-interruptive debug output
#[cfg(debug_assertions)]
#[macro_export]
macro_rules! debug_println {
    ($( $args:expr ),*) => {
        print!( "{}: ", function_name!());
        println!( $( $args ),* );
    };
}
#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! debug_println {
    ($( $args:expr ),*) => {}
}


///////////////////////////////////////////////////////////////////////////////
//  Module Declarations
///////////////////////////////////////////////////////////////////////////////

#[macro_use]
pub mod ability;
pub mod actor;
pub mod context;
pub mod environment;
pub mod fileops;
pub mod hex_direction_provider;
pub mod logger;
pub mod polyfunc;