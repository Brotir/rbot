use rbot_messages::messages as msg;
use rbot_messages::{Message, MessageIdentity, MessageType};
use serde::Serialize;

extern "C" {
    /// Sends bytes over TCP to the game server and returns a response.
    ///
    /// This function sends a byte array (`ptr_void`) of a specified size
    /// (`size`) over TCP to the game server and waits for a response. The
    /// response is represented by a pointer (`i32`) to an array of three
    /// integers: `[type, size, bytes]`, where `type` indicates the response
    /// type, `size` specifies the response size, and `bytes` the response data.
    ///
    /// It is recommended for users to use the `send_message` function instead,
    /// which provides a higher-level interface for message passing using
    /// predefined message types and handles the communication with the server.
    ///
    /// # Arguments
    ///
    /// * `ptr_void` - Pointer to the byte array (`i32`) containing the data to be sent.
    /// * `size` - Size of the byte array in bytes.
    ///
    /// # Returns
    ///
    /// A pointer (`i32`) to an array `[type, size, bytes]` representing the
    /// response from the game server.
    ///
    /// # Note
    ///
    /// Users are encouraged to use `send_message` for message passing, as it provides a safer and more
    /// abstracted way of interacting with the game server using predefined message types.
    pub fn tcp_send(ptr_void: i32, size: i32) -> i32;

    /// Writes a string to a log file while in programming mode.
    ///
    /// This function is designed for debugging purposes and allows writing a
    /// string to a log file while the robot is in programming mode. However,
    /// this functionality is disabled once the code is deployed.
    ///
    /// # Arguments
    ///
    /// * `bytes_ptr` - The pointer to the memory location containing the string data.
    /// * `size` - The size of the string data in bytes.
    ///
    /// # Note
    ///
    /// This function is only functional during programming mode and has no
    /// effect once the code is deployed.
    pub fn dbg_log(bytes_ptr: i32, size: i32);

    /// Blocks the thread and put it to sleep for x milliseconds
    pub fn bot_sleep(seconds: f32);

    /// Generates a random floating-point value between 0 (inclusive) and 1 (inclusive).
    ///
    /// This function returns a pseudo-random number within the range [0, 1],
    /// suitable for use in generating other random number sequences. In the
    /// sandbox environment where the robot operates, direct access to the
    /// underlying processor for random number generation is restricted due to
    /// safety concerns.
    ///
    /// # Returns
    ///
    /// A random `f32` value between 0 (inclusive) and 1 (inclusive).
    pub fn random() -> f32;
}

/// Sends a message to the game server over TCP using the R-Protocol [TYPE, SIZE, BYTES].
///
/// This function sends a message to the game server using the R-Protocol, which
/// consists of message serialization into JSON format and subsequent
/// transmission over TCP. Each message exchange involves sending a message and
/// receiving a response from the server.
///
/// # Arguments
///
/// * `msg` - A reference to the message (`M`) that implements `Message`, `MessageIdentity`, and `Serialize`.
///
/// # Returns
///
/// The `MessageType` representing the response type received from the server.
///
/// # Note
///
/// The data is currently serialized using JSON encoding but may be subject to change
/// for faster serialization methods in future implementations.
pub fn send_message<M: Message + MessageIdentity + Serialize>(msg: &M) -> MessageType {
    unsafe {
        // Send message byte
        // [Type Size Bytes]
        let byte_msg = rbot_messages::serialize_message(msg).unwrap();
        let result_ptr = tcp_send(
            std::ptr::addr_of!(byte_msg[0]) as i32,
            byte_msg.len() as i32,
        );

        // Read Result
        let [typ, size, res_ptr] = *(result_ptr as *const [i32; 3]);
        let bytes = Vec::from_raw_parts(res_ptr as *mut u8, size as usize, size as usize);
        msg::decode_message(&bytes, typ).unwrap()
    }
}
