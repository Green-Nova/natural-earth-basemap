//! Draw the basemap
use std::{fs::File, io::BufReader, path::PathBuf};


use styles::LayerStyle;
use svg::{Document, Node, node::element};
//use projections::equirectangular_mapping_function;
use projections::orthographic_mapping_function;


use super::{projections::{self, point_visible}, styles, Map};

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
        visualize_shapefile(map, reader, document, &layer_style);
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
        .set("r", "2")  // Small circle for the point
        .set("fill", layer_style.fill)
        .set("stroke", layer_style.stroke)
        .set("stroke-width", layer_style.stroke_width);
    document.append(circle);

    // Draw the label if provided
    if let Some(text) = label {
        let text_element = element::Text::new(text)
            .set("x", point.0 + 5.0)  // Offset from point
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
                    let data = element::path::Data::new();

                    // 1. Check if points are visible.
                    // 2. If Points are not visible, keep going until we get to a visible point
                    
                    let points = ring.points();
                    let mut interp_state = false;
                    let mut mapped_points = Vec::new();
                    for i in 0..points.len(){
                        let point = points[i];
                        
                        if  point_visible(point.y, point.x){
                            let (px,py) = orthographic_mapping_function(point.y, point.x, map);
                            mapped_points.push(Some((px,py)));
                            interp_state = false;
                        }else{
                            if interp_state == false{
                                mapped_points.push(None);
                                interp_state = true
                            }
                        }                       
                    }
                    // 3. Interpolate points onto ARC that lies between missing points
                    // Now for each None, we need to replace it with N points that are interpolated between.
                    
                    



                    /*
                    let pts: Vec<_> = ring
                        .points()
                        .iter()
                        //.map(|point| equirectangular_mapping_function(point.y, point.x, map))
                        .filter(|point| point_visible(point.y, point.x))
                        .map(|point| orthographic_mapping_function(point.y, point.x, map))
                        .collect();
                     */
                    // TODO account for cases like the ocean, which are completely outside the map
                    // Check if the polygon is entirely within the map
                    //if !pts.iter().all(|pt| pt.0 >= 0.0 && pt.0 <= map.cols as f64 && pt.1 >= 0.0 && pt.1 <= map.rows as f64) {
                    //    continue;
                    //}
                    if pts.len()>3{
                        draw_polygon(&pts, document, layer_style, data);
                    }
                }
            }

            /*
            shapefile::Shape::Polyline(polyline) => {
                for part in polyline.parts() {
                    let data = element::path::Data::new();
                    let pts: Vec<_> = part
                        .iter()
                        //.map(|point| equirectangular_mapping_function(point.y, point.x, map))
                        .filter(|point| point_visible(point.y, point.x))
                        .map(|point| orthographic_mapping_function(point.y, point.x, map))
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
                    if pts.len()>0{
                        println!("DING!");
                        draw_polyline(&pts, document, layer_style, data);
                    }
                }
            }
            */
            _ => {}
        }
    }
}




/// Draw a polygon
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

/// Draw a polyline
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

