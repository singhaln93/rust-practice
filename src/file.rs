/**
 * @file file.rs
 * Copyright © 2021  Neeraj Singhal
 * All rights reserved
 */
use std::{
    error,
    fs::{read_to_string, write},
    path::Path,
    result,
};

type TResult<T> = result::Result<T, TError>;
type TError = Box<dyn error::Error>;
/**
 * Read in the file as String
 */
pub fn read_file(p: &str) -> TResult<String> {
    unimplemented!();
}

// Covert String to a Vector of Numbers
// Sum all of the Numbers together
// Write the Sum back into the file
