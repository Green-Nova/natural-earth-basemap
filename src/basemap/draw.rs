//! Draw the basemap
use std::{fs::File, io::BufReader, path::PathBuf};

use styles::LayerStyle;
use svg::{Document, Node, node::element};
//use projections::equirectangular_mapping_function;
use projections::orthographic_mapping_function;

use super::{
    Map,
    projections::{self, point_visible},
    styles,
};

/// Set the background of the map
pub fn set_background(map: &Map, document: &mut Document) {
    let data = element::path::Data::new();
    let data = data
        .move_to((0, 0))
        .line_to((0, map.rows))
        .line_to((map.cols, map.rows))
        .line_to((map.cols, 0));

    // TODO allow the background to be set by the style
    let path = element::Path::new()
        .set("stroke", "white")
        .set("fill", "white")
        .set("fill-opacity", "0")
        .set("d", data);
    document.append(path);
}

/// Draw the basemap
pub fn draw_basemap(map: &Map, document: &mut Document) {
    let layers = styles::ocean_style();
    set_background(map, document);
    for (filename, layer_style) in layers {
        let file_path = PathBuf::from(format!("data/10m_physical/{filename}"));
        let reader = shapefile::Reader::from_path(&file_path)
            .unwrap_or_else(|_| panic!("Error loading shapefile: {}", file_path.display()));
        visualize_shapefile(map, reader, document, &layer_style, (100.0, 0.0, 0.0));
    }
}

/// Draw a point with a text label
pub fn draw_point(
    point: (f64, f64),
    document: &mut Document,
    layer_style: &LayerStyle,
    label: Option<&str>,
) {
    // Draw the point marker
    let circle = element::Circle::new()
        .set("cx", point.0)
        .set("cy", point.1)
        .set("r", "2") // Small circle for the point
        .set("fill", layer_style.fill)
        .set("stroke", layer_style.stroke)
        .set("stroke-width", layer_style.stroke_width);
    document.append(circle);

    // Draw the label if provided
    if let Some(text) = label {
        let text_element = element::Text::new(text)
            .set("x", point.0 + 5.0) // Offset from point
            .set("y", point.1 + 5.0)
            .set("font-family", "Arial")
            .set("font-size", "12px")
            .set("fill", layer_style.stroke);
        document.append(text_element);
    }
}

/// Visualize a shapefile
pub fn visualize_shapefile(
    map: &Map,
    mut reader: shapefile::Reader<BufReader<File>, BufReader<File>>,
    document: &mut Document,
    layer_style: &LayerStyle,
    camera_location: (f64, f64, f64),
) {
    for result in reader.iter_shapes_and_records() {
        let (shape, record) = result.expect("Error reading data from shapefile");

        /*
        if record.get("name_en").is_some() {
            let label = record.get("name_en").unwrap();
            match label {
                shapefile::dbase::FieldValue::Character(s) => {
                    if let Some(label) = s {
                        println!("label: {:?}", label);
                    }
                }
                _ => {

                }
            }
        }
         */
        match shape {
            //shapefile::Shape::Point(point) => {
            //    let mapped_point = equirectangular_mapping_function(point.x, point.y, map);
            //    draw_point(mapped_point, document, layer_style, label.as_deref());
            //}
            shapefile::Shape::Polygon(polygon) => {
                for ring in polygon.rings() {
                    let visible_count = ring
                        .points()
                        .iter()
                        .filter(|point| point_visible(point.y, point.x, camera_location))
                        .count();

                    if visible_count > 3 {
                        let pts: Vec<_> = ring
                            .points()
                            .iter()
                            .map(|point| orthographic_mapping_function(point.y, point.x, map, camera_location))
                            .collect();
                        draw_polygon(&pts, document, layer_style);
                    }
                }
            }

            shapefile::Shape::Polyline(polyline) => {
                for part in polyline.parts() {
                    let visible_count = part
                        .iter()
                        .filter(|point| point_visible(point.y, point.x, camera_location))
                        .count();

                    if visible_count > 3 {
                        let pts: Vec<_> = part
                            .iter()
                            .map(|point| orthographic_mapping_function(point.y, point.x, map, camera_location))
                            .collect();
                        draw_polyline(&pts, document, layer_style);
                    }
                }
            }

            _ => {}
        }
    }
}

/// Draw a polygon
pub fn draw_polygon(pts: &Vec<(f64, f64)>, document: &mut Document, layer_style: &LayerStyle) {
    let data = element::path::Data::new();
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

/// Draw a polyline
pub fn draw_polyline(pts: &Vec<(f64, f64)>, document: &mut Document, layer_style: &LayerStyle) {
    let data = element::path::Data::new();
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
