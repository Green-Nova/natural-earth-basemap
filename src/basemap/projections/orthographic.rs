//! Orthographic projection and helpers

use crate::basemap::Map;

use super::Point3D;

/// Map from lon,lat to a pixel position using an orthographic projection
#[must_use]
pub fn orthographic_mapping_function(
    point:Point3D,
    map: &Map,
    camera_location: (f64, f64, f64),
) -> (f64, f64) {
    // Scaling coordinates from [-1,1] to [0, 1]
    let mapping_fn1 = |(a, b)| ((a + 1.0) / (2.0), (b + 1.0) / (2.0));

    // Scale coordinates to the map size
    let mapping_fn2 = |(x, y)| {
        (
            f64::from(map.cols) * x,
            f64::from(map.rows) - f64::from(map.rows) * y,
        )
    };

    /*
    let (x, y, z) = lat_lon_to_xyz(lat, lon);

    let yaw = -140.0_f64.to_radians();
    let pitch = 0.0_f64.to_radians();
    let roll = 0.0;
    let (x, y, z) = rotate_point((x, y, z), (roll, pitch, yaw));

    let u = y;
    let v = z;
     */
    
    let (x, y, z) = (point.x, point.y, point.z);
    let u = y;
    let v = z;
    if !point_visible_xyz((x, y, z), camera_location) {
        let norm = (u.powi(2) + v.powi(2)).sqrt();
        let u = u / norm;
        let v = v / norm;
        return mapping_fn2(mapping_fn1((u, v)));
    } else {
        return mapping_fn2(mapping_fn1((u, v)));
    }
}

/// Check if a point is visible from the camera location
pub fn point_visible_xyz(point: (f64, f64, f64), camera_location: (f64, f64, f64)) -> bool {
    let camera_point_vector = (
        camera_location.0 - point.0,
        camera_location.1 - point.1,
        camera_location.2 - point.2,
    );
    dot(point, camera_point_vector) > 0.0
}

/// Check if a point is visible from the camera location
pub fn point_visible(lat: f64, lon: f64, camera_location: (f64, f64, f64)) -> bool {
    let (x, y, z) = lat_lon_to_xyz(lat, lon);
    let v1 = (x, y, z);

    // Camera location
    let v2 = (
        camera_location.0 - x,
        camera_location.1 - y,
        camera_location.2 - z,
    );
    dot(v1, v2) > 0.0
}

/// Converts latitude and longitude coordinates to 3D Cartesian coordinates (x,y,z)
/// on a unit sphere centered at the origin.
fn lat_lon_to_xyz(lat: f64, lon: f64) -> (f64, f64, f64) {
    let radius = 1.0; // Unit sphere
    let lon_rad = lon.to_radians();
    let lat_rad = lat.to_radians();
    let x = radius * (lat_rad.cos() * lon_rad.cos());
    let y = radius * (lat_rad.cos() * lon_rad.sin());
    let z = radius * lat_rad.sin();
    (x, y, z)
}

/// Computes the dot product of two 3D vectors.
fn dot(v1: (f64, f64, f64), v2: (f64, f64, f64)) -> f64 {
    v1.0 * v2.0 + v1.1 * v2.1 + v1.2 * v2.2
}

/// Rotates a 3D point by three Euler angles (in radians) using the ZYX (intrinsic) convention.
fn rotate_point(point: (f64, f64, f64), angles: (f64, f64, f64)) -> (f64, f64, f64) {
    let (roll, pitch, yaw) = angles;

    // Pre-compute trig functions
    let (sin_a, cos_a) = roll.sin_cos();
    let (sin_b, cos_b) = pitch.sin_cos();
    let (sin_c, cos_c) = yaw.sin_cos();

    // Extract point coordinates
    let (x, y, z) = point;

    // Apply rotations in ZYX order
    let x1 = x * (cos_b * cos_c) + y * (-cos_b * sin_c) + z * sin_b;
    let y1 = x * (cos_a * sin_c + sin_a * sin_b * cos_c)
        + y * (cos_a * cos_c - sin_a * sin_b * sin_c)
        + z * (-sin_a * cos_b);
    let z1 = x * (sin_a * sin_c - cos_a * sin_b * cos_c)
        + y * (sin_a * cos_c + cos_a * sin_b * sin_c)
        + z * (cos_a * cos_b);

    (x1, y1, z1)
}
