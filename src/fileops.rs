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

use std::fs::OpenOptions;
use std::error::Error;
use std::io::prelude::*;
use std::io;
use std::io::SeekFrom;
use std::fmt::Write as FmtWrite;

use super::actor::Actor;

///////////////////////////////////////////////////////////////////////////////
//  Data Structures
///////////////////////////////////////////////////////////////////////////////



///////////////////////////////////////////////////////////////////////////////
//  Functions and Methods
///////////////////////////////////////////////////////////////////////////////

// Writes actor data to castiron.dat file, creating the file if necessary
// Returns number of bytes written, or IO Error
//TODO: Migrate this to an install folder eventually
pub fn write_actor(_actor: &Actor) -> Result<u32, io::Error> {
    // Open castiron.dat for R/W, and create if doesn't exist
    let mut data_file = match OpenOptions::new().read(true).write(true).create(true).open("castiron.dat") {
        Err(io_err) => panic!("IO ERROR: {}", io_err.description()),
        Ok(file)    => file,
    };

    // Write top-level template to file if it is empty
    let metadata = data_file.metadata()?;
    if metadata.len() == 0 {
        write!(data_file, "_ACTORS_\n_ABILITIES_\n")
            .expect("Error occurred while trying to write to data file.");
        
        // Reset file cursor to start
        data_file.seek(SeekFrom::Start(0))?;
    }

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
                None         => panic!("IO ERROR: Could not find _ACTORS_ in castiron.dat"),
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

    Ok(99)
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
    fn file_create() {
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
    fn file_update() {
        let player_two = Actor::new("John Public");
        let result = match write_actor(&player_two) {
            Err(io_err) => panic!("IO ERROR: {}", io_err.description()),
            Ok(val)     => val,
        };
        assert_eq!(result, 99);
    }
}