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

use std::fs;
use std::path::PathBuf;
use std::io::prelude::*;

use crate::logger::{
    LogLevel,
    LogTuple
};

use chrono::Local;


///////////////////////////////////////////////////////////////////////////////
//  Named Constants
///////////////////////////////////////////////////////////////////////////////

/// Padding required to align text after LogLevel label
const LEVEL_LABEL_WIDTH: usize = 9;

/// Padding to the left of the log message
const MESSAGE_LEFT_PADDING: usize = 3;


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
        let start_time = Local::now();
        println!("{}: Entered LogReceiver thread.", start_time.format("%Y-%b-%d %T%.3f"));

        // Open a logfile, creating logs directory if necessary
        let logfile_dir = "logs";
        let logfile_name = format!("sandcasting_log_{}.log", start_time.format("%F_%H_%M_%S%.3f"));

        let mut path_buf = PathBuf::from(logfile_dir);
        if path_buf.as_path().exists() == false {
            match fs::create_dir(path_buf.as_path()) {
                Ok(()) => (),
                Err(e) => panic!("Failed to create logs directory. Error: {}", e),
            }
        }

        path_buf.push(logfile_name);
        let mut logfile = match fs::File::create(path_buf.as_path()) {
            Ok(file) => file,
            Err(err) => panic!("Failed to open logfile at {}. Error: {}", path_buf.as_path().display(), err),
        };

        loop {
            if let Ok(log_tuple) = self.channel_rx.recv() {
                let timestamp = Local::now().format("%Y-%b-%d %T%.3f");

                // Format for console output
                let log_color = match log_tuple.level {
                    LogLevel::TRACE     => "\x1b[030;105m",
                    LogLevel::DEBUG     => "\x1b[030;106m",
                    LogLevel::INFO      => "\x1b[030;107m",
                    LogLevel::WARNING   => "\x1b[030;103m",
                    LogLevel::ERROR     => "\x1b[030;101m",
                    LogLevel::FATAL     => "\x1b[031;040m",
                };
                let msg_formatted_console = format!(
                    "{timestamp}: {color_set}[{level:^level_width$}]{color_reset} {fn_name}: Line {line}:\n{msg:>msg_leftpad$}",
                    timestamp   = timestamp,
                    color_set   = log_color,
                    level       = String::from(log_tuple.level),
                    level_width = LEVEL_LABEL_WIDTH,
                    color_reset = "\x1b[0m",
                    fn_name     = log_tuple.fn_name,
                    line        = log_tuple.line,
                    msg         = log_tuple.msg,
                    msg_leftpad = MESSAGE_LEFT_PADDING + log_tuple.msg.len(),
                );
                println!("{}", msg_formatted_console);

                // Format for file output
                let msg_formatted_file = format!(
                    "{timestamp}: [{level:^level_width$}] {fn_name}: Line {line}:\n{msg:>msg_leftpad$}\n",
                    timestamp   = timestamp,
                    level       = String::from(log_tuple.level),
                    level_width = LEVEL_LABEL_WIDTH,
                    fn_name     = log_tuple.fn_name,
                    line        = log_tuple.line,
                    msg         = log_tuple.msg,
                    msg_leftpad = MESSAGE_LEFT_PADDING + log_tuple.msg.len(),
                );
                logfile.write_all(msg_formatted_file.as_bytes()).unwrap();
            }
        }
    }
}
