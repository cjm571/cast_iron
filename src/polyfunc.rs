/* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *\
Filename : polyfunc.rs

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
    //TODO: purpose writeup for polyfunc
    Polynomial function representation
    Format: y = -[magnitude]x^2 + [duration]x + 0

Changelog:

\* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * */


///////////////////////////////////////////////////////////////////////////////
//  Data Structures
///////////////////////////////////////////////////////////////////////////////

//TODO: This needs architectural rework to accommodate starting times
pub struct PolyFunc {
    magnitude:  u8,
    duration:   u8,
}


///////////////////////////////////////////////////////////////////////////////
//  Functions and Methods
///////////////////////////////////////////////////////////////////////////////

impl PolyFunc {

    // Creates and returns a new PolyFunc object
    pub fn new() -> PolyFunc {
        PolyFunc {
            magnitude:  1,
            duration:   1,
        }
    }
    
    // Creates and returns a new PolyFunc object from the given parameters
    pub fn from(_magnitude: u8, _duration: u8) -> PolyFunc {
        PolyFunc {
            magnitude:  _magnitude,
            duration:   _duration,
        }
    }

    // Solves the polynomial function for the given value of x
    pub fn solve(&self, _x: u32) -> i32 {
        // Note that inputs to this function should all be unsigned, but the result may be negative
        let mag: i32 = self.magnitude as i32;
        let dur: i32 = mag * (self.duration as i32);  // must multiple mag * dur to get the desired duration
        let x: i32 = _x as i32;

        debug_println!("polyfunc::solve: -{}x^2 + {}x + 0", mag, dur);
        let ans = -1*mag*(x*x) + dur*x;
        debug_println!("polyfunc::solve: {}", ans);

        -1*mag*(x*x) + dur*x
    }
}


///////////////////////////////////////////////////////////////////////////////
//  Unit Tests
///////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        //TODO: Implement or delete
        assert_eq!(true, true);
    }
}