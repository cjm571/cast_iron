/* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *\
Filename : mechanics/obstacle.rs

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
    This package defines obstacles in the game world. They may interfere with
    actors, resources, etc. in a number of ways.

    Note - a single obstacle may occupy more than one hex cell.

\* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * */

use crate::{
    context::Context,
    coords,
    element::{
        Element,
        Elemental
    },
    hex_directions,
    Locatable,
    Randomizable,
};

use uuid::Uuid;
use rand::Rng;


///////////////////////////////////////////////////////////////////////////////
//  Data Structures
///////////////////////////////////////////////////////////////////////////////

/// Odds of terminating an obstacle on a given iteration
const OBSTACLE_TERMINATION_ODDS: f32 = 0.05;

///////////////////////////////////////////////////////////////////////////////
//  Data Structures
///////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct Obstacle {
    uid:        Uuid,
    all_coords: Vec<coords::Position>,
    element:    Element
}


///////////////////////////////////////////////////////////////////////////////
//  Object Implementation
///////////////////////////////////////////////////////////////////////////////

impl Obstacle {
    /// Fully-qualified constructor
    pub fn new(all_coords: Vec<coords::Position>, element: Element) -> Self {
        Self {
            uid: Uuid::new_v4(),
            all_coords, //TODO: Need to validate that coords are contiguous
            element,
        }
    }
    
    
    /*  *  *  *  *  *  *\
     * Accessor Methods *
    \*  *  *  *  *  *  */
    
    pub fn uid(&self) -> Uuid {
        self.uid
    }
    
    pub fn all_coords(&self) -> &Vec<coords::Position> {
        &self.all_coords
    }
}


///////////////////////////////////////////////////////////////////////////////
//  Trait Implementations
///////////////////////////////////////////////////////////////////////////////

impl Elemental for Obstacle {
    fn element(&self) -> Element {
        self.element
    }
}
impl Locatable for Obstacle {
    fn origin(&self) -> &coords::Position {
        self.all_coords.first().unwrap()
    }
}
impl Randomizable for Obstacle {
    fn rand(ctx: &Context) -> Self {
        // Set UID
        let uid = Uuid::new_v4();

        //  Get RNG thread handle and generate random origin
        let mut rng = rand::thread_rng();
        let rand_origin_coords = coords::Position::rand(ctx);
        let mut all_coords = Vec::new();
        all_coords.push(rand_origin_coords);

        // Up to Context's constraint, make a randomly-snaking string of coords::Position objects
        let mut last_coord = rand_origin_coords;
        let mut direction_provider: hex_directions::Provider<hex_directions::Side>;
        for _i in 0 .. ctx.max_obstacle_len() {
            // Long, snaking objects are cooler, so we want a bit better odds than a coinflip
            let obstacle_termination_roll: f32 = rng.gen_range(0.0, 1.0);
            if obstacle_termination_roll < OBSTACLE_TERMINATION_ODDS {
                break;
            }

            // Re-roll the direction provider on each iteration so we don't keep turning in the same pattern
            direction_provider = rng.gen();

            // It's possible the current coords are completely surrounded, so use this flag to determine
            // if we should stop the obstacle here
            let mut found_good_coords = false;

            for direction in direction_provider {
                // Create a temporary copy of the last position, to check if the current direction is a valid destination
                let mut trial_coord = last_coord;
                match trial_coord.move_one_cell(direction, ctx) {
                    Ok(()) => {},       // Move succeeded, do nothing
                    Err(_e) => continue // Move failed, try another direction
                };

                //FEAT: Need to do a global collision check here?

                // Ensure the new position does not double-back on an existing obstacle cell 
                if all_coords.contains(&trial_coord) {
                    continue;
                }

                // All checks passed!
                last_coord = trial_coord;
                found_good_coords = true;
                break;
            }

            // If we were able to find good Position, create an object and push it into the collection
            if found_good_coords {
                all_coords.push(coords::Position::new(last_coord.x(), last_coord.y(), last_coord.z(), ctx).unwrap());
            } else {
                // Nowhere left to go! Stop the obstacle here
                break;
            }
        }

        // Finally, generate a random element
        let element: Element = rng.gen();

        Self {uid, all_coords, element}
    }
}