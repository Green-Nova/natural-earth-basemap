//! Draw the basemap
//! 
//! This module provides functions for rendering the Natural Earth basemap to SVG.
//! It handles the conversion of shapefile data into SVG paths and applies the appropriate styles.

use std::{fs::File, io::BufReader, path::PathBuf};

use shapefile::{Reader, Shape};
use styles::{Layer, LayerStyle, Style};
use svg::{Document, Node, node::element};
use utils::mapping_function;

use super::{Map, styles, utils};

/// Sets the background of the map using the specified layer
/// 
/// # Arguments
/// * `map` - The map dimensions and projection settings
/// * `document` - The SVG document to modify
/// * `layer` - The layer to use for the background (typically ocean)
pub fn set_background(map: &Map, document: &mut Document, layer: &Layer) {
    let data = element::path::Data::new();
    let data = data
        .move_to((0, 0))
        .line_to((0, map.rows))
        .line_to((map.cols, map.rows))
        .line_to((map.cols, 0));

    let path = element::Path::new()
        .set("stroke", layer.layer_style.stroke)
        .set("fill", layer.layer_style.fill)
        .set("fill-opacity", layer.layer_style.fill_opacity)
        .set("d", data);
    document.append(path);
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
pub fn draw_basemap(map: &Map, document: &mut Document) {
    let style = styles::ocean_style();
    set_background(map, document, &style.background);
    for layer in &style.layers {
        let file_path = PathBuf::from(format!("data/10m_physical/{}", layer.filename));
        let reader = shapefile::Reader::from_path(&file_path)
            .unwrap_or_else(|_| panic!("Error loading shapefile: {}", file_path.display()));
        visualize_shapefile(map, reader, document, &layer.layer_style);
    }
}

/// Visualizes a shapefile by converting its features to SVG paths
/// 
/// This function handles different types of features:
/// * Polygons (e.g., land masses, lakes)
/// * Polylines (e.g., rivers, coastlines)
/// 
/// Features that fall entirely outside the map bounds are skipped.
/// 
/// # Arguments
/// * `map` - The map dimensions and projection settings
/// * `reader` - The shapefile reader containing the features
/// * `document` - The SVG document to modify
/// * `layer_style` - The style to apply to the features
pub fn visualize_shapefile(
    map: &Map,
    mut reader: Reader<BufReader<File>, BufReader<File>>,
    document: &mut Document,
    layer_style: &LayerStyle,
) {
    for result in reader.iter_shapes_and_records() {
        let (shape, _record) = result.expect("Error reading data from shapefile");

        match shape {
            Shape::Polygon(polygon) => {
                for ring in polygon.rings() {
                    let data = element::path::Data::new();
                    let pts: Vec<_> = ring
                        .points()
                        .iter()
                        .map(|point| mapping_function(point.x, point.y, map))
                        .collect();

                    // TODO account for cases like the ocean, which are completely outside the map
                    // Check if the polygon is entirely within the map
                    //if !pts.iter().all(|pt| pt.0 >= 0.0 && pt.0 <= map.cols as f64 && pt.1 >= 0.0 && pt.1 <= map.rows as f64) {
                    //    continue;
                    //}
                    draw_polygon(&pts, document, layer_style, data);
                }
            }

            Shape::Polyline(polyline) => {
                for part in polyline.parts() {
                    let data = element::path::Data::new();
                    let pts: Vec<_> = part
                        .iter()
                        .map(|point| mapping_function(point.x, point.y, map))
                        .collect();

                    // Check if the polyline is entirely within the map
                    if !pts.iter().all(|pt| {
                        pt.0 >= 0.0
                            && pt.0 <= f64::from(map.cols)
                            && pt.1 >= 0.0
                            && pt.1 <= f64::from(map.rows)
                    }) {
                        continue;
                    }

                    draw_polyline(&pts, document, layer_style, data);
                }
            }
            _ => {}
        }
    }
}

/// Draws a polygon as an SVG path
/// 
/// # Arguments
/// * `pts` - The points defining the polygon's vertices
/// * `document` - The SVG document to modify
/// * `layer_style` - The style to apply to the polygon
/// * `data` - The initial path data to build upon
pub fn draw_polygon(
    pts: &Vec<(f64, f64)>,
    document: &mut Document,
    layer_style: &LayerStyle,
    data: element::path::Data,
) {
    let data = data.move_to(pts[0]);
    let data = pts.iter().fold(data, |data, position| {
        data.line_to((position.0, position.1))
    });

    let path = element::Path::new()
        .set("fill", layer_style.fill)
        .set("fill-opacity", layer_style.fill_opacity)
        .set("stroke", layer_style.stroke)
        .set("stroke-width", layer_style.stroke_width)
        .set("d", data);

    document.append(path);
}

/// Draws a polyline as an SVG path
/// 
/// # Arguments
/// * `pts` - The points defining the polyline's vertices
/// * `document` - The SVG document to modify
/// * `layer_style` - The style to apply to the polyline
/// * `data` - The initial path data to build upon
pub fn draw_polyline(
    pts: &Vec<(f64, f64)>,
    document: &mut Document,
    layer_style: &LayerStyle,
    data: element::path::Data,
) {
    let data = data.move_to(pts[0]);
    let data = pts.iter().fold(data, |data, position| {
        data.line_to((position.0, position.1))
    });

    let path = element::Path::new()
        .set("fill", layer_style.fill)
        .set("fill-opacity", layer_style.fill_opacity)
        .set("stroke", layer_style.stroke)
        .set("stroke-width", layer_style.stroke_width)
        .set("stroke-linejoin", "round")
        .set("d", data);

    document.append(path);
}