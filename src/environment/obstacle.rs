/* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *\
Filename : obstacle.rs

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

use crate::context::Context;

use uuid::Uuid;
use rand::Rng;

use super::{
    element::Element,
    coords::Coords
};


///////////////////////////////////////////////////////////////////////////////
//  Data Structures
///////////////////////////////////////////////////////////////////////////////

pub struct Obstacle {
    uid:        Uuid,
    all_coords: Vec<Coords>,
    element:    Element
}


///////////////////////////////////////////////////////////////////////////////
//  Functions and Methods
///////////////////////////////////////////////////////////////////////////////

impl Obstacle {
    /// Generic Constructor
    pub fn new(all_coords: Vec<Coords>, element: Element) -> Self {
        Self {
            uid:        Uuid::new_v4(),
            all_coords: all_coords,
            element:    element
        }
    }

    // Creates a random, valid Obstacle within the constraints of the game Context
    pub fn rand(ctx: &Context) -> Self {
        // Set UID
        let uid = Uuid::new_v4();
        
        //  Get RNG thread handle and generate random origin
        let mut rng = rand::thread_rng();
        let rand_origin_coords = Coords::rand(ctx);
        let mut all_coords = Vec::new();
        all_coords.push(rand_origin_coords);
        
        // Up to Context's constraint, make a randomly-snaking string of Coords objects
        let mut last_coord = rand_origin_coords;
        for _i in 0 .. ctx.get_max_obstacle_len() {
            // Coin-flip to continue
            let coin_flip: bool = rng.gen();
            if coin_flip == false {
                break;
            }

            // Move in a random direction and push the new coords into the collection
            let rand_dir: f64 = rng.gen();
            last_coord.move_vec(1, rand_dir);
            all_coords.push(last_coord);
        }

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

    pub fn get_element(&self) -> Element {
        self.element
    }
}
