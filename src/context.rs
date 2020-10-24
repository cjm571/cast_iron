/* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *\
Filename : context.rs

Copyright (C) 2020 CJ McAllister
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
    Context module for tracking high-level state for the CastIron engine.

\* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * */


///////////////////////////////////////////////////////////////////////////////
//  Named Constants
///////////////////////////////////////////////////////////////////////////////

/* CastIron Game Defaults */
/// Default hexagonal grid radius (in cells)
const DEFAULT_GRID_RADIUS:              usize = 10;

/// Default maximum number of attempts before considering random mechanic generation a failure
const DEFAULT_MAX_RAND_ATTEMPTS:        usize = 10;

/// Default maximum for the radius of resources (in cells)
const DEFAULT_MAX_RESOURCE_RADIUS:      usize = 4;

/// Default maximum for the length of an obstacle (in cells)
const DEFAULT_MAX_OBSTACLE_LENGTH:      usize = 10;

/// Default maximum intensity of a weather event
const DEFAULT_MAX_WEATHER_INTENSITY:    f64 = 256.0;

/// Default maximum duration for a weather event (in seconds)
const DEFAULT_MAX_WEATHER_DURATION:     f64 = 10.0;


///////////////////////////////////////////////////////////////////////////////
//  Data structures
///////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
pub struct Context {
    grid_radius:            usize,
    max_obstacle_len:       usize,
    max_rand_attempts:      usize,
    max_resource_radius:    usize,
    max_weather_duration:   f64,
    max_weather_intensity:  f64,
}

#[derive(Default)]
pub struct ContextBuilder {
    context:    Context
}


///////////////////////////////////////////////////////////////////////////////
//  Object Implementation
///////////////////////////////////////////////////////////////////////////////

impl Context {
    //NOTE: Fully-qualified constructor intentionally ommitted due to excessive params


    /*  *  *  *  *  *  *  *\
     *  Accessor Methods  *
    \*  *  *  *  *  *  *  */

    pub fn grid_radius(&self) -> usize {
        self.grid_radius
    }

    pub fn max_obstacle_len(&self) -> usize {
        self.max_obstacle_len
    }

    pub fn max_rand_attempts(&self) -> usize {
        self.max_rand_attempts
    }

    pub fn max_resource_radius(&self) -> usize {
        self.max_resource_radius
    }

    pub fn max_weather_duration(&self) -> f64 {
        self.max_weather_duration
    }

    pub fn max_weather_intensity(&self) -> f64 {
        self.max_weather_intensity
    }
}


impl ContextBuilder {
    
    pub fn build(&self) -> Context {
        self.context.clone()
    }

    /*  *  *  *  *  *  *  *\
     *  Builder Methods   *
    \*  *  *  *  *  *  *  */

    pub fn grid_radius(&'_ mut self, radius: usize) -> &'_ mut Self {
        self.context.grid_radius = radius;
        self
    }

    pub fn max_obstacle_len(&'_ mut self, length: usize) -> &'_ mut Self {
        self.context.max_obstacle_len = length;
        self
    }

    pub fn max_rand_attempts(&'_ mut self, attempts: usize) -> &'_ mut Self {
        self.context.max_rand_attempts = attempts;
        self
    }

    pub fn max_resource_radius(&'_ mut self, radius: usize) -> &'_ mut Self {
        self.context.max_resource_radius = radius;
        self
    }

    pub fn max_weather_duration(&'_ mut self, duration: f64) -> &'_ mut Self {
        self.context.max_weather_duration = duration;
        self
    }

    pub fn max_weather_intensity(&'_ mut self, intensity: f64) -> &'_ mut Self {
        self.context.max_weather_intensity = intensity;
        self
    }
}


///////////////////////////////////////////////////////////////////////////////
//  Trait Implementations
///////////////////////////////////////////////////////////////////////////////

impl Default for Context {
    fn default() -> Self {
        Self {
            grid_radius:            DEFAULT_GRID_RADIUS,
            max_rand_attempts:      DEFAULT_MAX_RAND_ATTEMPTS,
            max_resource_radius:    DEFAULT_MAX_RESOURCE_RADIUS,
            max_obstacle_len:       DEFAULT_MAX_OBSTACLE_LENGTH,
            max_weather_intensity:  DEFAULT_MAX_WEATHER_INTENSITY,
            max_weather_duration:   DEFAULT_MAX_WEATHER_DURATION,
        }
    }
}
