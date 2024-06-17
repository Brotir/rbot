use crate::core;
use crate::errors::MessageError;
use crate::hostfn;
use rbot_messages::messages as msg;
use rbot_messages::MessageType;
use strum_macros::EnumIter;

#[derive(Debug, EnumIter, Clone, Copy)]
pub enum Module {
    Teleporter = 0,
    Radar,
    ForceField,
    Laser,
    Mine,
    Repair,
    Thruster,
    Scanner,
    GPS,
}

/// Macro for handling the incomming message.
macro_rules! match_message {
    ($msg: expr, $response_type:pat => $response: expr) => {
        match hostfn::send_message(&$msg) {
            MessageType::Error(m) => Err(MessageError::BadCommand(m.error_code)),
            $response_type => $response,
            _ => Err(MessageError::InvalidResponse),
        }
    };
}

/// Retrieves the status of a specific module.
///
/// This function fetches the status of the specified `module`, including details such as
/// the remaining cooldown duration before the module can be used again.
///
/// # Arguments
///
/// * `module` - The module for which to retrieve the status.
///
/// # Returns
///
/// A `Result` containing `msg::RMsgModuleStatus` representing the status of the module
/// if the retrieval is successful, or an error of type `MessageError` if there was a problem
/// retrieving the module status.
///
/// # Examples
///
/// ```
/// use bot;
///
/// let radar_status = bot::modules::status(Module::Radar)?;
/// ```
pub fn status(module: Module) -> Result<msg::RMsgModuleStatus, MessageError> {
    let msg = msg::MsgModuleStatusQuery {
        module_id: module as i32,
    };
    match_message!(msg, MessageType::RModuleStatus(m) => Ok(m))
}

/// Blocks execution until the remaining cooldown of the module expires.
///
/// This function pauses the execution of code until the module's cooldown is
/// ready and the module becomes available for use again.
///
/// # Examples
///
/// ```
/// use bot;
///
/// // Wait for the cooldown of the module to expire.
/// bot::modules::await_cooldown(Module.Radar);
/// // Now we can use the module and get a valid response.
/// let radar_msg = bot::modules::radar()?;
/// ```
pub fn await_module(module: Module) -> Result<(), MessageError> {
    while status(module)?.cooldown > 0.0 {
        core::sleep(0.01);
    }
    Ok(())
}

// ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
//            Module Specific Below
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

/// DEPRECATED Teleports the robot to the specified coordinates (x, y) relative
/// to the robot within the game environment.
///
/// This function moves the robot instantly to the specified coordinates (`x`, `y`) relative
/// to the robot within the game environment, effectively teleporting the robot to a new position.
///
/// # Arguments
///
/// * `x` - The x-coordinate of the teleport destination relative to the robots position.
/// * `y` - The y-coordinate of the teleport destination relative to the robots position.
///
/// # Returns
///
/// A `Result` containing `msg::MsgEmpty` indicating a successful teleportation operation,
/// or an error of type `MessageError` if the teleportation failed.
///
/// # Examples
///
/// ```
/// use bot;
///
/// let result = bot::modules::teleport(10, 2);
/// ```
pub fn teleport(x: f32, y: f32) -> Result<msg::MsgEmpty, MessageError> {
    let msg = msg::MsgTeleport { x, y };
    match_message!(msg, MessageType::Empty(m) => Ok(m))
}

/// Initiates a radar pulse to detect the closest enemy robot and retrieves the
/// position and distance relative to your robot.
///
/// This function triggers a radar pulse to scan for the closest enemy robot
/// within the detection range of your robot. It retrieves information about the
/// closest detected enemy robot, including its position relative to your robot
/// and the distance between them.
///
/// # Returns
///
/// A `Result` containing `msg::RMsgRadar` representing the radar scan results, including the position and distance of the closest enemy robot,
/// or an error of type `MessageError` if the radar pulse fails.
///
/// # Examples
///
/// ```
/// use bot;
///
/// let radar_msg = bot::modules::radar()?;
/// ```
pub fn radar() -> Result<msg::RMsgRadar, MessageError> {
    let msg = msg::MsgRadar { value: 0 };
    match_message!(msg, MessageType::RRadar(m) => Ok(m))
}

/// Sends a laser scan at a specified angle to detect an object within the
/// robot's line of sight.
///
/// This function performs a laser scan in the specified `angle` to detect an
/// object that are within the robot's line of sight. It retrieves information
/// about the detected object, including its tag, type, distance from the robot,
/// angle which will be the same as the provided argument, and any associated
/// buffs.
///
/// If the laser scan hits an object with the `BotComponent` tag, additional
/// information is provided about the 'kind' of component (e.g., Rifle) and any
/// associated `buffs`. For objects of type `Wall` or `Sentry`, the `kind` and `buffs`
/// will be empty.
///
/// # Arguments
///
/// * `angle` - The angle (in degrees) at which to perform the laser scan relative to the robot's orientation.
///
/// # Returns
///
/// A `Result` containing `msg::RMsgLaser` representing the results of the laser scan,
/// or an error of type `MessageError` if the scan fails.
///
/// # Examples
///
/// ```
/// use bot;
///
/// let laser_msg = bot::laser(45)?;
/// ```
pub fn laser(angle: f32) -> Result<msg::RMsgLaser, MessageError> {
    let msg = msg::MsgLaser { angle };
    match_message!(msg, MessageType::RLaser(m) => Ok(m))
}

