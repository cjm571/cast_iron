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
    pub level:      LogLevel,
    pub fn_name:    String,
    pub line:       u32,
    pub msg:        String,
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

    pub fn log_msg(
        &self,
        level: LogLevel,
        fn_name: String,
        line: u32,
        msg: String) -> Result<(), SendError<LogTuple>> {
        //OPT: *DESIGN* Proper filter masking instead of greater-than check
        // Check filter and send message if it passes
        if level as u8 >= self.filter {
            // Package log message into tuple and send
            let log_tuple = LogTuple {
                level:      level,
                fn_name:    fn_name,
                line:       line,
                msg:        msg,
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

        // Initialize receiver struct, build and spawn thread
        let log_receiver = LogReceiver::new(receiver);
        thread::Builder::new()
            .name("log_receiver".to_owned())
            .spawn(move || log_receiver.main())
            .unwrap();

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

//OPT: *PERFORMANCE* Are the string type conversions expensive?
#[macro_export]
macro_rules! ci_log {
    ($logger_instance:expr, $log_level:expr, $( $fmt_args:expr ),*) => {
        let fn_name = {
            fn f() {}
            fn type_name_of<T>(_: T) -> &'static str {
                std::any::type_name::<T>()
            }
            let name = type_name_of(f);
            &name[..name.len() - 3]
        };

        let msg_content: String = format!($( $fmt_args ),*);

        $logger_instance.log_msg($log_level, fn_name.to_owned(), line!(), msg_content).unwrap();
    };
}


///////////////////////////////////////////////////////////////////////////////
//  Unit Tests
///////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;
    use std::{thread, time};

    #[test]
    fn visual_verification() {
        // Create a logger instance that will log all messsages
        let logger = LoggerInstance::new(LogLevel::TRACE as u8);

        ci_log!(&logger, LogLevel::TRACE,   "This is a TRACE message.");
        ci_log!(&logger, LogLevel::DEBUG,   "This is a DEBUG message.");
        ci_log!(&logger, LogLevel::INFO,    "This is an INFO message.");
        ci_log!(&logger, LogLevel::WARNING, "This is a WARNING message.");
        ci_log!(&logger, LogLevel::ERROR,   "This is an ERROR message.");
        ci_log!(&logger, LogLevel::FATAL,   "This is a FATAL message.");

        // Sleep for 5 seconds to allow other thread to do stuff
        println!("Sleeping for 5s...");
        thread::sleep(time::Duration::from_secs(5));
        println!("Done sleeping!");
    }
}