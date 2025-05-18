//! Different styles for maps
//!
//! This module provides various map styles that can be used to render the Natural Earth basemap.
//! Each style defines how different geographic features (land, ocean, lakes, etc.) should be displayed.

/// The style of a layer, defining its visual appearance
pub struct LayerStyle<'a> {
    /// The stroke (outline) color of the layer
    /// (e.g. "black", "#000000")
    pub stroke: &'a str,
    /// The fill color of the layer
    /// (e.g. "lightseagreen", "#20B2AA")
    pub fill: &'a str,
    /// The fill opacity of the layer (0.0 to 1.0)
    /// (e.g. "0.5" for 50% opacity)
    pub fill_opacity: &'a str,
    /// The stroke width of the layer in pixels
    /// (e.g. "1" for 1 pixel width)
    pub stroke_width: &'a str,
}

/// A layer in the map, combining a style with its source file
pub struct Layer<'a> {
    /// The visual style of this layer
    pub layer_style: LayerStyle<'a>,
    /// The filename of the shapefile containing this layer's data
    /// (e.g. "`ne_10m_land.shp`")
    pub filename: &'a str,
}

/// A complete map style, defining the background and all layers
pub struct Style<'a> {
    /// The background layer of the map (typically ocean)
    pub background: Layer<'a>,
    /// The ordered list of layers to be drawn on top of the background
    pub layers: Vec<Layer<'a>>,
}

/// Returns a classic map style with a light blue ocean and beige land
///
/// This style uses a traditional color scheme with:
/// - Light seagreen ocean
/// - Wheat-colored land
/// - Sky blue lakes
/// - Silver reefs
/// - Light cyan ice shelves
/// - Alice blue glaciated areas
/// - Sky blue rivers
#[must_use]
pub fn classic_style<'a>() -> Style<'a> {
    Style {
        background: Layer {
            layer_style: LayerStyle {
                stroke: "black",
                fill: "lightseagreen",
                fill_opacity: "0.5",
                stroke_width: "1",
            },
            filename: "ne_10m_ocean.shp",
        },
        layers: vec![
            Layer {
                layer_style: LayerStyle {
                    stroke: "none",
                    fill: "wheat",
                    fill_opacity: "1.0",
                    stroke_width: "0",
                },
                filename: "ne_10m_land.shp",
            },
            Layer {
                layer_style: LayerStyle {
                    stroke: "none",
                    fill: "skyblue",
                    fill_opacity: "1.0",
                    stroke_width: "0",
                },
                filename: "ne_10m_lakes.shp",
            },
            Layer {
                layer_style: LayerStyle {
                    stroke: "none",
                    fill: "silver",
                    fill_opacity: "1.0",
                    stroke_width: "0",
                },
                filename: "ne_10m_reefs.shp",
            },
            Layer {
                layer_style: LayerStyle {
                    stroke: "none",
                    fill: "lightcyan",
                    fill_opacity: "1.0",
                    stroke_width: "0",
                },
                filename: "ne_10m_antarctic_ice_shelves_polys.shp",
            },
            Layer {
                layer_style: LayerStyle {
                    stroke: "none",
                    fill: "aliceblue",
                    fill_opacity: "1.0",
                    stroke_width: "0",
                },
                filename: "ne_10m_glaciated_areas.shp",
            },
            Layer {
                layer_style: LayerStyle {
                    stroke: "skyblue",
                    fill: "none",
                    fill_opacity: "1.0",
                    stroke_width: "1",
                },
                filename: "ne_10m_rivers_lake_centerlines.shp",
            },
        ],
    }
}

