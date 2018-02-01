/* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *\
Filename : fileops.rs

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
    Provide file operations for various game-state data structures.

    File format is as follows:

    _ACTORS_
    [UID]:[Name]:[Position]:[Fatigue]:[Ability List (CSV)]\n
    [UID]:[Name]:[Position]:[Fatigue]:[Ability List (CSV)]\n
    ...
    _ABILITIES_
    [UID]:[Name]:[Aspect List (ordered CSV)]:[potency]\n
    [UID]:[Name]:[Aspect List (ordered CSV)]:[potency]\n
    ...
    EOF

Changelog:
    CJ McAllister   30 Jan 2018     File created

\* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * */

use std::fs::OpenOptions;
use std::error::Error;
use std::io::prelude::*;
use std::io;
use super::actor::Actor;

///////////////////////////////////////////////////////////////////////////////
//  Data Structures
///////////////////////////////////////////////////////////////////////////////



///////////////////////////////////////////////////////////////////////////////
//  Functions and Methods
///////////////////////////////////////////////////////////////////////////////

// Writes actor data to actors.dat file, creating the file if necessary
// Returns number of bytes written, or IO Error
//TODO: Migrate this to an install folder eventually
pub fn write_actor(_actor: &Actor) -> Result<u32, io::Error> {
    // Open actors.dat for R/W, and create if doesn't exist
    let mut actors_file = match OpenOptions::new().read(true).append(true).create(true).open("actors.dat") {
        Err(io_err) => panic!("IO ERROR: {}", io_err.description()),
        Ok(file)    => file,
    };

    // Read everything into a string for seeking/editing TODO: Optimze this trash
    let mut buffer = String::new();
    actors_file.read_to_string(&mut buffer)?;
    
    // Seek through buffer for actors

    Ok(99)
}


///////////////////////////////////////////////////////////////////////////////
//  Unit Tests
///////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn file_open() {
        let mut player_one: Actor = Actor::new("CJ McAllister");

        write_actor(&player_one);
        assert_eq!(true, true);
    }
}