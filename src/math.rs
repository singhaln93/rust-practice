/**
 * @file math.rs
 * Copyright © 2021  Neeraj Singhal
 * All rights reserved
 */

/**
 * Function used to add two numbers
 */
pub fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

/**
 * Function used to subtract two numbers
 */
pub fn sub(a: i32, b: i32) -> i32 {
    return a - b;
}

/**
 * Function used to multiply two numbers
 */
pub fn mul(a: i32, b: i32) -> i32 {
    return a * b;
}

/**
 * Function used to divide two numbers
 */
pub fn div(a: i32, b: i32) -> i32 {
    return a / b;
}

/**
 * Function used to calculate average of three numbers
 */
pub fn average(a: i32, b: f64, c: f64) -> f64 {
    return (a as f64 + b + c) / 3.0;
}

/**
 * Function used to calculate square root of the number
 */
pub fn sqrt(number: f64) -> Result<f64, String> {
    if number >= 0.0 {
        Ok(number.powf(0.5))
    } else {
        Err("negative floats don't have square roots".to_owned())
    }
}
