// file temp.rs
// MIT License
// Copyright (c) 2025 Neeraj Singhal

/// Function used to convert temperature from celsius to fahrenheit
///
/// ## Example:
/// ```rust
/// let celsius = 0;
/// let result = celsius_to_fahrenheit(celsius);
/// assert_eq!(32, result);
/// ```
pub fn celsius_to_fahrenheit(temp: f64) -> f64 {
    (temp * 1.8) + 32.0
}

/// Function used to convert temperature from fahrenheit to celsius
///
/// ## Example:
/// ```rust
/// let fahrenheit = 32;
/// let result = fahrenheit_to_celsius(fahrenheit);
/// assert_eq!(0, result);
/// ```
pub fn fahrenheit_to_celsius(temp: f64) -> f64 {
    (temp - 32.0) * (5.0 / 9.0)
}

#[cfg(test)]
mod tests {
    //! This module contains unit tests for above functions
    use super::*;
    use crate::rand::Rng;

    #[test]
    fn ut_celsius_to_fahrenheit() {
        let celsius = rand::rng().random_range(0.0..1000.0);

        let result = celsius_to_fahrenheit(celsius);
        assert_eq!(result, (celsius * 1.8) + 32.0);
    }

    #[test]
    fn ut_fahrenheit_to_celsius() {
        let fahrenheit = rand::rng().random_range(0.0..1000.0);
        let result = fahrenheit_to_celsius(fahrenheit);
        assert_eq!(result, (fahrenheit - 32.0) * (5.0 / 9.0));
    }
}
