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
    [UID]:[Name]:[Position]:[Fatigue]:[Ability List (CSV)]\n
    [UID]:[Name]:[Position]:[Fatigue]:[Ability List (CSV)]\n
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
use std::fmt::Write as FmtWrite;
use std::io::Error as IoError;
use std::io::{ErrorKind, SeekFrom, BufReader};
use std::error::Error;

use super::actor::Actor;

///////////////////////////////////////////////////////////////////////////////
//  Data Structures
///////////////////////////////////////////////////////////////////////////////

const FILENAME: &'static str = "castiron.dat";
const TEMPLATE: &'static str = "_ACTORS_\n_ABILITIES\n";

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
                                Ok(mut file) => file,
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

// Writes actor data to CastIron data file, creating the file if necessary
// Returns number of bytes written, or IO Error
//TODO: Migrate this to an install folder eventually
pub fn write_actor(_actor: &Actor) -> Result<u32, IoError> {
    // Open castiron.dat for R/W, and create if doesn't exist
    let mut data_file = open_data_file();

    // Read everything into a string for seeking/editing TODO: Optimze this trash
    let mut data_file_buf = String::new();
    data_file.read_to_string(&mut data_file_buf)?;

    // Construct actor data string
    let mut actor_data = String::new();
    write!(actor_data, "{}:{}:{}:{}:", _actor.get_uid(), _actor.get_name(), _actor.get_pos(), _actor.get_cur_fatigue())
        .expect("Error occurred while trying to write in String");
    for abil in _actor.get_abilities() {
        write!(actor_data, "{},", abil.get_name())
            .expect("Error occurred while trying to write in String");
    }
    write!(actor_data, "\n")
        .expect("Error occurred while trying to write in String");
    
    // Check if actor already exists in file
    match data_file_buf.find(_actor.get_uid().to_string().as_str()){
        None => { // Insert new actor record
            let write_cursor = match data_file_buf.find("_ACTORS_"){
                None         => panic!("IO ERROR: Could not find _ACTORS_ in {}", FILENAME),
                Some(loc)    => loc + 9, //use location just after \n
            };
            data_file_buf.insert_str(write_cursor, actor_data.as_str());

            
            // Reset file cursor to start and write
            data_file.seek(SeekFrom::Start(0))?;
            data_file.write(data_file_buf.as_bytes())
                .expect("Error occurred while trying to write to data file.");
        },
        Some(preactor_loc) => { // Overwrite existing actor record
            let (preactor_buf, temp_buf) = data_file_buf.split_at(preactor_loc);
            let postactor_loc = match temp_buf.find("\n"){
                None        => panic!("IO ERROR: Could not find newline after actor data in castiron.dat"),
                Some(loc)   => loc + 1, //use location just after \n
            };
            let (_temp_buf, postactor_buf) = data_file_buf.split_at(postactor_loc);
            
            // Concatenate new actor record between pre- and post-actor data_file_bufs
            let mut final_data_buf = String::from(preactor_buf);
            final_data_buf.push_str(actor_data.as_str());
            final_data_buf.push_str(postactor_buf);

            
            // Reset file cursor to start and write
            data_file.seek(SeekFrom::Start(0))?;
            data_file.write(final_data_buf.as_bytes())
                .expect("Error occurred while trying to write to data file.");
        },
    }
    data_file.flush()?;

    Ok(99)
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

        if data_line.contains(_actor.get_uid().to_string().as_str()) {
            println!("-- Actor UID found at line {}", line_num);
            return Ok(data_line)
        }

        // Clear line buffer and increment in prep for next line
        data_line.clear();
        line_num = line_num + 1;
    }
}

// WINDOWS-SPECIFIC
// 
pub fn win_write_actor (_actor: &Actor) -> Result<u32, IoError> {
    // Read actor data line

    Ok(10)
}


///////////////////////////////////////////////////////////////////////////////
//  Unit Tests
///////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use ::ability::Ability;
    use ::ability::aspect::*;
    use ::environment::Element;

    #[test]
    fn a_file_create() {
        let mut data_file = open_data_file();
        let metadata = match data_file.metadata() {
            Err(_err)    => panic!("Error occurred while attempting to retrieve data file metadata."),
            Ok(meta)    => meta,
        };
        let file_len = metadata.len();

        // Assert that file should only contain the template, which is 20 bytes long
        assert_eq!(file_len, 20);

        let cursor = match data_file.seek(SeekFrom::Current(0)) {
            Err(_err)    => panic!("Error occurred while attempting to check cursor position"),
            Ok(pos)     => pos,
        };

        // Assert that the current cursor position is at the start of the file
        assert_eq!(cursor, 0);
    }

    #[test]
    fn b_actor_write() {
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

        let result = match write_actor(&player_one) {
            Err(io_err) => panic!("IO ERROR: {}", io_err.description()),
            Ok(val)     => val,
        };

        assert_eq!(result, 99);
    }

    #[test]
    fn c_file_update() {
        let player_two = Actor::new("John Public");
        let result = match write_actor(&player_two) {
            Err(io_err) => panic!("IO ERROR: {}", io_err.description()),
            Ok(val)     => val,
        };
        assert_eq!(result, 99);
    }

    #[test]
    fn win_b_actor_write() {
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

        let result = match win_write_actor(&player_one) {
            Err(io_err) => panic!("IO ERROR: {}", io_err.description()),
            Ok(val)     => val,
        };

        assert_eq!(result, 10);
    }
}