//! Draw the basemap
use std::{fs::File, io::BufReader, path::PathBuf};

use styles::LayerStyle;
use svg::{Document, Node, node::element};

use super::{projections::{orthographic::orthographic_mapping_function, Point3D}, styles, Map};

// TODO add text
// TODO add graticules
// TODO Add svg shading


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

/// Visualize a shapefile
pub fn visualize_shapefile(
    map: &Map,
    mut reader: shapefile::Reader<BufReader<File>, BufReader<File>>,
    document: &mut Document,
    layer_style: &LayerStyle,
    camera_location: (f64, f64, f64),
) {
    let camera_point = Point3D::new(camera_location.0, camera_location.1, camera_location.2);
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
            /*
            shapefile::Shape::Point(point) => {
                // DO Stuff
                //1. Transform to ECEF
                    //2. Transform to camera coordinates
                    //3. Check if the point is visible
                    //4. If visible, transform to screen coordinates
                    //5. If visible in screen coordinates, draw the point
            }
             */
            shapefile::Shape::Polygon(polygon) => {
                for ring in polygon.rings() {
                    let mut points = Vec::new();
                    for point in ring.points() {
                        //1. Transform to ECEF
                        let mut point3d = Point3D::from_lat_lon(point.y, point.x);
                        //2. Transform to camera coordinates
                        point3d.rotate_point((0.0, 0.0, 0.0));
                        //3. Check if the point is visible
                        if point3d.is_visible(&camera_point) {
                            let (x, y) = orthographic_mapping_function(point3d, map, camera_location);
                            //4. If visible, transform to screen coordinates
                            //5. If visible in screen coordinates, draw the point
                            points.push((x, y));
                        }
                    }
                    if points.len() > 3 {
                        draw_polygon(&points, document, layer_style);
                    }
                }
            } 

            /*
            shapefile::Shape::Polyline(polyline) => {
                for part in polyline.parts() {
                    //1. Transform to ECEF
                    //2. Transform to camera coordinates
                    //3. Check if the PolyLine is completely/partially/not visible
                    //4. If visible, transform to screen coordinates
                    //5. If visible in screen coordinates, draw the PolyLine
                }
            }
            */

            _ => {}
        }
    }
}



/// Draw a point with a text label
pub fn draw_point(
    point: (f64, f64),
    document: &mut Document,
    layer_style: &LayerStyle,
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
}

/// Draw text
pub fn draw_text(text: &str, document: &mut Document, layer_style: &LayerStyle) {
    // Do Stuff
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
