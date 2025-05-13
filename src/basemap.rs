//! Draw the basemap

pub mod draw;
pub mod projections;
pub mod styles;
pub mod utils;

/// Map struct
/// This struct contains the information needed to draw a map
pub struct Map {
    /// Number of rows in the raster in pixels
    pub rows: i32,
    /// Number of columns in the raster in pixels
    pub cols: i32,
    /// The minimum latitude of the raster
    pub lat_min: f64,
    /// The maximum latitude of the raster
    pub lat_max: f64,
    /// The minimum longitude of the raster
    pub lon_min: f64,
    /// The maximum longitude of the raster
    pub lon_max: f64,
}
