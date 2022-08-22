// file file.rs
// MIT License
// Copyright (c) 2021 Neeraj Singhal

use std::{error, result};

type TResult<T> = result::Result<T, TError>;
type TError = Box<dyn error::Error>;

/// Read in the file as String
pub fn read_file(_p: &str) -> TResult<String> {
    unimplemented!();
}

// Covert String to a Vector of Numbers
// Sum all of the Numbers together
// Write the Sum back into the file
