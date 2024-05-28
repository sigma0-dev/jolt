#![cfg_attr(feature = "guest", no_std)]
#![no_main]

#[jolt::provable]
fn correct_factors(p: i32, a: i32, b: i32) -> bool {
    p == a * b
}
