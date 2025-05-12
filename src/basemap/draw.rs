//! Draw the basemap
use std::{fs::File, io::BufReader, path::PathBuf};

use shapefile::{Reader, Shape};
use styles::{LayerStyle, grey_style};
use svg::{Document, Node, node::element};
use utils::mapping_function;

use super::{Map, styles, utils};

/// Set the background of the map
pub fn set_background(map: &Map, document: &mut Document) {
    let data = element::path::Data::new();
    let data = data
        .move_to((0, 0))
        .line_to((0, map.rows))
        .line_to((map.cols, map.rows))
        .line_to((map.cols, 0));

    let path = element::Path::new()
        .set("stroke", "white")
        .set("fill", "white")
        .set("fill-opacity", "0")
        .set("d", data);
    document.append(path);
}

/// Draw the basemap
pub fn draw_basemap(map: &Map, document: &mut Document) {
    let layers = grey_style();
    //set_background(map, document);
    for (filename, layer_style) in layers {
        let file_path = PathBuf::from(format!("data/10m_physical/{filename}"));
        let reader = shapefile::Reader::from_path(file_path).expect("Error loading shapefile");
        visualize_shapefile(map, reader, document, layer_style);
    }
}

/// Visualize a shapefile
pub fn visualize_shapefile(
    map: &Map,
    mut reader: Reader<BufReader<File>, BufReader<File>>,
    document: &mut Document,
    layer_style: LayerStyle,
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

                    // TODO Check if this falls entirely within the map
                    // TODO move this draw polygon function
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
            }

            Shape::Polyline(polyline) => {
                for part in polyline.parts() {
                    let data = element::path::Data::new();
                    let pts: Vec<_> = part
                        .iter()
                        .map(|point| mapping_function(point.x, point.y, map))
                        .collect();

                    // TODO move this draw line function
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
            }
            _ => {}
        }
    }
}
