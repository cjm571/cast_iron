/* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * *\
Filename : logger/mod.rs

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
    This module will provide data structures and functions for that provide
    1st-party logging functionality for game events.

!!!USAGE NOTE!!!
    This module is meant to be created once in a top level, and then cloned
    in each submodule's constructors from a reference to the original.

    Due to the nature of Rusts' "multiple producer, single consumer" model
    of inter-thread communication, all clones will send their messages to
    the single reciever spawned by the original LoggerInstance.

\* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * */

use std::sync::mpsc::{
    self,
    SendError
};
use std::thread;


///////////////////////////////////////////////////////////////////////////////
//  Module Declarations
///////////////////////////////////////////////////////////////////////////////

pub mod log_sender;
use self::log_sender::LogSender;
pub mod log_receiver;
use self::log_receiver::LogReceiver;


///////////////////////////////////////////////////////////////////////////////
//  Named Constants
///////////////////////////////////////////////////////////////////////////////

/// Padding required to align text after LogLevel label
const PADDING_FOR_LEVEL_LABEL: usize = 24;


///////////////////////////////////////////////////////////////////////////////
//  Data Structures
///////////////////////////////////////////////////////////////////////////////

/// Denotes the level or severity of the log message.
#[derive(Copy, Clone)]
pub enum LogLevel {
    TRACE   = 0x01,
    DEBUG   = 0x02,
    INFO    = 0x04,
    WARNING = 0x08,
    ERROR   = 0x10,
    FATAL   = 0x20,
}

/// Tuple struct containing log message and its log level
pub struct LogTuple {
    pub msg:    String,
    pub level:  LogLevel
}

#[derive(Clone)]
pub struct LoggerInstance {
    sender: LogSender,
    filter: u8
}


///////////////////////////////////////////////////////////////////////////////
//  Object Implementation
///////////////////////////////////////////////////////////////////////////////

impl LoggerInstance {
    pub fn new(filter: u8) -> Self {
        let mut logger_instance = LoggerInstance::default();
        logger_instance.set_filter(filter);

        logger_instance
    }


    /* Accessor Methods */

    pub fn get_filter(&self) -> u8 {
        self.filter
    }


    /* Mutator Methods */

    pub fn set_filter(&mut self, new_filter: u8) {
        self.filter = new_filter;
    }


    /* Utility Methods */

    pub fn log_msg(&self, level: LogLevel, msg: String) -> Result<(), SendError<LogTuple>> {
        //OPT: *DESIGN* Proper filter masking instead of greater-than check
        // Check filter and send message if it passes
        if level as u8 >= self.filter {
            // Package log message into tuple
            let log_tuple = LogTuple {
                msg:    msg,
                level:  level
            };
            self.sender.send_log(log_tuple)
        }
        else
        {
            Ok(())
        }
    }
}




///////////////////////////////////////////////////////////////////////////////
//  Trait Implementations
///////////////////////////////////////////////////////////////////////////////

impl Default for LoggerInstance {
    fn default() -> Self {
        // Create the channel between log sender and reciever
        let (sender, receiver) = mpsc::channel::<LogTuple>();

        // Initialize receiver struct and spawn thread
        let log_receiver = LogReceiver::new(receiver);
        thread::spawn(move || log_receiver.main());

        // Initialize sender struct
        let log_sender = LogSender::new(sender);

        Self {
            sender: log_sender,
            filter: LogLevel::INFO as u8
        }
    }
}

impl From<LogLevel> for String {
    fn from(src: LogLevel) -> Self {
        match src {
            LogLevel::TRACE   => String::from("TRACE"),
            LogLevel::DEBUG   => String::from("DEBUG"),
            LogLevel::INFO    => String::from("INFO"),
            LogLevel::WARNING => String::from("WARNING"),
            LogLevel::ERROR   => String::from("ERROR"),
            LogLevel::FATAL   => String::from("FATAL")
        }
    }
}



///////////////////////////////////////////////////////////////////////////////
//  Macro Definitions
///////////////////////////////////////////////////////////////////////////////

#[macro_export]
macro_rules! ci_log {
    ($logger_instance:expr, $log_level:expr, $( $fmt_args:expr ),*) => {
        let padding_len = 38; // PADDING_FOR_LEVEL_LABEL + 13 to align message with label
        
        let fn_name = {
            fn f() {}
            fn type_name_of<T>(_: T) -> &'static str {
                std::any::type_name::<T>()
            }
            let name = type_name_of(f);
            &name[..name.len() - 3]
        };
        let content: String = format!($( $fmt_args ),*);

        $logger_instance.log_msg(
            $log_level,
            format!(
                "\x1b[030;100m{fn_name}() Line: {line}:\x1b[0m\n{content:>padded_content_width$}",
                fn_name = fn_name,
                line    = line!(),
                content = content,
                padded_content_width=content.len() + padding_len
            )
        ).unwrap();
    };
}
