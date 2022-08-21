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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ut_add() {
        assert_eq!(4, add(2, 2));
    }

    #[test]
    fn ut_sub() {
        assert_eq!(0, sub(2, 2));
    }

    #[test]
    fn ut_mul() {
        assert_eq!(4, mul(2, 2));
    }

    #[test]
    fn ut_div() {
        assert_eq!(1, div(2, 2));
    }

    #[test]
    fn ut_average() {
        assert_eq!(2.0, average(2, 2.0, 2.0));
    }
}
