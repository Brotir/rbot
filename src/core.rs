use crate::errors::MessageError;
use crate::hostfn;
use crate::rotations;
use crate::rotations::transform_rotation_to_component;
use rbot_messages::messages as msg;
use rbot_messages::MessageType;

/// Fires the component with the specified `component_id`.
///
/// If `sticky` is `true`, it will keep firing the component whenever the cooldown is ready.
/// Otherwise, it will fire the component once.
///
/// # Arguments
///
/// * `component_id` - The identifier of the component to fire.
/// * `sticky` - A flag indicating whether to continuously fire the component (`true`) or fire it once (`false`).
///
/// # Returns
///
/// Whether or not the command sent was recived successfully or not.
///
/// # Examples
///
/// ```
/// // Trigger the first component to fire (if cooldown is ready).
/// let result = rbot::use_component(0, false);
///
/// // If something went wrong in the communication with the server, print it
/// // out to the game console.
/// if result.err {
///     rbot::print("Failed communicating with the server when calling `use_component`.")
/// }
/// ```
pub fn use_component(component_id: i32, sticky: bool) -> Result<(), MessageError> {
    let sticky = match sticky {
        false => 0,
        _ => 1,
    };
    let msg_use = msg::MsgUse {
        component_id,
        sticky,
    };
    let response = hostfn::send_message(&msg_use);

    match response {
        MessageType::Error(m) => Err(MessageError::BadCommand(m.error_code)),
        _ => Ok(()),
    }
}

/// Sets the velocity and direction for the robot's traversal.
///
/// This function specifies the direction and speed at which the robot should
/// move. The `x` and `y` parameters represent components of a 2D vector that
/// determines the direction of movement. These values will be normalized to
/// create a unit vector pointing in the specified direction.
///
/// The `speed` parameter is a decimal number between 0 and 1, indicating the
/// magnitude of the velocity vector. A `speed` value of 0 corresponds to no
/// movement, while a `speed` value of 1 represents maximum speed in the
/// specified direction. The speed will be clamped between 0 and 1 on the game
/// server.
///
/// # Arguments
///
/// * `x` - The X-component of the direction vector.
/// * `y` - The Y-component of the direction vector.
/// * `speed` - The speed of traversal (between 0 and 1).
///
/// # Returns
///
/// Returns `Ok(())` if the velocity command was sent successfully.
///
/// # Examples
///
/// ```
/// let result = rbot::velocity(1.0, 0.5, 0.8);
///
/// // If something went wrong in the communication with the server, print it
/// // out to the game console.
/// if result.err {
///     rbot::print("Failed communicating with the server when calling `velocity`.")
/// }
/// ```
pub fn velocity(x: f32, y: f32, speed: f32) -> Result<(), MessageError> {
    let msg_use = msg::MsgVelocity { x, y, speed };
    let response = hostfn::send_message(&msg_use);
    match response {
        MessageType::Error(m) => Err(MessageError::BadCommand(m.error_code)),
        _ => Ok(()),
    }
}

/// Rotates the robot to a specified angle in degrees.
///
/// This function rotates the robot to the desired angle measured in degrees.
/// It is generally discouraged to call this function frequently with small rotation
/// updates, as it may reduce the speed and efficiency of the robot's rotation.
///
/// # Arguments
///
/// * `angle` - The target angle in degrees to which the robot should rotate.
///
/// # Returns
///
/// Returns `Ok(())` if the rotation command was sent successfully.
///
/// # Examples
///
/// ```
/// let result = rbot::rotate(90.0);
///
/// // If something went wrong in the communication with the server, print it
/// // out to the game console.
/// if result.err {
///     rbot::print("Failed communicating with the server when calling `use_component`.")
/// }
/// ```
pub fn rotate(angle: f32) -> Result<(), MessageError> {
    let msg_use = msg::MsgAngle { angle };
    let response = hostfn::send_message(&msg_use);
    match response {
        MessageType::Error(m) => Err(MessageError::BadCommand(m.error_code)),
        _ => Ok(()),
    }
}

