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

    _ACTORS_\n
    [UID]:[Name]:[Position]:[Fatigue]:[Ability UID List (CSV)]\n
    [UID]:[Name]:[Position]:[Fatigue]:[Ability UID List (CSV)]\n
    ...
    _ABILITIES_\n
    [UID]:[Name]:[Aspect List (ordered CSV)]:[potency]\n
    [UID]:[Name]:[Aspect List (ordered CSV)]:[potency]\n
    ...
    EOF

Changelog:
    CJ McAllister   30 Jan 2018     File created

\* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * */

use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::io::Error as IoError;
use std::io::{ErrorKind, SeekFrom, BufReader};
use std::error::Error;

use super::actor::Actor;
use super::ability::Ability;

///////////////////////////////////////////////////////////////////////////////
//  Data Structures
///////////////////////////////////////////////////////////////////////////////

//TODO: Migrate this to an install folder eventually
const FILENAME: &'static str = "castiron.dat";
const TEMPLATE: &'static str = "_ACTORS_\n_ABILITIES_";

///////////////////////////////////////////////////////////////////////////////
//  Functions and Methods
///////////////////////////////////////////////////////////////////////////////

// Opens the CastIron data file, creates it if it doesn't exist.
// Returns a File with R/W and cursor at position 0
fn open_data_file() -> File {
    // Attemp to create a new data file
    match OpenOptions::new().read(true).write(true).create_new(true).open(FILENAME) {
        Err(ref io_err) if io_err.kind() == ErrorKind::AlreadyExists => {
                            // Data file already exists, open it as-is
                            match OpenOptions::new().read(true).write(true).open(FILENAME) {
                                Err(io_err) => panic!("IO ERROR: {}", io_err.description()),
                                Ok(file) => file,
                            }
                        },
        Err(io_err) => panic!("IO_ERROR: {}", io_err.description()),
        Ok(mut file)    => {
            // Data file did not exist, populate it with template
            match file.write_all(TEMPLATE.as_bytes()) {
                Err(io_err) => panic!("IO_ERROR: {}", io_err.description()),
                Ok(())      => (),
            }

            // Reset cursor to 0
            match file.seek(SeekFrom::Start(0)) {
                Err(io_err) => panic!("IO_ERROR: {}", io_err.description()),
                Ok(_pos)    => (),
            }

            // Return the File object
            file
        }
    }
}

// Reads actor data from CastIron data file
// Returns actor data, or an IO Error if not found
pub fn read_actor (_actor: &Actor) -> Result<String, IoError> {
    // Open data file for R/W
    let data_file = open_data_file();

    // Search for actor
    let mut data_reader = BufReader::new(data_file);
    let mut data_line = String::new();
    let mut line_num: i32 = 0;
    loop {
        data_reader.read_line(&mut data_line)?;
        if data_line.is_empty() {
            println!("-- EOF found at line {}", line_num);
            return Err(IoError::new(ErrorKind::NotFound, "Actor data not found"))
        }

        if data_line.contains(_actor.uid().to_string().as_str()) {
            println!("-- Actor UID found at line {}", line_num);
            return Ok(data_line)
        }

        // Clear line buffer and increment in prep for next line
        data_line.clear();
        line_num = line_num + 1;
    }
}

// Writes actor data to CastIron data file, creating the file if necessary
pub fn write_actor(actor: &Actor) -> Result<(), IoError> {
    // Open castiron.dat for R/W, and create if doesn't exist
    let mut data_file = open_data_file();

    // Read the file into one big string buffer
    let mut data_buf = String::new();
    data_file.read_to_string(&mut data_buf)?;

    // Tokenize the string on '\n' to get the lines as Strings
    let data_strs: Vec<&str> = data_buf.split('\n').collect();
    let mut data_lines: Vec<String> = Vec::new();
    for data_str in data_strs {
        data_lines.push(data_str.to_string());
    }

    // Check the lines between _ACTOR_ and _ABILITIES_ for the given actor
    for i in 0 .. data_lines.len() {

        // Did not find actor, append a new actor entry
        if data_lines[i].contains("_ABILITIES_") {
            data_lines.insert(i, actor.to_string());
            break;
        }

        // Found actor, overwrite existing line
        if data_lines[i].starts_with(actor.uid().to_string().as_str()) {
            data_lines[i] = actor.to_string();
            break;
        }
    }

    // Push the data lines back together
    let mut upd_data_buf = String::new();
    for i in 0 .. data_lines.len() {
        upd_data_buf = upd_data_buf + data_lines[i].as_str();

        // Avoid adding a trailing \n
        if i != (data_lines.len() - 1) {
            upd_data_buf = upd_data_buf + "\n";
        }
    }

    // Write the updated data back to the file
    data_file.seek(SeekFrom::Start(0))?;
    data_file.write_all(upd_data_buf.as_bytes())
}

