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
    Plottable,
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
    uid:        [u8; 16],
    positions:  Vec<coords::Position>,
    element:    Element
}

#[derive(Debug)]
pub enum ObstacleError {
    NoncontiguousObstacle,
}


///////////////////////////////////////////////////////////////////////////////
//  Object Implementation
///////////////////////////////////////////////////////////////////////////////

impl Obstacle {
    /// Fully-qualified constructor
    pub fn new(positions: Vec<coords::Position>, element: Element) -> Result<Self, ObstacleError> {
        // Verify that all positions in list are contiguous
        let mut prev_pos = positions.first().unwrap();
        for pos in positions.iter() {
            if pos.is_neighbor(prev_pos) {
                prev_pos = pos;
            }
            else { // Noncontiguity detected!
                return Err(ObstacleError::NoncontiguousObstacle)
            }
        }
        

        Ok(Self {
            uid:        *Uuid::new_v4().as_bytes(),
            positions,
            element,
        })
    }


    /*  *  *  *  *  *  *  *\
     *  Accessor Methods  *
    \*  *  *  *  *  *  *  */
    
    pub fn uid(&self) -> &[u8; 16] {
        &self.uid
    }
    
    pub fn positions(&self) -> &Vec<coords::Position> {
        &self.positions
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
impl Plottable for Obstacle {
    fn origin(&self) -> &coords::Position {
        self.positions.first().unwrap()
    }
}
impl Randomizable for Obstacle {
    fn rand(ctx: &Context) -> Self {
        // Set UID
        let uid = *Uuid::new_v4().as_bytes();

        //  Get RNG thread handle and generate random origin
        let mut rng = rand::thread_rng();
        let rand_origin = coords::Position::rand(ctx);
        let mut positions = Vec::new();
        positions.push(rand_origin);

        // Up to Context's constraint, make a randomly-snaking string of coords::Position objects
        let mut trial_pos = rand_origin;
        let mut direction_provider: hex_directions::Provider<hex_directions::Side>;
        for _i in 0 .. ctx.max_obstacle_len() {
            // Long, snaking objects are cooler, so we want a bit better odds than a coinflip
            let obstacle_termination_roll: f32 = rng.gen_range(0.0, 1.0);
            if obstacle_termination_roll < OBSTACLE_TERMINATION_ODDS {
                break;
            }

            // Re-roll the direction provider on each iteration so we don't keep turning in the same pattern
            direction_provider = rng.gen();

            // It's possible the current position is completely surrounded, so use this flag to determine
            // if we should stop the obstacle here
            let mut found_good_position = false;

            for direction in direction_provider {
                // Determine if we can move in the current direction
                let trans = coords::Translation::from(direction);
                match trial_pos.translate(&trans, ctx) {
                    Ok(())  => {},      // Translation is valid, carry on
                    Err(_e) => continue // Translation is invalid, try another direction
                };

                // Ensure the new position does not double-back on an existing obstacle cell 
                if positions.contains(&trial_pos) {
                    // Undo the translation and try another direction
                    trial_pos.translate(&(-trans), ctx).expect("Could not undo translation after double-back detection.");
                    continue;
                }

                // All checks passed!
                found_good_position = true;
                break;
            }

            // If we were able to find good Position, create an object and push it into the collection
            if found_good_position {
                positions.push(coords::Position::new(trial_pos.x(), trial_pos.y(), trial_pos.z(), ctx).unwrap());
            } else {
                // Nowhere left to go! Stop the obstacle here
                break;
            }
        }

        // Finally, generate a random element
        let element: Element = rng.gen();

        Self {uid, positions, element}
    }
}