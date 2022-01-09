/**
 * @file temp.rs
 * Copyright © 2021  Neeraj Singhal
 * All rights reserved
 */

/**
 * Function used to convert temperature from celsius to fahrenheit
 */
pub fn celsius_to_fahrenheit(temp: f64) -> f64 {
    return (temp * 1.8) + 32.0;
}

/**
 * Function used to convert temperature from fahrenheit to celsius
 */
pub fn fahrenheit_to_celsius(temp: f64) -> f64 {
    return (temp - 32.0) * (5.0 / 9.0);
}