/// Activates a force field that grants temporary invincibility to the robot.
///
/// This function activates a force field that provides temporary invincibility to the robot,
/// making it immune to damage for a short duration.
///
/// # Returns
///
/// A `Result` containing `msg::MsgEmpty` indicating a successful activation of the force field,
/// or an error of type `MessageError` if the activation fails.
///
/// # Examples
///
/// ```
/// use bot;
///
/// let result = bot::modules::force_field();
/// ```
pub fn force_field() -> Result<msg::MsgEmpty, MessageError> {
    let msg = msg::MsgForceField { value: 0 };
    match_message!(msg, MessageType::Empty(m) => Ok(m))
}

/// Drops a mine that activates after a short duration.
///
/// This function drops a mine beneath the robot, which will activate after a brief delay.
/// It's important to exercise caution as the mine can deal damage to the robot if driven over
/// after activation.
///
/// # Returns
///
/// A `Result` containing `msg::MsgEmpty` indicating successful deployment of the mine,
/// or an error of type `MessageError` if the deployment fails.
///
/// # Examples
///
/// ```
/// use bot;
///
/// let result = bot::modules::mine();
/// ```
pub fn mine() -> Result<msg::MsgEmpty, MessageError> {
    let msg = msg::MsgMine { value: 0 };
    match_message!(msg, MessageType::Empty(m) => Ok(m))
}

/// Repairs the specified robot component, restoring it to a significantly
/// healthier state.
///
/// This function performs repairs on the specified `component_id`, restoring
/// the health of the robot component to a significantly improved state.
///
/// # Arguments
///
/// * `component_id` - The identifier of the component to be repaired.
///
/// # Returns
///
/// A `Result` containing `msg::RMsgRepair` representing the restored health of
/// the component, or an error of type `MessageError` if the repair operation
/// fails.
///
/// # Examples
///
/// ```
/// use bot;
///
/// let healed_amount = bot::modules::repair(0);
/// ```
pub fn repair(component_id: i32) -> Result<msg::RMsgRepair, MessageError> {
    let msg = msg::MsgRepair { component_id };
    match_message!(msg, MessageType::RRepair(m) => Ok(m))
}

/// Activates a thruster to swiftly move the robot a short distance in the
/// specified global angle.
///
/// This function activates a thruster to propel the robot quickly in the
/// direction specified by the global `angle`. The angle is defined in the
/// global coordinate system, where:
///
/// * 0 degrees points to the right.
/// * 90 degrees points upward.
/// * 180 degrees points to the left.
/// * 270 degrees points downward.
///
/// # Arguments
///
/// * `angle` - The global angle (in degrees) at which to move the robot.
///
/// # Returns
///
/// A `Result` containing `msg::MsgEmpty` indicating successful activation of the thruster,
/// or an error of type `MessageError` if the activation fails.
///
/// # Examples
///
/// ```
/// use bot;
///
/// let result = bot::modules::thrust(0);
/// ```
pub fn thrust(angle: f32) -> Result<msg::MsgEmpty, MessageError> {
    let msg = msg::MsgThrust { angle };
    match_message!(msg, MessageType::Empty(m) => Ok(m))
}

/// Initiates a 360-degree scan to detect nearby objects around the robot within a specified range.
///
/// This function performs a full 360-degree scan around the robot to detect all nearby objects
/// within the scanning range. The scan results include detailed information about each detected object,
/// such as its `tag`, `kind`, position (`x`, `y`), and potential buffs.
///
/// # Returns
///
/// A `Result` containing `msg::RMsgScan` representing the scan results, including a list of all detected objects
/// and their associated information, or an error of type `MessageError` if the scan operation fails.
///
/// # Examples
///
/// ```
/// use bot;
///
/// let scan_results = bot::modules::scan()?;
/// ```
pub fn scan() -> Result<msg::RMsgScan, MessageError> {
    let msg = msg::MsgScan { value: 0 };
    match_message!(msg, MessageType::RScan(m) => Ok(m))
}

/// Retrieves the absolute position (`x`, `y`) of the robot from the center of the map using GPS.
///
/// This function utilizes the GPS system to determine the robot's absolute position (`x`, `y`)
/// relative to the center of the map.
///
/// # Returns
///
/// A `Result` containing `msg::RMsgGPS` representing the robot's absolute position on the map,
/// or an error of type `MessageError` if the GPS retrieval fails.
///
/// # Examples
///
/// ```
/// use bot;
///
/// let position = bot::modules::gps()?;
/// ```
pub fn gps() -> Result<msg::RMsgGPS, MessageError> {
    let msg = msg::MsgGPS { value: 0 };
    match_message!(msg, MessageType::RGPS(m) => Ok(m))
}
