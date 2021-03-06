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

use crate::logger;

use chrono::Local;


///////////////////////////////////////////////////////////////////////////////
//  Named Constants
///////////////////////////////////////////////////////////////////////////////

/// Padding required to align text after logger::FilterLevel label
const LEVEL_LABEL_WIDTH: usize = 9;

/// Padding to the left of the log message
const MESSAGE_LEFT_PADDING: usize = 3;


///////////////////////////////////////////////////////////////////////////////
//  Data Structures
///////////////////////////////////////////////////////////////////////////////

pub struct LogReceiver {
    logger_rx:    mpsc::Receiver<logger::Command>,
    output_type:  logger::OutputType,
}


///////////////////////////////////////////////////////////////////////////////
//  Object Implementation
///////////////////////////////////////////////////////////////////////////////

impl LogReceiver {
    /// Fully-qualified constructor
    pub fn new(logger_rx: mpsc::Receiver<logger::Command>, output_type: logger::OutputType) -> Self {
        Self {logger_rx, output_type}
    }


    /*  *  *  *  *  *  *\
     * Utility Methods *
    \*  *  *  *  *  *  */

    /// Main loop for receiving logger commands
    pub fn main(&mut self) {
        let start_time = Local::now();
        println!("{}: Entered LogReceiver thread.", start_time.format("%Y-%m-%d %T%.3f"));

        // Open a logfile, creating logs directory if necessary
        let logfile_dir = "logs";
        let logfile_name = format!("sandcasting_log_{}.log", start_time.format("%F_%H_%M_%S%.3f"));

        let mut path_buf = PathBuf::from(logfile_dir);
        if !path_buf.as_path().exists() {
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
            // Check the channel for commands
            if let Ok(logger_cmd) = self.logger_rx.recv() {
                let timestamp = Local::now().format("%Y-%m-%d %T%.3f");

                // Handle command based on type
                match logger_cmd {
                    // Log a message
                    logger::Command::LogMsg(log_tuple) => {
                        // Console output
                        if self.output_type as u8 & logger::OutputType::Console as u8 != 0 {
                            let log_color = match log_tuple.level {
                                logger::FilterLevel::Trace     => "\x1b[030;105m",
                                logger::FilterLevel::Debug     => "\x1b[030;106m",
                                logger::FilterLevel::Info      => "\x1b[030;107m",
                                logger::FilterLevel::Warning   => "\x1b[030;103m",
                                logger::FilterLevel::Error     => "\x1b[030;101m",
                                logger::FilterLevel::Fatal     => "\x1b[031;040m",
                            };
                            println!(
                                "{timestamp}: {color_set}[{level:^level_width$}]\x1b[0m {fn_name}() line {line}:\n{msg:>msg_leftpad$}",
                                timestamp   = timestamp,
                                color_set   = log_color,
                                level       = String::from(log_tuple.level),
                                level_width = LEVEL_LABEL_WIDTH,
                                fn_name     = log_tuple.fn_name,
                                line        = log_tuple.line,
                                msg         = log_tuple.msg,
                                msg_leftpad = MESSAGE_LEFT_PADDING + log_tuple.msg.len(),
                            );
                        }

                        // File output
                        if self.output_type as u8 & logger::OutputType::File as u8 != 0 {
                            let msg_formatted = format!(
                                "{timestamp}: [{level:^level_width$}] {fn_name}() line {line}:\n{msg:>msg_leftpad$}\n",
                                timestamp   = timestamp,
                                level       = String::from(log_tuple.level),
                                level_width = LEVEL_LABEL_WIDTH,
                                fn_name     = log_tuple.fn_name,
                                line        = log_tuple.line,
                                msg         = log_tuple.msg,
                                msg_leftpad = MESSAGE_LEFT_PADDING + log_tuple.msg.len(),
                            );
                            logfile.write_all(msg_formatted.as_bytes()).unwrap();
                        }
                    },

                    logger::Command::SetOutput(output_type) => {
                        self.output_type = output_type;
                    },
                };
            }
        }
    }
}
