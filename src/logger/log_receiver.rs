/* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *\
Filename : logger/log_receiver.rs

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
    This module defines the Log Receiver module, which will exist in its own
    thread, listening for messages on a channel from the Sender.

\* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * */

use std::sync::mpsc;

use crate::logger::{
    LogLevel,
    LogTuple,
    PADDING_FOR_LEVEL_LABEL
};

use chrono::Local;


///////////////////////////////////////////////////////////////////////////////
//  Data Structures
///////////////////////////////////////////////////////////////////////////////

pub struct LogReceiver {
    channel_rx: mpsc::Receiver<LogTuple>
}


///////////////////////////////////////////////////////////////////////////////
//  Object Implementation
///////////////////////////////////////////////////////////////////////////////

impl LogReceiver {
    /// Fully-qualified constructor
    pub fn new(channel_rx: mpsc::Receiver<LogTuple>) -> Self {
        Self {
            channel_rx: channel_rx
        }
    }


    /*  *  *  *  *  *  *\
     * Utility Methods *
    \*  *  *  *  *  *  */
    
    /// Main loop for recieving log messages
    pub fn main(&self) {
        println!("{}: Entered LogReceiver thread.", Local::now().format("%Y-%b-%d %T%.3f"));

        loop {
            if let Ok(log_tuple) = self.channel_rx.recv() {
                // Set label color based on level
                let log_color = match log_tuple.level {
                    LogLevel::TRACE     => "\x1b[030;105m",
                    LogLevel::DEBUG     => "\x1b[030;106m",
                    LogLevel::INFO      => "\x1b[030;107m",
                    LogLevel::WARNING   => "\x1b[030;103m",
                    LogLevel::ERROR     => "\x1b[030;101m",
                    LogLevel::FATAL     => "\x1b[031;040m",
                };

                // Log message to console
                println!(
                    "{timestamp}: {colored_level:<label_width$} {msg}",
                    timestamp       = Local::now().format("%Y-%b-%d %T%.3f"),
                    colored_level   = format!(
                        "{color_set}[{level}]{color_reset}",
                        color_set   = log_color,
                        level       = String::from(log_tuple.level),
                        color_reset = "\x1b[0m",
                    ),
                    msg             = log_tuple.msg,
                    label_width     = PADDING_FOR_LEVEL_LABEL
                );

                //TODO: Log message to file
            }
        }
    }
}