/// Checks if the robot is at a specified rotation angle within a tolerance range.
///
/// This function determines whether the robot's current rotation angle matches
/// the target `angle` within a specified tolerance `slack`. The `slack` parameter
/// allows for a larger offset from the target angle, enabling flexibility in the
/// angle comparison.
///
/// A recommended value for `slack` is 0.5 degrees if you want to consider the robot
/// at the desired angle within a small tolerance.
///
/// # Arguments
///
/// * `component_id` - The identifier of the robot component to check.
/// * `angle` - The target rotation angle in degrees.
/// * `slack` - The allowed tolerance (slack) in degrees for the angle comparison.
///
/// # Returns
///
/// Returns `true` if the robot's rotation angle is within the target angle ±
/// `slack`, otherwise returns `false`. If any communication to the game server
/// fails the function returns an MessageError.
///
/// # Examples
///
/// ```
/// let is_at_angle = at_rotation(0, 90, 0.5)?;
/// rbot::print(&format!("Component 0 is at angle 90 degrees: {is_at_angle}"));
/// ```
pub fn at_rotation(component_id: i32, angle: f32, slack: f32) -> Result<bool, MessageError> {
    let target_angle = rotations::transform_rotation_to_component(component_id, angle);
    let current_rotation = state()?.angle;
    let angle_difference = rotations::angle_distance(target_angle, current_rotation);
    Ok(angle_difference < slack)
}

/// Aims a robot component towards a specified angle in a 2D coordinate system.
///
/// This function aims the specified robot `component_id` towards the given `angle`
/// within a 2D coordinate system where:
///
/// - The origin (0,0) is at the center of the map.
/// - Y-axis points upwards.
/// - X-axis points to the right.
/// - Angle measurement:
///   - 0 degrees: Towards the right (positive X-axis).
///   - 90 degrees: Towards the top (positive Y-axis).
///   - 180 degrees: Towards the left (negative X-axis).
///   - 270 degrees: Towards the bottom (negative Y-axis).
///
/// # Arguments
///
/// * `component_id` - The identifier of the robot component to aim.
/// * `angle` - The target angle in degrees (0 to 360), where 0 degrees points towards the right.
///
/// # Returns
///
/// Returns `Ok(())` if the aiming operation is successful, or an error of type
/// `MessageError` if there was a problem during communication with the game
/// server.
///
/// # Examples
///
/// ```
/// let result = rbot::aim(0, 90);
///
/// // If something went wrong in the communication with the server, print it
/// // out to the game console.
/// if result.err {
///     rbot::print("Failed communicating with the game server.")
/// }
/// ```
pub fn aim(component_id: i32, angle: f32) -> Result<(), MessageError> {
    rotate(transform_rotation_to_component(component_id, angle))
}

/// Aims the robot component towards the specified angle and waits for the
/// aiming process to complete within a tolerance range.
///
/// This function blocks the execution of code until the specified
/// `component_id` is aimed towards the target `angle` within the specified
/// `slack` tolerance range.
///
/// # Arguments
///
/// * `component_id` - The identifier of the robot component to aim.
/// * `angle` - The target angle in degrees (0 to 360), where 0 degrees points towards the right.
/// * `slack` - The allowed tolerance (slack) in degrees for the aiming process.
///
/// # Returns
///
/// Returns `Ok(())` if the aiming process is successful within the specified
/// tolerance range, or an error of type `MessageError` if there was a problem
/// during communication with the game server.
///
/// # Examples
///
/// ```
/// let result = rbot::await_aim(0, 90, 0.5);
///
/// // If something went wrong in the communication with the server, print it
/// // out to the game console.
/// if result.err {
///     rbot::print("Failed communicating with the game server.")
/// }
/// ```
pub fn await_aim(component_id: i32, angle: f32, slack: f32) -> Result<(), MessageError> {
    aim(component_id, angle)?;
    while !at_rotation(component_id, angle, slack)? {
        sleep(0.01);
    }
    Ok(())
}

/// Waits for the specified robot component's cooldown.
///
/// This function blocks the execution of code until the cooldown is ready of
/// the specified `component_id`. After awaiting the cooldown, it becomes
/// possible to call `use_component` and trigger the specified component.
///
/// # Arguments
///
/// * `component_id` - The identifier of the robot component for which to await the cooldown.
///
/// # Returns
///
/// Returns `Ok(())` if the await is successful else an error of type
/// `MessageError` if there was a problem during communication with the game
/// server.
///
/// # Examples
///
/// ```
/// await_component(0)?;
/// use_component(0)?;
/// ```
pub fn await_component(component_id: i32) -> Result<(), MessageError> {
    while component_state(component_id)?.cooldown > 0.0 {
        sleep(0.01);
    }
    Ok(())
}

