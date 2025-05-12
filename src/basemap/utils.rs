//! Module for basemap utils

use std::path::PathBuf;

use image::ImageBuffer;
use resvg::usvg;

use super::Map;

/// Map from lon,lat to a pixel position
#[must_use] pub fn mapping_function(lon: f64, lat: f64, map: &Map) -> (f64, f64) {
    // [-180-180] -> [xmin,xmax]
    // [xmin,xmax] -> [0, 1]
    let mapping_fn1 = |(lon, lat)| {
        (
            (lon - map.lon_min) / (map.lon_max - map.lon_min),
            (lat - map.lat_min) / (map.lat_max - map.lat_min),
        )
    };

    //Equirectangular Projection
    let mapping_fn2 = |(x, y)| {
        (
            f64::from(map.cols) * x,
            f64::from(map.rows) - f64::from(map.rows) * y,
        )
    };

    mapping_fn2(mapping_fn1((lon, lat)))
}

/// Convert from svg to png
pub fn svg_to_png(input_svg_path: &PathBuf, output_png_path: &PathBuf) {
    let tree = create_svg_tree(input_svg_path);
    let pixmap_size = tree.size().to_int_size();
    let mut pixmap = tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height())
        .expect("Error creating bitmap");
    resvg::render(&tree, tiny_skia::Transform::default(), &mut pixmap.as_mut());

    //pixmap.draw_pixmap(x, y, pixmap, paint, transform, mask);
    // Save the pixmap to a file
    pixmap.save_png(output_png_path).expect("Error writing png");
}

/// Convert from svg to buffer
#[must_use] pub fn svg_to_image_buffer(input_svg_path: &PathBuf) -> ImageBuffer<image::Rgba<u8>, Vec<u8>> {
    let tree = create_svg_tree(input_svg_path);
    let pixmap_size = tree.size().to_int_size();
    let mut pixmap = tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height())
        .expect("Error creating bitmap");
    resvg::render(&tree, tiny_skia::Transform::default(), &mut pixmap.as_mut());

    let buffer = pixmap.data_mut().to_vec();

    ImageBuffer::from_raw(pixmap_size.width(), pixmap_size.height(), buffer)
        .expect("Error creating image buffer")
}

/// Create a svg tree from a file
fn create_svg_tree(input_svg_path: &PathBuf) -> usvg::Tree {
    let tree = {
        let mut opt = usvg::Options::<'_> {
            resources_dir: std::fs::canonicalize(input_svg_path)
                .ok()
                .and_then(|p| p.parent().map(std::path::Path::to_path_buf)),
            ..Default::default()
        };

        opt.fontdb_mut().load_system_fonts();

        let svg_data = std::fs::read(input_svg_path).expect("Error reading svg");
        usvg::Tree::from_data(&svg_data, &opt).expect("Error parsing svg")
    };
    tree
}
