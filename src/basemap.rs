//! Draw the basemap

use std::path::PathBuf;
use shapefile::Reader;
use svg::{Document, node::element};

pub mod draw_svg;
pub mod shapefile_visualizer;
pub mod styles;
pub mod utils;

use styles::{Style, LayerStyle};
use draw_svg::{set_background, draw_polyline};
use utils::mapping_function;

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
    // Draw graticules and equator
    draw_graticules(map, document, &style.graticule_style);
    draw_equator(map, document, &style.equator_style);
    // Todo draw user defined content
    //draw_text((500.0, 500.0), "Hello, world!", document, 12, "black");
}

/// Draw a map
pub fn draw_map(map: &Map,style: &Style, output_path: &PathBuf) {
    let mut document = svg::Document::new().set("viewBox", (0, 0, map.cols, map.rows));
    
    draw_basemap(map, &mut document, style);

    svg::save(output_path, &document).expect("Error saving svg");
    utils::svg_to_png(output_path, &PathBuf::from("Map.png"));
}

/// Draws the graticule grid (latitude and longitude lines)
///
/// This function draws a grid of lines at 15-degree intervals for both longitude and latitude.
/// The grid helps users understand the geographic coordinates on the map.
///
/// # Arguments
/// * `map` - The map dimensions and projection settings
/// * `document` - The SVG document to modify
/// * `graticule_style` - The style to use for the graticule lines
///
/// # Example
/// ```
/// let map = Map { /* ... */ };
/// let mut document = Document::new();
/// let style = Style { /* ... */ };
/// draw_graticules(&map, &mut document, &style.graticule_style);
/// ```
fn draw_graticules(map: &Map, document: &mut Document, graticule_style: &LayerStyle) {
    let data = element::path::Data::new();

    // Draw meridians (vertical lines)
    for lon in (-180..=180).step_by(15) {
        let mut points = Vec::new();
        for lat in (map.lat_min.floor() as i32..=map.lat_max.ceil() as i32).step_by(1) {
            points.push(mapping_function(lon as f64, lat as f64, map));
        }
        draw_polyline(&points, document, graticule_style, data.clone());
    }

    // Draw parallels (horizontal lines) 
    for lat in (-90..=90).step_by(15) {
        let mut points = Vec::new();
        for lon in (map.lon_min.floor() as i32..=map.lon_max.ceil() as i32).step_by(1) {
            points.push(mapping_function(lon as f64, lat as f64, map));
        }
        draw_polyline(&points, document, graticule_style, data.clone());
    }
}

/// Draws the equator line
///
/// # Arguments
/// * `map` - The map dimensions and projection settings
/// * `document` - The SVG document to modify
/// * `equator_style` - The style to use for the equator line
fn draw_equator(map: &Map, document: &mut Document, equator_style: &LayerStyle) {
    let data = element::path::Data::new();
    let mut points = Vec::new();
    
    // Draw equator (0° latitude)
    for lon in (map.lon_min.floor() as i32..=map.lon_max.ceil() as i32).step_by(1) {
        points.push(mapping_function(lon as f64, 0.0, map));
    }
    draw_polyline(&points, document, equator_style, data);
}