/// Returns an ocean-focused style that emphasizes bathymetry
///
/// This style uses a color gradient from light to dark blue to show ocean depth:
/// - Lightest blue (#fff7fb) for shallow water (0m)
/// - Darkest blue (#023858) for deep water (10000m)
/// - Additional layers for land features in muted colors
#[must_use]
pub fn ocean_style<'a>() -> Style<'a> {
    Style {
        background: Layer {
            layer_style: LayerStyle {
                stroke: "none",
                fill: "#023858",
                fill_opacity: "1.0",
                stroke_width: "1",
            },
            filename: "ne_10m_bathymetry_A_10000.shp",
        },
        layers: vec![
            Layer {
                layer_style: LayerStyle {
                    stroke: "none",
                    fill: "#fff7fb",
                    fill_opacity: "1.0",
                    stroke_width: "1",
                },
                filename: "ne_10m_bathymetry_L_0.shp",
            },
            Layer {
                layer_style: LayerStyle {
                    stroke: "none",
                    fill: "#ece7f2",
                    fill_opacity: "1.0",
                    stroke_width: "1",
                },
                filename: "ne_10m_bathymetry_K_200.shp",
            },
            Layer {
                layer_style: LayerStyle {
                    stroke: "none",
                    fill: "#d0d1e6",
                    fill_opacity: "1.0",
                    stroke_width: "1",
                },
                filename: "ne_10m_bathymetry_J_1000.shp",
            },
            Layer {
                layer_style: LayerStyle {
                    stroke: "none",
                    fill: "#a6bddb",
                    fill_opacity: "1.0",
                    stroke_width: "1",
                },
                filename: "ne_10m_bathymetry_I_2000.shp",
            },
            Layer {
                layer_style: LayerStyle {
                    stroke: "none",
                    fill: "#74a9cf",
                    fill_opacity: "1.0",
                    stroke_width: "0",
                },
                filename: "ne_10m_bathymetry_H_3000.shp",
            },
            Layer {
                layer_style: LayerStyle {
                    stroke: "none",
                    fill: "#3690c0",
                    fill_opacity: "1.0",
                    stroke_width: "1",
                },
                filename: "ne_10m_bathymetry_G_4000.shp",
            },
            Layer {
                layer_style: LayerStyle {
                    stroke: "none",
                    fill: "#0570b0",
                    fill_opacity: "1.0",
                    stroke_width: "1",
                },
                filename: "ne_10m_bathymetry_F_5000.shp",
            },
            Layer {
                layer_style: LayerStyle {
                    stroke: "none",
                    fill: "#045a8d",
                    fill_opacity: "1.0",
                    stroke_width: "1",
                },
                filename: "ne_10m_bathymetry_E_6000.shp",
            },
            Layer {
                layer_style: LayerStyle {
                    stroke: "none",
                    fill: "#023858",
                    fill_opacity: "1.0",
                    stroke_width: "1",
                },
                filename: "ne_10m_bathymetry_D_7000.shp",
            },
            Layer {
                layer_style: LayerStyle {
                    stroke: "none",
                    fill: "#023858",
                    fill_opacity: "1.0",
                    stroke_width: "1",
                },
                filename: "ne_10m_bathymetry_C_8000.shp",
            },
            Layer {
                layer_style: LayerStyle {
                    stroke: "none",
                    fill: "#023858",
                    fill_opacity: "1.0",
                    stroke_width: "1",
                },
                filename: "ne_10m_bathymetry_B_9000.shp",
            },
            Layer {
                layer_style: LayerStyle {
                    stroke: "none",
                    fill: "dimgray",
                    fill_opacity: "1.0",
                    stroke_width: "0",
                },
                filename: "ne_10m_land.shp",
            },
            Layer {
                layer_style: LayerStyle {
                    stroke: "none",
                    fill: "skyblue",
                    fill_opacity: "1.0",
                    stroke_width: "0",
                },
                filename: "ne_10m_lakes.shp",
            },
            Layer {
                layer_style: LayerStyle {
                    stroke: "none",
                    fill: "silver",
                    fill_opacity: "1.0",
                    stroke_width: "0",
                },
                filename: "ne_10m_reefs.shp",
            },
            Layer {
                layer_style: LayerStyle {
                    stroke: "none",
                    fill: "lightcyan",
                    fill_opacity: "1.0",
                    stroke_width: "0",
                },
                filename: "ne_10m_antarctic_ice_shelves_polys.shp",
            },
            Layer {
                layer_style: LayerStyle {
                    stroke: "none",
                    fill: "aliceblue",
                    fill_opacity: "1.0",
                    stroke_width: "0",
                },
                filename: "ne_10m_glaciated_areas.shp",
            },
            Layer {
                layer_style: LayerStyle {
                    stroke: "skyblue",
                    fill: "none",
                    fill_opacity: "1.0",
                    stroke_width: "1",
                },
                filename: "ne_10m_rivers_lake_centerlines.shp",
            },
        ],
    }
}

/// Returns a minimalistic grey style for high-resolution maps
///
/// This style uses a simple black and white scheme:
/// - Transparent silver ocean
/// - Black land with white borders
#[must_use]
pub fn grey_style<'a>() -> Style<'a> {
    Style {
        background: Layer {
            layer_style: LayerStyle {
                stroke: "black",
                fill: "silver",
                fill_opacity: "0",
                stroke_width: "0",
            },
            filename: "ne_10m_ocean.shp",
        },
        layers: vec![Layer {
            layer_style: LayerStyle {
                stroke: "white",
                fill: "black",
                fill_opacity: "1.0",
                stroke_width: "1",
            },
            filename: "ne_10m_land.shp",
        }],
    }
}

/// Returns a minimalistic grey style for high-resolution maps
///
/// This style uses a simple black and white scheme:
/// - Transparent background
/// - Black land with white borders
#[must_use]
pub fn grey_style_transparent<'a>() -> Style<'a> {
    Style {
        background: Layer {
            layer_style: LayerStyle {
                stroke: "black",
                fill: "silver",
                fill_opacity: "0",
                stroke_width: "0",
            },
            filename: "ne_10m_ocean.shp",
        },
        layers: vec![Layer {
            layer_style: LayerStyle {
                stroke: "white",
                fill: "black",
                fill_opacity: "1.0",
                stroke_width: "1",
            },
            filename: "ne_10m_land.shp",
        }],
    }
}

/// Returns a minimalistic grey style for low-resolution maps
///
/// This style uses a simple grey scheme:
/// - Semi-transparent silver ocean
/// - Dark grey land without borders
#[must_use]
pub fn grey_style_110<'a>() -> Style<'a> {
    Style {
        background: Layer {
            layer_style: LayerStyle {
                stroke: "black",
                fill: "silver",
                fill_opacity: "0.5",
                stroke_width: "1",
            },
            filename: "ne_110m_ocean.shp",
        },
        layers: vec![Layer {
            layer_style: LayerStyle {
                stroke: "none",
                fill: "dimgray",
                fill_opacity: "1.0",
                stroke_width: "0",
            },
            filename: "ne_110m_land.shp",
        }],
    }
}
