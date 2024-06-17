/// Transforms a global rotation angle to the local reference frame of a specified component.
///
/// This function converts a rotation angle specified in the global coordinate system to the
/// local reference frame of the robot component identified by `component_id`. It adjusts the
/// angle based on the orientation/location of the component within the robot's structure.
///
/// # Arguments
///
/// * `component_id` - The identifier of the robot component whose reference frame to use for transformation.
/// * `angle` - The rotation angle in degrees in the global coordinate system.
///
/// # Returns
///
/// The transformed rotation angle adjusted to the local reference frame of the specified component,
/// represented as a floating-point number in degrees.
///
/// # Examples
///
/// ```
/// let component_id = 1;
/// let global_angle = 45.0;
/// let local_angle = transform_rotation_to_component(component_id, global_angle);
/// ```
pub fn transform_rotation_to_component(component_id: i32, angle: f32) -> f32 {
    angle + (-90. * component_id as f32)
}

/// Transforms a rotation angle from the local reference frame of a component to the global coordinate system.
///
/// This function converts a rotation angle specified in the local reference frame of a robot component
/// (identified by `component_id`) to the global coordinate system. It adjusts the angle based on the
/// orientation of the component within the robot's structure.
///
/// # Arguments
///
/// * `component_id` - The identifier of the robot component whose local reference frame to use for transformation.
/// * `angle` - The rotation angle in degrees in the local reference frame of the component.
///
/// # Returns
///
/// The transformed rotation angle adjusted to the global coordinate system, represented as a floating-point number in degrees.
///
/// # Examples
///
/// ```
/// let component_id = 1;
/// let local_angle = 30.0;
/// let global_angle = transform_rotation_from_component(component_id, local_angle);
/// ```
pub fn transform_rotation_from_component(component_id: i32, angle: f32) -> f32 {
    angle + (90. * component_id as f32)
}

/// Computes the angular distance between two angles.
///
/// This function calculates the smallest angular difference (distance) between two given angles,
/// taking into account the circular nature of angle measurements (0 to 360 degrees).
///
/// # Arguments
///
/// * `angle` - The first angle in degrees.
/// * `other_angle` - The second angle in degrees.
///
/// # Returns
///
/// The angular distance between the two angles as a floating-point number.
/// The returned value represents the smallest magnitude difference between the angles,
/// ranging from 0 to 180 degrees.
///
/// # Examples
///
/// ```
/// use bot;
///
/// let angle1 = 30.0;
/// let angle2 = 350.0;
/// let distance = bot::rotations::angle_distance(angle1, angle2);
/// bot::print(&format!("Angular distance: {:.2} degrees", distance));
/// ```
pub fn angle_distance(angle: f32, other_angle: f32) -> f32 {
    let angle_difference = angle - other_angle;
    ((angle_difference + 180.0).rem_euclid(360.0) - 180.0).abs()
}
