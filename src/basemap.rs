//! Draw the basemap

use std::path::PathBuf;
use shapefile::Reader;
use svg::Document;

pub mod draw_svg;
pub mod shapefile_visualizer;
pub mod styles;
pub mod utils;

use styles::Style;
use draw_svg::set_background;

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


/// Draws the complete basemap using the ocean style
///
/// This function:
/// 1. Sets the background using the ocean layer
/// 2. Loads and draws each layer in sequence
/// 3. Applies the appropriate styles to each feature
///
/// # Arguments
/// * `map` - The map dimensions and projection settings
/// * `document` - The SVG document to modify
/// * `style` - The style to use for the basemap
pub fn draw_basemap(map: &Map, document: &mut Document, style: &Style) {
    set_background(map, document, &style.background);
    for layer in &style.layers {
        let file_path = PathBuf::from(format!("data/10m_physical/{}", layer.filename));
        let reader = Reader::from_path(&file_path)
            .unwrap_or_else(|_| panic!("Error loading shapefile: {}", file_path.display()));
        shapefile_visualizer::visualize_shapefile(map, reader, document, &layer.layer_style);
    }
    // Todo draw graticules
    // Todo draw user defined content
    //draw_text((500.0, 500.0), "Hello, world!", document, 12, "black");
}