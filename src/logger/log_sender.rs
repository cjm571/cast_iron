/* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *\
Filename : logger/log_sender.rs

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
    This module defines the Log Sender object, used to dispatch messages to
    the Reciever, which will avoid blocking the main thread for logging
    operations.

\* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * */

use std::sync::mpsc::{
    Sender,
    SendError
};

use crate::logger::LogTuple;


///////////////////////////////////////////////////////////////////////////////
//  Data Structures
///////////////////////////////////////////////////////////////////////////////

#[derive(Clone)]
pub struct LogSender {
    channel_tx: Sender<LogTuple>
}


///////////////////////////////////////////////////////////////////////////////
//  Object Implementation
///////////////////////////////////////////////////////////////////////////////

impl LogSender {
    /// Fully-qualified constructor
    pub fn new(channel_tx: Sender<LogTuple>) -> Self {
        Self {
            channel_tx: channel_tx
        }
    }

    pub fn send_log(&self, log_tuple: LogTuple) -> Result<(), SendError<LogTuple>> {
        self.channel_tx.send(log_tuple)
    }
}

