//! Module for basemap utils

use std::path::PathBuf;

use image::ImageBuffer;
use resvg::usvg;

/// Convert from svg to png
pub fn svg_to_png(input_svg_path: &PathBuf, output_png_path: &PathBuf) {
    let tree = create_svg_tree(input_svg_path);
    let pixmap_size = tree.size().to_int_size();
    let mut pixmap = tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height())
        .expect("Error creating bitmap");
    resvg::render(&tree, tiny_skia::Transform::default(), &mut pixmap.as_mut());

    // Save the pixmap to a file
    pixmap.save_png(output_png_path).expect("Error writing png");
}

/// Convert from svg to buffer
#[must_use]
pub fn svg_to_image_buffer(input_svg_path: &PathBuf) -> ImageBuffer<image::Rgba<u8>, Vec<u8>> {
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
