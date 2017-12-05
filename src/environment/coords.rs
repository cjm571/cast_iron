/* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *\
// Filename : environment\coords.rs

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
\* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * */

use std::fmt;

///////////////////////////////////////////////////////////////////////////////
// Data structures
///////////////////////////////////////////////////////////////////////////////

pub struct Coords {
    x: i32,
    y: i32,
    z: i32,
}


///////////////////////////////////////////////////////////////////////////////
//  Functions and Methods
///////////////////////////////////////////////////////////////////////////////
 
impl Coords {

    // Creates a new coordinates object
    pub fn new() -> Coords {
        Coords{
            x: 0,
            y: 0,
            z: 0,
        }
    }

    // Moves the object East by '_mag' cells
    pub fn move_east(&mut self, _mag: i32) {
        self.x = self.x + _mag;
        self.y = self.y - _mag;
    }

    // Moves the object Southeast by '_mag' cells
    pub fn move_southeast(&mut self, _mag: i32) {
        self.z = self.z + _mag;
        self.y = self.y - _mag;
    }

    // Moves the object Southwest by '_mag' cells
    pub fn move_southwest(&mut self, _mag: i32) {
        self.z = self.z + _mag;
        self.x = self.x - _mag;
    }
    
    // Moves the object West by '_mag' cells
    pub fn move_west(&mut self, _mag: i32) {
        self.y = self.y + _mag;
        self.x = self.x - _mag;
    }

    // Moves the object Southwest by '_mag' cells
    pub fn move_northwest(&mut self, _mag: i32) {
        self.y = self.y + _mag;
        self.z = self.z - _mag;
    }

    // Moves the object Southwest by '_mag' cells
    pub fn move_northeast(&mut self, _mag: i32) {
        self.x = self.x + _mag;
        self.z = self.z - _mag;
    }

    // Moves the object by vector
    //  _mag: number of "straightline" cells to move
    //  _dir: direction of movement in radians
    pub fn move_vec(&mut self, _mag: i32, _dir: f64) {
        let flt_mag: f64 = _mag as f64;
        self.move_east((flt_mag * _dir.cos()).ceil() as i32);

        // approximate movement in the "sine" direction with partial NE/NW movement
        let ne_mag: f64 = (_mag / 2) as f64;
        let nw_mag: f64 = (_mag % 2) as f64;
        self.move_northeast((ne_mag * _dir.sin()).ceil() as i32);
        self.move_northwest((nw_mag * _dir.sin()).ceil() as i32);
    }
}

impl fmt::Debug for Coords {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Coords: {{X: {} Y: {} Z: {}}}", self.x, self.y, self.z)
    }
}