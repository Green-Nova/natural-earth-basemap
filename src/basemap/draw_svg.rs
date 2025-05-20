//! Draw the basemap
//!
//! This module provides functions for rendering the Natural Earth basemap to SVG.
//! It handles the conversion of shapefile data into SVG paths and applies the appropriate styles.

use super::styles::{Layer, LayerStyle};
use svg::{Document, Node, node::element};

use super::Map;

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

/// Draws a polygon as an SVG path
///
/// # Arguments
/// * `pts` - The points defining the polygon's vertices
/// * `document` - The SVG document to modify
/// * `layer_style` - The style to apply to the polygon
/// * `data` - The initial path data to build upon
pub fn draw_polygon(
    pts: &[(f64, f64)],
    document: &mut Document,
    layer_style: &LayerStyle,
    data: element::path::Data,
) {
    let data = data.move_to(pts[0]);
    let data = pts.iter().skip(1).fold(data, |data, position| {
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
    pts: &[(f64, f64)],
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

/// Draws text at a specified location on the map
///
/// # Arguments
/// * `position` - The (x,y) coordinates where the text should be placed
/// * `text` - The text string to draw
/// * `document` - The SVG document to modify
/// * `font_size` - The font size in pixels
/// * `fill` - The text color
pub fn draw_text(
    position: (f64, f64),
    text: &str,
    document: &mut Document,
    font_size: u32,
    fill: &str,
) {
    let text_element = element::Text::new(text)
        .set("x", position.0)
        .set("y", position.1)
        .set("font-family", "Arial")
        .set("font-size", font_size)
        .set("fill", fill)
        .set("text-anchor", "middle");

    document.append(text_element);
}
