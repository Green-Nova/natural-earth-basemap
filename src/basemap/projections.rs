//! Module for projections

use super::Map;

/// Map from lon,lat to a pixel position
#[must_use]
pub fn equirectangular_mapping_function(lat: f64, lon: f64, map: &Map) -> (f64, f64) {
    // [-180-180] -> [xmin,xmax]
    // [xmin,xmax] -> [0, 1]
    let mapping_fn1 = |(lat, lon)| {
        (
            (lon - map.lon_min) / (map.lon_max - map.lon_min),
            (lat - map.lat_min) / (map.lat_max - map.lat_min),
        )
    };

    // Scale coordinates to the map size
    let mapping_fn2 = |(x, y)| {
        (
            f64::from(map.cols) * x,
            f64::from(map.rows) - f64::from(map.rows) * y,
        )
    };

    mapping_fn2(mapping_fn1((lat, lon)))
}



#[must_use]
pub fn orthographic_mapping_function(lat: f64, lon: f64, map: &Map, camera_location: (f64, f64, f64)) -> (f64, f64) {
    // Scaling coordinates from [-1,1] to [0, 1]
    let mapping_fn1 = |(a, b)| ((a + 1.0) / (2.0), (b + 1.0) / (2.0));

    // Scale coordinates to the map size
    let mapping_fn2 = |(x, y)| {
        (
            f64::from(map.cols) * x,
            f64::from(map.rows) - f64::from(map.rows) * y,
        )
    };

    let (x, y, z) = lat_lon_to_xyz(lat, lon);
    

    // Project point onto basis vectors
    let u= y;
    let v = z;
    
    if !point_visible(lat, lon, camera_location) {
        let norm = (u.powi(2) + v.powi(2)).sqrt();
        let u = u / norm;
        let v = v / norm;
        return mapping_fn2(mapping_fn1((u, v)));
    } else {
        return mapping_fn2(mapping_fn1((u, v)));
    }
}

/// Check if a point is visible from the camera location
pub fn point_visible(lat: f64, lon: f64, camera_location: (f64, f64, f64)) -> bool {
    let (x, y, z) = lat_lon_to_xyz(lat, lon);
    let v1 = (x, y, z);

    // Camera location
    let v2 = (camera_location.0 - x, camera_location.1 - y, camera_location.2 - z);
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
