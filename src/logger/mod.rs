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
    This module will provide data structures and functions that provide
    1st-party logging functionality for game events.

!!!USAGE NOTE!!!
    This module is meant to be created once in a top level, and then cloned
    in each submodule's constructors from a reference to the original.

    Due to the nature of Rusts' "multiple producer, single consumer" model
    of inter-thread communication, all clones will send their messages to
    the single reciever spawned by the original Instance.

\* * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * * */

use std::sync::mpsc::{
    self,
    SendError
};
use std::thread;

use crate::Disableable;

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
#[derive(Debug, Copy, Clone)]
pub enum FilterLevel {
    Trace   = 0x01,
    Debug   = 0x02,
    Info    = 0x04,
    Warning = 0x08,
    Error   = 0x10,
    Fatal   = 0x20,
}

/// Tuple struct containing log message and its log level
pub struct MsgTuple {
    pub level:      FilterLevel,
    pub fn_name:    String,
    pub line:       u32,
    pub msg:        String,
}

#[derive(Debug, Copy, Clone)]
pub enum OutputType {
    Neither = 0x0,
    Console = 0x1,
    File    = 0x2,
    Both    = 0x3,
}

pub enum Command {
    LogMsg(MsgTuple),
    SetOutput(OutputType)
}

#[derive(Clone)]
pub struct Instance {
    enabled:    bool,
    sender:     LogSender,
    filter:     u8
}


///////////////////////////////////////////////////////////////////////////////
//  Object Implementation
///////////////////////////////////////////////////////////////////////////////

impl Instance {
    /// Fully-qualified constructor
    pub fn new(filter: u8, output_type: OutputType) -> Self {
        let mut logger_instance = Instance::default();
        logger_instance.set_filter(filter);
        
        logger_instance.log_cmd(Command::SetOutput(output_type)).unwrap();

        logger_instance
    }

    /// Default constructor for debugging
    pub fn debug_default() -> Self {
        let mut logger_instance = Instance::default();
        logger_instance.set_filter(FilterLevel::Debug as u8);
        logger_instance.log_cmd(Command::SetOutput(OutputType::Both)).unwrap();

        logger_instance
    }

    
    /*  *  *  *  *  *  *  *
     *  Accessor Methods  *
     *  *  *  *  *  *  *  */

    pub fn filter(&self) -> u8 {
        self.filter
    }


    /*  *  *  *  *  *  *  *
     *  Mutator Methods   *
     *  *  *  *  *  *  *  */

    pub fn set_filter(&mut self, new_filter: u8) {
        self.filter = new_filter;
    }
    
    /// Disables the logger instance
    pub fn disable(&mut self) {
        self.enabled = false;
    }


    /*  *  *  *  *  *  *  *
     *  Utility Methods   *
     *  *  *  *  *  *  *  */

    pub fn log_msg(&self,
                   level: FilterLevel,
                   fn_name: String,
                   line: u32,
                   msg: String) -> Result<(), SendError<Command>> {
        // Check filter and send message if it passes
        if self.enabled && level as u8 >= self.filter {
            // Package log message into tuple and send
            let log_tuple = MsgTuple {
                level,
                fn_name,
                line,
                msg,
            };
            self.sender.send_log(Command::LogMsg(log_tuple))
        } else {
            Ok(())
        }
    }

    pub fn log_cmd(&self, cmd: Command) -> Result<(), SendError<Command>> {
        if self.enabled {
            self.sender.send_cmd(cmd)
        }
        else {
            Ok(())
        }
    }
}




///////////////////////////////////////////////////////////////////////////////
//  Trait Implementations
///////////////////////////////////////////////////////////////////////////////

