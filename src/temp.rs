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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rand::thread_rng;
    use crate::rand::Rng;

    #[test]
    fn ut_celsius_to_fahrenheit() {
        let mut rng = thread_rng();
        let celsius = rng.gen_range(0.0..1000.0);

        let result = celsius_to_fahrenheit(celsius);
        assert_eq!(result, (celsius * 1.8) + 32.0);
    }

    #[test]
    fn ut_fahrenheit_to_celsius() {
        let mut rng = thread_rng();
        let fahrenheit = rng.gen_range(0.0..1000.0);

        let result = fahrenheit_to_celsius(fahrenheit);
        assert_eq!(result, (fahrenheit - 32.0) * (5.0 / 9.0));
    }
}
