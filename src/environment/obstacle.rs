/* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *\
Filename : environment/obstacle.rs

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
    environment::{
        element::{
            Element,
            Elemental
        },
        coords::Coords
    },
    hex_direction_provider::*,
    logger::{
        LoggerInstance,
        LogLevel
    },
    ci_log
};

use uuid::Uuid;
use rand::Rng;


///////////////////////////////////////////////////////////////////////////////
//  Data Structures
///////////////////////////////////////////////////////////////////////////////

/// Odds of terminating an obstacle on a given iteration
const OBSTACLE_TERMINATION_ODDS: f32 = 0.01;

///////////////////////////////////////////////////////////////////////////////
//  Data Structures
///////////////////////////////////////////////////////////////////////////////

#[derive(Debug)]
pub struct Obstacle {
    uid:        Uuid,
    all_coords: Vec<Coords>,
    element:    Element
}


///////////////////////////////////////////////////////////////////////////////
//  Object Implementation
///////////////////////////////////////////////////////////////////////////////

impl Obstacle {
    /// Fully-qualified constructor
    pub fn new(all_coords: Vec<Coords>, element: Element) -> Self {
        Self {
            uid:        Uuid::new_v4(),
            all_coords: all_coords, //FIXME: Need to validate that coords are contiguous
            element:    element,
        }
    }

    /// Constructs a random, valid Obstacle within the constraints of the game Context
    pub fn rand(logger: &LoggerInstance, ctx: &Context) -> Self {
        // Set UID
        let uid = Uuid::new_v4();

        //  Get RNG thread handle and generate random origin
        let mut rng = rand::thread_rng();
        let rand_origin_coords = Coords::rand(ctx);
        let mut all_coords = Vec::new();
        all_coords.push(rand_origin_coords);
        ci_log!(logger, LogLevel::DEBUG, "Origin of rand obstacle: {}", all_coords.last().unwrap());

        // Up to Context's constraint, make a randomly-snaking string of Coords objects
        let mut last_coord = rand_origin_coords;
        let mut direction_provider: HexDirectionProvider<HexSides>;
        for i in 0 .. ctx.get_max_obstacle_len() {
            // Long, snaking objects are cooler, so we want a bit better odds than a coinflip
            let obstacle_termination_roll: f32 = rng.gen_range(0.0, 1.0);
            if obstacle_termination_roll < OBSTACLE_TERMINATION_ODDS {
                ci_log!(logger, LogLevel::DEBUG, "Obstacle terminated after adding {} cells.", i);
                break;
            }

            // Re-roll the direction provider on each iteration
            direction_provider = rng.gen();
            ci_log!(logger, LogLevel::DEBUG, "Re-Rolled dir provider: {:?}", direction_provider);

            // It's possible the current coords are completely surrounded, so use this flag to know
            // if we should stop the obstacle here
            let mut found_good_coords = false;

            for direction in direction_provider {
                // Re-roll the direction provider in case we have to 

                //OPT: *DESIGN* a "try_move_vec" function would be much cleaner here
                // Attempt a move and then check for a double-back
                ci_log!(logger, LogLevel::DEBUG, "Checking for empty adject coords to the {:?}.", direction);
                match last_coord.move_vec(1, direction.into(), logger, ctx) {
                    Ok(()) => {},       // Move succeeded, do nothing
                    Err(_e) => continue // Move failed, try another direction
                };

                ci_log!(logger, LogLevel::DEBUG, "Coords to the {:?} ({}) are unoccupied! Checking for double-back.", direction, last_coord);
                if all_coords.contains(&last_coord) {
                    // Double-back detected! Undo the move, rotate the direction and try again
                    ci_log!(logger, LogLevel::DEBUG, "Double-back detected, trying the next direction.");
                    last_coord.move_vec(-1, direction.into(), logger, ctx).unwrap();
                    continue;
                }

                // All checks passed! Set the success flag and break
                ci_log!(logger, LogLevel::DEBUG, "Cell passed all checks!");
                found_good_coords = true;
                break;
            }

            // If we were able to find good Coords, create an object and push it into the collection
            if found_good_coords {
                let new_coord = last_coord;
                all_coords.push(new_coord);
                ci_log!(logger, LogLevel::DEBUG, "{:?} pushed onto end of obstacle coords chain.", all_coords.last().unwrap());
            }
            else
            {
                // Nowhere left to go! Stop the obstacle here
                break;
            }
        }
        ci_log!(logger, LogLevel::DEBUG, "Finished assembling obstacle coords.");

        // Finally, generate a random element
        let rand_elem: Element = rng.gen();

        Self {
            uid:        uid,
            all_coords: all_coords,
            element:    rand_elem
        }
    }

    ///////////////////////////////////////////////////////////////////////////
    //  Accessor Methods
    ///////////////////////////////////////////////////////////////////////////
    pub fn get_uid(&self) -> Uuid {
        self.uid
    }

    pub fn get_all_coords(&self) -> &Vec<Coords> {
        &self.all_coords
    }
}


///////////////////////////////////////////////////////////////////////////////
//  Trait Implementations
///////////////////////////////////////////////////////////////////////////////

impl Elemental for Obstacle {
    fn get_element(&self) -> Element {
        self.element
    }
}