impl Default for Instance {
    fn default() -> Self {
        // Create the log messaging and control channel
        let (logger_tx, logger_rx) = mpsc::channel::<Command>();

        //OPT: *PERFORMANCE* Would be better to set the receiver thread's priority as low as possible
        // Initialize receiver struct, build and spawn thread
        let mut log_receiver = LogReceiver::new(logger_rx, OutputType::Both);
        thread::Builder::new()
            .name("log_receiver".to_owned())
            .spawn(move || log_receiver.main())
            .unwrap();

        // Initialize sender struct
        let log_sender = LogSender::new(logger_tx);

        Self {
            enabled:    true,
            sender:     log_sender,
            filter:     FilterLevel::Info as u8
        }
    }
}

impl Disableable for Instance {
    fn disabled() -> Self {
        // Create dummy channel handles
        let (dummy_tx, _dummy_rx) = mpsc::channel::<Command>();
        
        // Initialize dummy sender struct
        let dummy_sender = LogSender::new(dummy_tx);

        Self {
            enabled:    false,
            sender:     dummy_sender,
            filter:     FilterLevel::Fatal as u8,
        }
    }
}

impl From<FilterLevel> for String {
    fn from(src: FilterLevel) -> Self {
        match src {
            FilterLevel::Trace     => "TRACE".to_owned(),
            FilterLevel::Debug     => "DEBUG".to_owned(),
            FilterLevel::Info      => "INFO".to_owned(),
            FilterLevel::Warning   => "WARNING".to_owned(),
            FilterLevel::Error     => "ERROR".to_owned(),
            FilterLevel::Fatal     => "FATAL".to_owned(),
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
        // Create a logger instance that will log all messsages to Both outputs
        let logger = Instance::new(FilterLevel::Trace as u8, OutputType::Both);

        ci_log!(&logger, FilterLevel::Trace,   "This is a TRACE message.");
        ci_log!(&logger, FilterLevel::Debug,   "This is a DEBUG message.");
        ci_log!(&logger, FilterLevel::Info,    "This is an INFO message.");
        ci_log!(&logger, FilterLevel::Warning, "This is a WARNING message.");
        ci_log!(&logger, FilterLevel::Error,   "This is an ERROR message.");
        ci_log!(&logger, FilterLevel::Fatal,   "This is a FATAL message.");

        // Sleep for 5 seconds to allow the reciever thread to do stuff
        println!("Sleeping for 5s...");
        thread::sleep(time::Duration::from_secs(5));
        println!("Done sleeping!");
    }

    #[test]
    fn output_type_cmd_test() {
        // Create a logger instance that will log messsages to BOTH outputs
        let logger = Instance::new(FilterLevel::Trace as u8, OutputType::Both);

        ci_log!(&logger, FilterLevel::Trace, "This message appears in BOTH console and file.");
        ci_log!(&logger, FilterLevel::Fatal, "This message appears in BOTH console and file.");

        // Log messages to CONSOLE only
        logger.log_cmd(Command::SetOutput(OutputType::Console)).unwrap();
        ci_log!(&logger, FilterLevel::Trace, "This message appears in CONSOLE ONLY.");
        ci_log!(&logger, FilterLevel::Fatal, "This message appears in CONSOLE ONLY.");

        // Log messages to FILE only
        logger.log_cmd(Command::SetOutput(OutputType::File)).unwrap();
        ci_log!(&logger, FilterLevel::Trace, "This message appears in FILE ONLY.");
        ci_log!(&logger, FilterLevel::Fatal, "This message appears in FILE ONLY.");

        // Log messages to NEITHER output
        logger.log_cmd(Command::SetOutput(OutputType::Neither)).unwrap();
        ci_log!(&logger, FilterLevel::Trace, "This message appears in NEITHER ONLY.");
        ci_log!(&logger, FilterLevel::Fatal, "This message appears in NEITHER ONLY.");

        // Sleep for 5 seconds to allow the reciever thread to do stuff
        println!("Sleeping for 5s...");
        thread::sleep(time::Duration::from_secs(5));
        println!("Done sleeping!");
    }
}
