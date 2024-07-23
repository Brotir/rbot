//! Here’s a simple example to get you started. This program makes your robot
//! move upward, uses the radar module to locate an enemy robot, and then fires
//! a component towards it. The `#![allow(unused_must_use)]` attribute is used to
//! suppress warnings about unused results from certain function calls.
//!
//! ```rust
//! // usr_main.rs
//! #![allow(unused_must_use)]
//! use rbot;
//!
//! pub fn main() {
//!     // Move the robot upward with the specified velocity
//!     rbot::velocity(0.0, 1.0, 1.0);
//!
//!     // Wait for the Radar module to become available
//!     rbot::modules::await_module(rbot::modules::Module::Radar);
//!
//!     // Retrieve the radar message, which contains the position of the enemy robot
//!     let radar_msg = rbot::modules::radar().expect("failed to get radar message");
//!
//!     // Convert the enemy's position (x, y coordinates) to an angle
//!     let angle = rbot::conversions::xy_to_angle(radar_msg.x, radar_msg.y);
//!
//!     // Aim at the enemy robot using the calculated angle
//!     rbot::await_aim(2, angle, 0.5);
//!
//!     // Wait for the component to be ready
//!     rbot::await_component(2);
//!
//!     // Fire the component towards the enemy robot
//!     rbot::use_component(2, false);
//! }
//! ```
//!
//! For more detailed documentation, please visit our Discord channel,
//! accessible through our website:
//! [https://botbeats.net](https://botbeats.net). If you have any questions,
//! feel free to reach out to us on Discord.

pub mod constants;
pub mod conversions;
pub mod core;
pub mod errors;
pub mod hostfn;
pub mod modules;
pub mod print_macros;
pub mod rotations;
pub use crate::core::*;
pub use rbot_messages::messages;

/// Allocates a memory buffer in the robot's sandbox environment for storing
/// data from the host environment.
///
/// This function provides an interface for the host environment to allocate a
/// memory buffer within the robot's sandbox environment. The allocated buffer
/// is initialized with zeros and can be used to store data passed from the host
/// environment to the robot's runtime.
///
/// # Safety
///
/// This function is marked as `unsafe` because it involves direct manipulation
/// of memory pointers and relies on external interactions with the robot's
/// sandbox environment.
///
/// # Arguments
///
/// * `size` - The size of the memory buffer to allocate in bytes.
///
/// # Returns
///
/// An integer representing the pointer to the allocated memory buffer within
/// the robot's sandbox environment.
#[no_mangle]
pub extern "C" fn alloc_wasm(size: i32) -> i32 {
    let buf: Vec<u8> = vec![0; size as usize];
    let raw_p_buf = std::ptr::addr_of!(buf[0]);
    let p_buf = raw_p_buf as i32;
    std::mem::forget(buf);
    p_buf
}

/// Prevents Rust compiler optimizations that could hinder robot booting.
#[no_mangle]
pub extern "C" fn dummy() {
    core::print("Hello World");
    core::sleep(100.);
    core::use_component(0, false).unwrap();
}
