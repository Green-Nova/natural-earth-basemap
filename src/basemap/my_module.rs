use std::fs::File;
use std::io::BufReader;

use shapefile::{Point, Reader, Shape};
use svg::{Document, node::element};

use super::draw_svg::{draw_polygon, draw_polyline, draw_text};
use super::utils::mapping_function;
use super::{Map, styles::LayerStyle};

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
        let (shape, record) = result.expect("Error reading data from shapefile");

        //println!("Record: {:?}", record);
//println!("Record: {:?}", record.get("scalerank"));

       
        match shape {
            Shape::Point(point) => {
                println!("Point: {:?}", point);
                let name = record.get("name").unwrap();

                let label = match name {
                    shapefile::dbase::FieldValue::Character(s) => {
                        if let Some(label) = s {
                           label
                        } else {
                            &"".to_string()
                        }
                    }
                    _ => {
                        &"".to_string()
                    }
                };

                point_fn(&point, label, map, document, layer_style);
            }
            Shape::Multipoint(multi_point) => {
                for point in multi_point.points() {
                    println!("MPoint: {:?}", point);
                }
            }
            Shape::Polygon(polygon) => {
                for ring in polygon.rings() {
                    polygon_fn(ring.points(), map, document, layer_style);
                }
            }

            Shape::Polyline(polyline) => {
                for part in polyline.parts() {
                    polyline_fn(part, map, document, layer_style);
                }
            }
            _ => {}
        }
    }
}

fn point_fn(point: &Point, label: &str, map: &Map, document: &mut Document, layer_style: &LayerStyle) {
    let pt = mapping_function(point.x, point.y, map);
    draw_text(pt, label, document, 12, layer_style.fill);
}


fn polyline_fn(part: &[Point], map: &Map, document: &mut Document, layer_style: &LayerStyle) {
    let data = element::path::Data::new();
    let pts: Vec<_> = part
        .iter()
        .map(|point| mapping_function(point.x, point.y, map))
        .collect();

    // TODO Enabling this will clip polylines outside the map, reducing file size
    /*
    // Check if the polyline is entirely within the map
    if !pts.iter().all(|pt| {
        pt.0 >= 0.0
            && pt.0 <= f64::from(map.cols)
            && pt.1 >= 0.0
            && pt.1 <= f64::from(map.rows)
    }) {
        continue;
    }
    */
    draw_polyline(&pts, document, layer_style, data);
}

fn polygon_fn(ring: &[Point], map: &Map, document: &mut Document, layer_style: &LayerStyle) {
    let data = element::path::Data::new();
    let pts: Vec<_> = ring
        .iter()
        .map(|point| mapping_function(point.x, point.y, map))
        .collect();

    // TODO Enabling this will clip polygons outside the map, reducing file size
    // TODO account for cases like the ocean, which are completely outside the map
    // Check if the polygon is entirely within the map
    //if !pts.iter().all(|pt| pt.0 >= 0.0 && pt.0 <= map.cols as f64 && pt.1 >= 0.0 && pt.1 <= map.rows as f64) {
    //    continue;
    //}
    draw_polygon(&pts, document, layer_style, data);
}
