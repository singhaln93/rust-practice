/**
 * @file main.rs
 * Copyright © 2021  Neeraj Singhal
 * All rights reserved
 */
extern crate rand;

use rand::Rng;
use std::io;
mod math;
mod temp;

fn main() {
    println!("Hello World!\nI'm a Rustacean!\n");
    let x: i8 = -9;
    println!("Result= {}", x);

    let secret_number = rand::thread_rng().gen_range(1, 100);
    println!("The secret number= {}", secret_number);

    let mut a = String::new();
    io::stdin().read_line(&mut a).expect("failed to read line");

    let a = 2;
    let b = 9;
    let sum = math::add(a, b);
    println!("Result {} + {} = {}", a, b, sum);
    let sub = math::sub(a, b);
    println!("Result {} - {} = {}", a, b, sub);
    let mul = math::mul(a, b);
    println!("Result {} * {} = {}", a, b, mul);

    let average = math::average(13, 2.3, 120.0);
    assert_eq!(average, 45.1);
    println!("Test# {} Passed!", std::stringify!(average));

    let celsius_temp = 23.0;
    let fahrenheit_temp = temp::celsius_to_fahrenheit(celsius_temp);
    assert_eq!(fahrenheit_temp, 73.4);
    println!("Test# {} Passed!", std::stringify!(celsius_to_fahrenheit));

    let fahrenheit_temp = 73.4;
    let celsius_temp = temp::fahrenheit_to_celsius(fahrenheit_temp);
    assert_eq!(celsius_temp, 23.0);
    println!("Test# {} Passed!", std::stringify!(fahrenheit_to_celsius));
}
