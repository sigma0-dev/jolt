#![cfg_attr(feature = "guest", no_std)]
#![no_main]

#[jolt::provable]
pub fn correct_factors(p: i32, #[private] a: i32, #[private] b: i32) -> bool {
    p == a * b
}
