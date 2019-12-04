//=============================================================================
// debug.rs
// Methods and macros for handling debug messages from the engine and application
//
// Created by Victor on 2019/10/24
//=============================================================================

use log::{debug, error, info, trace, warn};

pub fn log(s: &str) {
    //println!("{}", s); // TODO: We should log to a file, etc
    info!("{}", s);
}

pub enum LogType {
    Error,
    Warning,
    Info
}
