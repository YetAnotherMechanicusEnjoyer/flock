use crate::utils::consts::{ABS_ZERO_FAHRENHEIT, ZERO_CELSIUS};

#[inline]
pub fn kelvin_to_celsius(kelvin: f32) -> f32 {
    kelvin - ZERO_CELSIUS
}

#[inline]
pub fn kelvin_to_fahrenheit(kelvin: f32) -> f32 {
    (kelvin * 1.8) + ABS_ZERO_FAHRENHEIT
}

#[inline]
pub fn celsius_to_kelvin(celsius: f32) -> f32 {
    celsius + ZERO_CELSIUS
}

#[inline]
pub fn celsius_to_fahrenheit(celsius: f32) -> f32 {
    (celsius * 1.8) + 32.0
}

#[inline]
pub fn fahrenheit_to_kelvin(fahrenheit: f32) -> f32 {
    (fahrenheit - ABS_ZERO_FAHRENHEIT) / 1.8
}

#[inline]
pub fn fahrenheit_to_celsius(fahrenheit: f32) -> f32 {
    (fahrenheit - 32.0) / 1.8
}
