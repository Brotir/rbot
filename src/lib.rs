pub mod constants;
pub mod conversions;
pub mod core;
pub mod errors;
pub mod hostfn;
pub mod modules;
pub mod rotations;
use crate as bot;
pub use bot::core::*;
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
    bot::core::print("Hello World");
    bot::core::sleep(100.);
    bot::core::use_component(0, false).unwrap();
}
