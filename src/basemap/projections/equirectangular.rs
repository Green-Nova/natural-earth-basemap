//! Equirectangular projection

use crate::basemap::Map;

use super::Point3D;

/*
/// Map from lon,lat to a pixel position
#[must_use]
pub fn equirectangular_mapping_function(point:Point3D, map: &Map) -> (f64, f64) {
    // [-lon_min,lon_max] -> [0, 1]
    let mapping_fn1 = |(lat, lon)| {
        (
            (lon - map.lon_min) / (map.lon_max - map.lon_min),
            (lat - map.lat_min) / (map.lat_max - map.lat_min),
        )
    };

    // Scale coordinates to the map size
    // [0, 1] -> [0, map.cols]
    let mapping_fn2 = |(x, y)| {
        (
            f64::from(map.cols) * x,
            f64::from(map.rows) - f64::from(map.rows) * y,
        )
    };

    mapping_fn2(mapping_fn1((lat, lon)))
}
 */