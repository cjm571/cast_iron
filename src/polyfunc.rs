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
    that are time-driven and have behavior that can be modelled by polynomial functions.

    Available models:
    - Quadratic

    Format: y = -([magnitude]/([duration]/2)^2) * (x - [start_time]) * (x - ([duration]+[start_time]))
            where x is the current game tick

\* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * */

use std::fmt;

use rand::Rng;


///////////////////////////////////////////////////////////////////////////////
//  Data Structures
///////////////////////////////////////////////////////////////////////////////

#[derive(Default)]
pub struct PolyFunc {
    magnitude:  f64,
    duration:   f64,
    start_time: f64,
}


///////////////////////////////////////////////////////////////////////////////
//  Object Implementation
///////////////////////////////////////////////////////////////////////////////

impl PolyFunc {
    /// Fully-qualified constructor
    pub fn new(magnitude: f64, duration: f64, start_time: f64) -> Self {
        Self {magnitude, duration, start_time}
    }

    /// Construct a random polynomial function within the given constraints
    pub fn rand_constrained(max_magnitude: f64, max_duration: f64) -> Self {
        // Generate random values within constraints
        let mut rng = rand::thread_rng();

        let magnitude: f64 = rng.gen_range(0.0, max_magnitude);
        let duration: f64 = rng.gen_range(0.0, max_duration);
        let start_time: f64 = rng.gen();
        
        Self {magnitude, duration, start_time}
    }


    /*  *  *  *  *  *  *  *\
     *  Builder Methods   *
    \*  *  *  *  *  *  *  */
    
    pub fn starting_at(mut self, start_time: f64) -> Self {
        self.start_time = start_time;
        
        self
    }


    /*  *  *  *  *  *  *  *\
     *  Accessor Methods  *
    \*  *  *  *  *  *  *  */

    pub fn duration(&self) -> f64 {
        self.duration
    }

    pub fn start_time(&self) -> f64 {
        self.start_time
    }
    

    /*  *  *  *  *  *  *  *\
     *  Mutator Methods   *
    \*  *  *  *  *  *  *  */

    pub fn set_duration(&mut self, duration: f64) {
        self.duration = duration;
    }

    pub fn set_start_time(&mut self, start_time: f64) {
        self.start_time = start_time;
    }

    
    /*  *  *  *  *  *  *  *\
     *  Utility Methods   *
    \*  *  *  *  *  *  *  */

    // Solves the polynomial function at the given game time tick
    pub fn solve(&self, tick: f64) -> f64 {
        let a: f64 = self.magnitude / (self.duration / 2.0).powi(2);
        let b: f64 = self.start_time;
        let c: f64 = self.start_time + self.duration;

        -a * (tick - b) * (tick - c)
    }
}


///////////////////////////////////////////////////////////////////////////////
//  Trait Implementations
///////////////////////////////////////////////////////////////////////////////

impl fmt::Debug for PolyFunc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PolyFunc: y = -({}/({}/2)^2) * (x - {}) * (x - ({}+{}))",
                  self.magnitude, self.duration,
                  self.start_time,
                  self.duration, self.start_time)?;
        write!(f, "          mag: {}, dur: {}, start_time: {}", self.magnitude, self.duration, self.start_time)
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
        let func_a: PolyFunc = PolyFunc::new(10.0, 4.0, 0.0);
        let func_b: PolyFunc = PolyFunc::new(255.0, 16.0, 10.0);
        let func_c: PolyFunc = PolyFunc::new(150.0, 10.0, 30.0);
        let func_d: PolyFunc = PolyFunc::new(100.0, 5.0, 0.0);

        // Check function A's solutions
        assert_eq!(func_a.solve(0.0), 0.0);
        assert_eq!(func_a.solve(4.0), 0.0);
        assert_eq!(func_a.solve(2.0), 10.0);

        // Check function B's solutions
        assert_eq!(func_b.solve(10.0), 0.0);
        assert_eq!(func_b.solve(26.0), 0.0);
        assert_eq!(func_b.solve(18.0), 255.0);

        // Check function C's solutions
        assert_eq!(func_c.solve(30.0), 0.0);
        assert_eq!(func_c.solve(40.0), 0.0);
        assert_eq!(func_c.solve(35.0), 150.0);

        // Check function D's solutions
        assert_eq!(func_d.solve(0.0), 0.0);
        assert_eq!(func_d.solve(5.0), 0.0);
        assert_eq!(func_d.solve(2.0), 96.0);
    }
}
