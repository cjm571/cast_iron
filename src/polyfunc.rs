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
    This module provides a PolyFunc object and associated functions to game mechanics
    that are time-driven and have bevahior that can be modelled by polynomial functions.

    Available models:
    - Quadratic

    Format: y = -([magnitude]/([duration]/2)^2) * (x - [start_time]) * (x - ([duration]+[start_time]))
            where x is the current game tick

Changelog:
    CJ McAllister   17 May 2018     File created
    CJ McAllister   27 Aug 2018     Redesign to allow for start times

\* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * */


///////////////////////////////////////////////////////////////////////////////
//  Data Structures
///////////////////////////////////////////////////////////////////////////////

//FIXME: Needs a debug output implementation
pub struct PolyFunc {
    magnitude:  u8,
    duration:   u8,
    start_time: u32
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
            start_time: 1
        }
    }
    
    //FIXME: Should implement the From trait
    // Creates and returns a new PolyFunc object from the given parameters
    pub fn from(magnitude: u8, duration: u8, start_time: u32) -> PolyFunc {
        PolyFunc {
            magnitude:  magnitude,
            duration:   duration,
            start_time: start_time
        }
    }

    // Solves the polynomial function at the given game time tick
    pub fn solve(&self, tick: u32) -> i32 {
        let a: f32 = self.magnitude as f32 / (self.duration as f32 / 2.0).powi(2);
        let b: f32 = self.start_time as f32;
        let c: f32 = (self.start_time + self.duration as u32) as f32;

        (-1.0 * a * (tick as f32 - b) * (tick as f32 - c)) as i32
    }
}


///////////////////////////////////////////////////////////////////////////////
//  Unit Tests
///////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // Confirm that start, stop, and peak values are correct
    fn solving() {
        let func_a: PolyFunc = PolyFunc::from(10, 4, 0);
        let func_b: PolyFunc = PolyFunc::from(255, 16, 10);
        let func_c: PolyFunc = PolyFunc::from(150, 10, 30);
        let func_d: PolyFunc = PolyFunc::from(100, 5, 0);

        // Check function A's solutions
        assert_eq!(func_a.solve(0), 0);
        assert_eq!(func_a.solve(4), 0);
        assert_eq!(func_a.solve(2), 10);

        // Check function B's solutions
        assert_eq!(func_b.solve(10), 0);
        assert_eq!(func_b.solve(26), 0);
        assert_eq!(func_b.solve(18), 255);

        // Check function C's solutions
        assert_eq!(func_c.solve(30), 0);
        assert_eq!(func_c.solve(40), 0);
        assert_eq!(func_c.solve(35), 150);

        // Check function D's solutions
        assert_eq!(func_d.solve(0), 0);
        assert_eq!(func_d.solve(5), 0);
        assert_eq!(func_d.solve(2), 96);
    }
}