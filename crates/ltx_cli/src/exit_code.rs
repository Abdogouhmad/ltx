//! Process exit codes for the `ltx` CLI.
#![allow(dead_code)]

/// Successful operation.
pub const SUCCESS: u8 = 0;

/// A general internal error occurred.
pub const ERROR: u8 = 1;

/// The input file was not found.
pub const FILE_NOT_FOUND: u8 = 2;

/// The input is not valid UTF-8.
pub const INVALID_INPUT: u8 = 3;

/// Diagnostics (errors or warnings) were found during checking.
pub const DIAGNOSTICS_FOUND: u8 = 4;
