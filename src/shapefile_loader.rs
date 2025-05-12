//! Load data from a shapefile
use std::path::PathBuf;

use geo::{LineString, Polygon};
use rustc_hash::FxHashMap;

/// `NaturalEarth` features
pub enum GeoFeature {
    /// [See](http://naturalearthdata.com/downloads/10m-physical-vectors/10m-land/)
    Land,
    /// [See](https://www.naturalearthdata.com/downloads/10m-physical-vectors/10m-reefs/)
    Reefs,
}

/// Load a shapefile. (TODO IMPROVE)
#[must_use]
pub fn load_shapefile(file_path: &PathBuf) -> Vec<Vec<(f64, f64)>> {
    let mut reader = shapefile::Reader::from_path(file_path.clone())
        .unwrap_or_else(|_| panic!("Error loading shapefile:{file_path:?}"));

    let mut polygons = vec![];
    for result in reader.iter_shapes_and_records() {
        let (shape, _record) = result.expect("Error extracting data from shapefile");
        if let shapefile::Shape::Polygon(shapefile_polygon) = shape {
            for ring in shapefile_polygon.rings() {
                let points: Vec<_> = ring
                    .points()
                    .iter()
                    .map(|point| (point.x, point.y))
                    .collect();
                polygons.push(points);
            }
        }
    }
    polygons
}

/// Load the Geopolys
#[must_use]
pub fn load_geopolys(geographical_feature: &GeoFeature) -> FxHashMap<usize, Polygon> {
    let file_path = match geographical_feature {
        GeoFeature::Land => PathBuf::from("data/basemap/10m_physical/ne_10m_land.shp"),
        GeoFeature::Reefs => PathBuf::from("data/basemap/10m_physical/ne_10m_reefs.shp"),
    };

    let mut polygons = load_shapefile(&file_path);

    // Generate Geo Polygons
    let mut geo_polys = FxHashMap::default();
    for (index, poly) in polygons.drain(..).enumerate() {
        let geo_poly = Polygon::new(LineString::from(poly), vec![]);
        geo_polys.insert(index, geo_poly);
    }
    geo_polys
}
