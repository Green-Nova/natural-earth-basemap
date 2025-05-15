// This file is now empty. Equirectangular and orthographic projections are in their own modules: equirectangular.rs and orthographic.rs.

//! Module for projections

pub mod equirectangular;
pub mod orthographic;


/// A 3D point in Cartesian coordinates
#[derive(Debug, Clone, Copy)]
pub struct Point3D {
    /// x coordinate
    pub x: f64,
    /// y coordinate 
    pub y: f64,
    /// z coordinate
    pub z: f64,
}

impl Point3D {
    /// Create a new 3D point
    #[must_use]
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    /// Create a 3D point from latitude and longitude (in degrees)
    #[must_use] 
    pub fn from_lat_lon(lat: f64, lon: f64) -> Self {
        let (x, y, z) = lat_lon_to_xyz(lat, lon);
        Self { x, y, z }
    }

    /// Calculate dot product with another point
    #[must_use]
    pub fn dot(&self, other: &Point3D) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    /// Check if this point is visible from a camera location
    #[must_use]
    pub fn is_visible(&self, camera: &Point3D) -> bool {
        let camera_point_vector = Point3D::new(
            camera.x - self.x,
            camera.y - self.y, 
            camera.z - self.z
        );
        self.dot(&camera_point_vector) > 0.0
    }


    /// Rotates a 3D point by three Euler angles (in radians) using the ZYX (intrinsic) convention.
    /// This means the rotations are applied in the following order:
    /// 1. Rotation around Z axis (yaw)
    /// 2. Rotation around Y axis (pitch)
    /// 3. Rotation around X axis (roll)
    ///
    /// # Arguments
    ///
    /// * `point` - The point to rotate as (x,y,z)
    /// * `angles` - The rotation angles in radians as (roll, pitch, yaw)
    ///
    /// # Returns
    ///
    /// The rotated point as (x,y,z)
    pub fn rotate_point(&mut self, angles: (f64, f64, f64)){
    let (roll, pitch, yaw) = angles;

    // Pre-compute trig functions
    let (sin_a, cos_a) = roll.sin_cos();
    let (sin_b, cos_b) = pitch.sin_cos();
    let (sin_c, cos_c) = yaw.sin_cos();

    // Extract point coordinates
    let (x, y, z) = (&self.x, &self.y, &self.z);

    // Apply rotations in ZYX order
    let x1 = x * (cos_b * cos_c) + y * (-cos_b * sin_c) + z * sin_b;
    let y1 = x * (cos_a * sin_c + sin_a * sin_b * cos_c)
        + y * (cos_a * cos_c - sin_a * sin_b * sin_c)
        + z * (-sin_a * cos_b);
    let z1 = x * (sin_a * sin_c - cos_a * sin_b * cos_c)
        + y * (sin_a * cos_c + cos_a * sin_b * sin_c)
        + z * (cos_a * cos_b);

    self.x = x1;
    self.y = y1;
    self.z = z1;
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
///
/// # Arguments
///
/// * `lat` - Latitude in degrees
/// * `lon` - Longitude in degrees
///
/// # Returns
///
/// A tuple (x,y,z) representing the 3D Cartesian coordinates where:
/// * x = cos(lat) * cos(lon)
/// * y = cos(lat) * sin(lon)
/// * z = sin(lat)

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
///
/// # Arguments
///
/// * `v1` - The first vector (x,y,z)
/// * `v2` - The second vector (x,y,z)
///
/// # Returns
///
/// The dot product of the two vectors.
fn dot(v1: (f64, f64, f64), v2: (f64, f64, f64)) -> f64 {
    v1.0 * v2.0 + v1.1 * v2.1 + v1.2 * v2.2
}


/*


/*
// TODO Visible count needs to take into account the rotation of the sphere
let visible_count = ring
.points()
.iter()
.filter(|point| point_visible(point.y, point.x, camera_location))
.count();

//if visible_count > 3 {
let pts: Vec<_> = ring
    .points()
    .iter()
    .map(|point| {
        orthographic_mapping_function(
            point.y,
            point.x,
            map,
            camera_location,
        )
    })
    .collect();
draw_polygon(&pts, document, layer_style);
//}

*/*/