/// Retrieves the current state of the robot.
///
/// The robot state includes information such as its angle, velocity, motherboard health,
/// and active buffs on the main board. To extract detailed state information about specific
/// components, use the `component_state` function.
///
/// # Returns
///
/// Returns a `Result` containing `msg::RMsgState` representing the current state of the robot
/// if the retrieval is successful, or an error of type `MessageError` if there was a problem
/// retrieving the robot state.
///
/// # Examples
///
/// ```
/// let robot_state = state()?;
/// ```
pub fn state() -> Result<msg::RMsgState, MessageError> {
    let msg_use = msg::MsgState { value: 0 };
    let response = hostfn::send_message(&msg_use);

    match response {
        MessageType::Error(m) => Err(MessageError::BadCommand(m.error_code)),
        MessageType::RState(m) => Ok(m),
        _ => Err(MessageError::InvalidResponse),
    }
}

/// Retrieves the current status of the specified robot component.
///
/// This function retrieves detailed information about the health, cooldown status,
/// and activation state of the robot's component with id `component_id`.
///
/// # Arguments
///
/// * `component_id` - The identifier of the robot component for which to retrieve the state.
///
/// # Returns
///
/// Returns a `Result` containing `msg::RMsgComponentStatus` representing the current status
/// of the specified robot component if the retrieval is successful, or an error of type `MessageError`
/// if there was a problem retrieving the component state.
///
/// # Examples
///
/// ```
/// let component_status = rbot::component_status(0)?;
/// ```
pub fn component_state(component_id: i32) -> Result<msg::RMsgComponentStatus, MessageError> {
    let msg_comp_state = msg::MsgComponentStatusQuery { component_id };
    let response = hostfn::send_message(&msg_comp_state);

    match response {
        MessageType::Error(m) => Err(MessageError::BadCommand(m.error_code)),
        MessageType::RComponentStatus(m) => Ok(m),
        _ => Err(MessageError::InvalidResponse),
    }
}

/// Pauses the main thread for the specified duration in seconds.
///
/// This function blocks the execution of the main thread, causing it to sleep
/// for the given `seconds` before resuming.
///
/// # Arguments
///
/// * `seconds` - The duration in seconds for which the thread should sleep.
///
/// # Examples
///
/// ```
/// rbot::print("Starting to wait...");
/// rbot::sleep(2.5);
/// rbot::print("Waited for 2.5 seconds.");
/// ```
pub fn sleep(seconds: f32) {
    unsafe { hostfn::sleep(seconds) };
}

/// Generates a pseudo-random floating-point number between 0 (inclusive) and 1 (includive).
///
/// This function returns a random number that falls within the interval [0, 1].
///
/// # Returns
///
/// A random floating-point number in the range [0, 1].
///
/// # Examples
///
/// ```
/// let random_number = random();
/// fn main() {
///     let num = random();
///     println!("Random number between 0 and 1: {}", num);
/// }
///
/// fn random() -> f32 {
///     use rand::Rng;
///     let mut rng = rand::thread_rng();
///     rng.gen::<f32>()
/// }
/// ```
pub fn random() -> f32 {
    unsafe { hostfn::random() }
}

/// Logs a message to the game console in the programming scene.
///
/// This function writes the specified `string` message to a log file that is displayed
/// in the game's console in the programming scene.
///
/// # Arguments
///
/// * `string` - The string message to be logged and displayed in the game console.
///
/// # Examples
///
/// ```
/// rbot::print("Hello World");
/// ```
pub fn print(string: &str) {
    let size = string.len() as i32;
    let bytes_ptr = string.as_ptr() as i32;
    unsafe {
        hostfn::dbg_log(bytes_ptr, size);
    }
}

/// Retrieves the current timestamp in seconds.
///
/// This function returns the current timestamp as a floating-point number representing
/// the number of seconds elapsed since a specific reference time.
///
/// # Returns
///
/// Returns the current timestamp in seconds as a `Result<f32, MessageError>`.
/// If successful, the `Ok` variant contains the timestamp value.
///
/// # Examples
///
/// ```
/// let timestamp = rbot::time()?;
/// ```
pub fn time() -> Result<f32, MessageError> {
    let msg_comp_state = msg::MsgTime { value: 0 };
    let response = hostfn::send_message(&msg_comp_state);

    match response {
        MessageType::Error(m) => Err(MessageError::BadCommand(m.error_code)),
        MessageType::RTime(m) => Ok(m.timestamp),
        _ => Err(MessageError::InvalidResponse),
    }
}