// Writes ability data to CastIron data file, creating the file if necessary
pub fn write_ability(abil: &Ability) -> Result<(), IoError> {
    // Open castiron.dat for R/W, and create if doesn't exist
    let mut data_file = open_data_file();

    // Read the file into one big string buffer
    let mut data_buf = String::new();
    data_file.read_to_string(&mut data_buf)?;
    
    // Tokenize the string on '\n' to get the lines as Strings
    let data_strs: Vec<&str> = data_buf.split('\n').collect();
    let mut data_lines: Vec<String> = Vec::new();
    for data_str in data_strs {
        data_lines.push(data_str.to_string());
    }

    // Check the lines after _ABILITIES_ for the given ability
    for i in 0 .. data_lines.len() {

        // Found ability, overwrite existing line
        if data_lines[i].starts_with(abil.uid().to_string().as_str()) {
            data_lines[i] = abil.to_string();
            break;
        }

        // EOF reached, append a new ability entry
        if i == (data_lines.len() - 1) {
            data_lines.push(abil.to_string());
            break;
        }
    }       

    // Push the data lines back together
    let mut upd_data_buf = String::new();
    for i in 0 .. data_lines.len() {
        upd_data_buf = upd_data_buf + data_lines[i].as_str();

        // Avoid adding a trailing \n
        if i != (data_lines.len() - 1) {
            upd_data_buf = upd_data_buf + "\n";
        }
    }

    // Write the updated data back to the file
    data_file.seek(SeekFrom::Start(0))?;
    data_file.write_all(upd_data_buf.as_bytes())
}

///////////////////////////////////////////////////////////////////////////////
//  Unit Tests
///////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;
    use ::ability::Ability;
    use ::ability::aspect::*;
    use ::environment::Element;
    
    // Helper functions
    fn create_p1() -> Actor{
        let mut player_one = Actor::new("CJ McAllister");

        let null_abil = Ability::new("Null");
        let mut lightning_bolt = Ability::new("Lightning Bolt");
        lightning_bolt.set_potency(20);
        lightning_bolt.set_aesthetics(Aesthetics::Impressive);
        lightning_bolt.set_element(Element::Electric);
        lightning_bolt.set_method(Method::Wand);
        lightning_bolt.set_morality(Morality::Neutral);
        lightning_bolt.set_school(School::Destruction);

        player_one.add_ability(null_abil);
        player_one.add_ability(lightning_bolt);

        player_one
    }

    #[test]
    fn file_create() {
        // Delete the file so we start clean
        match fs::remove_file(FILENAME) {
            _ => (), // ignore errors
        }

        let data_file = open_data_file();
        
        let metadata = match data_file.metadata() {
            Err(_err)   => panic!("Error occurred while attempting to retrieve data file metadata."),
            Ok(meta)    => meta,
        };
        let file_len = metadata.len();

        // Assert that file should only contain the template
        assert_eq!(file_len, TEMPLATE.len() as u64);
    }

    #[test]
    fn actor_write() {
        // Delete the file so we start clean
        match fs::remove_file(FILENAME) {
            _ => (), // ignore errors
        }

        let player_one = create_p1();

        let result = match write_actor(&player_one) {
            Err(io_err) => panic!("IO ERROR: {}", io_err.description()),
            Ok(_tmp)    => (),
        };

        assert_eq!(result, ());
    }

    #[test]
    fn file_update() {       
        // Delete the file so we start clean
        match fs::remove_file(FILENAME) {
            _ => (), // ignore errors
        }

        let player_one = create_p1();
        let player_two = Actor::new("John Public");

        match write_actor(&player_one) {
            Err(io_err) => panic!("IO ERROR: {}", io_err.description()),
            Ok(_tmp)    => (),
        };

        match write_actor(&player_two) {
            Err(io_err) => panic!("IO ERROR: {}", io_err.description()),
            Ok(_tmp)    => (),
        };
    }

    #[test]
    fn abil_write() {  
        // Delete the file so we start clean
        match fs::remove_file(FILENAME) {
            _ => (), // ignore errors
        }

        let mut lightning_bolt = Ability::new("Lightning Bolt");
        lightning_bolt.set_potency(20);
        lightning_bolt.set_aesthetics(Aesthetics::Impressive);
        lightning_bolt.set_element(Element::Electric);
        lightning_bolt.set_method(Method::Wand);
        lightning_bolt.set_morality(Morality::Neutral);
        lightning_bolt.set_school(School::Destruction);

        let null_abil = Ability::new("Null");

        match write_ability(&lightning_bolt) {
            Err(io_err) => panic!("IO ERROR: {}", io_err.description()),
            Ok(_tmp)    => (),
        }
        match write_ability(&null_abil) {
            Err(io_err) => panic!("IO ERROR: {}", io_err.description()),
            Ok(_tmp)    => (),
        }
    }

    #[test]
    fn full_write() {
        // Delete the file so we start clean
        match fs::remove_file(FILENAME) {
            _ => (), // ignore errors
        }

        let player_one = create_p1();
        let player_two = Actor::new("John Public");

        write_actor(&player_one).unwrap();
        for abil in player_one.abilities() {
            write_ability(abil).unwrap();
        }

        write_actor(&player_two).unwrap();
    }
}