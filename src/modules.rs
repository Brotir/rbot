use crate::await_action;
use crate::constants;
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
/// let radar_status = rbot::modules::status(Module::Radar)?;
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
/// // Wait for the cooldown of the module to expire.
/// rbot::modules::await_cooldown(Module.Radar);
/// // Now we can use the module and get a valid response.
/// let radar_msg = rbot::modules::radar()?;
/// ```
pub fn await_module(module: Module) -> Result<(), MessageError> {
    await_action()?;
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
/// let result = rbot::modules::teleport(10, 2);
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
/// let radar_msg = rbot::modules::radar()?;
/// ```
///
/// Below is an example to use radar to aim and shoot at your enemy:
/// ```
/// loop {
///     // Wait for the radar to not have cooldown.
///     rbot::modules::await_module(rbot::modules::Module::Radar);
///
///     // Get the message from the radar.
///     let radar_message = rbot::modules::radar().expect("unexpected failure calling radar");
///
///     // Convert the xy position (from the radar) into an angle to aim.
///     let angle = rbot::conversions::xy_to_angle(radar_message.x, radar_message.y);
///     for i in 0..4 {
///         // Aim the specified component towards the enemy.
///         rbot::await_aim(i, angle, 0.5);
///
///         // Wait for the component to be ready to use.
///         rbot::await_component(i);
///
///         // Use the specific component.
///         rbot::use_component(i, false);
///     }
/// }
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
/// let laser_msg = rbot::laser(45)?;
/// ```
///
/// Below is an example function that uses the laser to find the enemy (if it is not obstructed):
/// ```
/// fn laser_search_enemy() -> Option<RMsgLaser> {
///    for angle in 0..360 {
///        let laser_message = rbot::modules::laser(angle as f32);
///        if let Ok(laser_message) = laser_message {
///            if laser_message.tag == rbot::constants::tag::COMPONENT {
///                return Some(laser_message);
///            }
///        }
///    }
///    None
/// }
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
/// let result = rbot::modules::force_field();
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
/// let result = rbot::modules::mine();
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
/// let healed_amount = rbot::modules::repair(0);
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
/// let result = rbot::modules::thrust(0);
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
/// let scan_results = rbot::modules::scan()?;
/// ```
///
/// Below is an example function to use the scanner to find the average enemy position (if the enemy is in range of the scanner):
/// ```
/// pub fn scan_for_average_bot_component() -> Option<[f32;2]> {
///    let scan_msg = rbot::modules::scan().ok()?;
///    let components: Vec<_> = scan_msg
///        .objects
///        .into_iter()
///        .filter(|o| o.tag == rbot::constants::tag::COMPONENT)
///        .collect();
///
///    if components.len() == 0 {
///        return None;
///    }
///
///    // Find the average position of the components.
///    let x: f32 = components.iter().map(|c| c.x).sum::<f32>() / components.len() as f32;
///    let y: f32 = components.iter().map(|c| c.y).sum::<f32>() / components.len() as f32;
///
///    Some([x, y])
///}
/// ```
pub fn scan() -> Result<msg::RMsgScan, MessageError> {
    let msg = msg::MsgScan { value: 0 };
    match_message!(msg, MessageType::RScan(m) => Ok(m))
}

/// Scans for the average position of the components of an enemy bot, if any are found.
///
/// This function is useful for locating the center of an enemy bot. It attempts to find
/// the position of the motherboard first. If the motherboard is not found, it calculates
/// and returns the average position of the bot's components. If no components are found,
/// it returns `None`.
///
/// # Returns
///
/// * `Ok(Some(msg::RMsgScanObject))` - containing the position of the motherboard if found,
///   or the average position of the components.
/// * `Ok(None)` - if no components are found.
/// * `Err(MessageError)` - if an error occurs during the scan.
///
/// # Examples
///
/// ```
/// // Scan for the enemy bot and get its average position or motherboard position.
/// match rbot::scan_for_bot()? {
///     Some(bot_position) => rbot::println!("Bot position: ({}, {})", bot_position.x, bot_position.y),
///     None => rbot::println!("No bot components found."),
/// }
/// ```
pub fn scan_for_bot() -> Result<Option<msg::RMsgScanObject>, MessageError> {
    let scan_msg = scan()?;
    let components: Vec<_> = scan_msg
        .objects
        .into_iter()
        .filter(|o| o.tag == crate::constants::tag::COMPONENT)
        .collect();

    if components.len() == 0 {
        return Ok(None);
    }

    let motherboard = components
        .iter()
        .find(|c| c.kind == constants::kind::MOTHERBOARD);

    // If the motherboard is found, return its position and information
    if let Some(motherboard) = motherboard {
        return Ok(Some(msg::RMsgScanObject {
            x: motherboard.x,
            y: motherboard.y,
            tag: constants::tag::BOT.into(),
            kind: "".into(),
            buffs: motherboard.buffs.to_owned(),
        }));
    }

    // Calculate the average position of the components if the motherboard is not found
    let x: f32 = components.iter().map(|c| c.x).sum::<f32>() / components.len() as f32;
    let y: f32 = components.iter().map(|c| c.y).sum::<f32>() / components.len() as f32;

    Ok(Some(msg::RMsgScanObject {
        x,
        y,
        tag: constants::tag::BOT.into(),
        kind: "".into(),
        buffs: vec![],
    }))
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
/// let position = rbot::modules::gps()?;
/// ```
pub fn gps() -> Result<msg::RMsgGPS, MessageError> {
    let msg = msg::MsgGPS { value: 0 };
    match_message!(msg, MessageType::RGPS(m) => Ok(m))
}
