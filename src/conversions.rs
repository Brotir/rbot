use std::f32::consts::PI;

/// Computes the angle (in degrees) from the positive x-axis to a point (x, y) in the Cartesian plane.
///
/// This function calculates the angle formed by the positive x-axis and the vector pointing from the origin
/// (0, 0) to the point (x, y) specified by the coordinates `x` and `y`. The angle is measured in degrees
/// counter-clockwise from the positive x-axis.
///
/// # Arguments
///
/// * `x` - The x-coordinate of the point.
/// * `y` - The y-coordinate of the point.
///
/// # Returns
///
/// The angle in degrees from the positive x-axis to the point (x, y). The angle is in the range (-180, 180],
/// where positive angles represent counter-clockwise rotations and negative angles represent clockwise rotations.
///
/// # Examples
///
/// ```
/// let x = 1.0;
/// let y = 1.0;
/// let angle = xy_to_angle(x, y);
/// ```
pub fn xy_to_angle(x: f32, y: f32) -> f32 {
    f32::atan2(y, x) * 180. / PI
}

/// Computes the Cartesian coordinates (x, y) corresponding to a given angle (in degrees) from the positive x-axis.
///
/// This function calculates the Cartesian coordinates (x, y) corresponding to a specified angle measured
/// in degrees from the positive x-axis. The angle determines the direction of a vector from the origin (0, 0)
/// to the point (x, y), where the length of the vector is 1 (unit vector).
///
/// # Arguments
///
/// * `angle` - The angle in degrees from the positive x-axis.
///
/// # Returns
///
/// An array `[x, y]` containing the Cartesian coordinates corresponding to the specified angle.
/// - `x` is the cosine of the angle.
/// - `y` is the sine of the angle.
///
/// # Examples
///
/// ```
/// let angle = 45.0;
/// let [x, y] = angle_to_xy(angle);
/// ```
pub fn angle_to_xy(angle: f32) -> [f32; 2] {
    let rad = angle * PI / 180.;
    [rad.cos(), rad.sin()]
}
