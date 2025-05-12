//! Different styles for maps

/// The style of a layer
pub struct LayerStyle<'a> {
    /// The stroke color of the layer
    /// (e.g. "black")
    pub stroke: &'a str,
    /// The fill color of the layer
    /// (e.g. "lightseagreen")
    pub fill: &'a str,
    /// The fill opacity of the layer
    /// (e.g. "0.5")
    pub fill_opacity: &'a str,
    /// The stroke width of the layer
    /// (e.g. 1)
    pub stroke_width: &'a str,
}

/// Classic map style
#[must_use]
pub fn classic_style<'a>() -> Vec<(&'static str, LayerStyle<'a>)> {
    vec![
        (
            "ne_10m_ocean.shp",
            LayerStyle {
                stroke: "black",
                fill: "lightseagreen",
                fill_opacity: "0.5",
                stroke_width: "1",
            },
        ),
        (
            "ne_10m_land.shp",
            LayerStyle {
                stroke: "none",
                fill: "wheat",
                fill_opacity: "1.0",
                stroke_width: "0",
            },
        ),
        (
            "ne_10m_lakes.shp",
            LayerStyle {
                stroke: "none",
                fill: "skyblue",
                fill_opacity: "1.0",
                stroke_width: "0",
            },
        ),
        (
            "ne_10m_reefs.shp",
            LayerStyle {
                stroke: "none",
                fill: "silver",
                fill_opacity: "1.0",
                stroke_width: "0",
            },
        ),
        (
            "ne_10m_antarctic_ice_shelves_polys.shp",
            LayerStyle {
                stroke: "none",
                fill: "lightcyan",
                fill_opacity: "1.0",
                stroke_width: "0",
            },
        ),
        (
            "ne_10m_glaciated_areas.shp",
            LayerStyle {
                stroke: "none",
                fill: "aliceblue",
                fill_opacity: "1.0",
                stroke_width: "0",
            },
        ),
        (
            "ne_10m_rivers_lake_centerlines.shp",
            LayerStyle {
                stroke: "skyblue",
                fill: "none",
                fill_opacity: "1.0",
                stroke_width: "1",
            },
        ),
    ]
}

/// Minimalistic grey style for High resolution
#[must_use]
pub fn grey_style<'a>() -> Vec<(&'static str, LayerStyle<'a>)> {
    vec![
        (
            "ne_10m_ocean.shp",
            LayerStyle {
                stroke: "black",
                fill: "silver",
                fill_opacity: "0",
                stroke_width: "0",
            },
        ),
        (
            "ne_10m_land.shp",
            LayerStyle {
                stroke: "white",
                fill: "black",
                fill_opacity: "1.0",
                stroke_width: "1",
            },
        ),
    ]
}

/// Minimalistic grey style for Low resolution
#[must_use]
pub fn grey_style_110<'a>() -> Vec<(&'static str, LayerStyle<'a>)> {
    vec![
        (
            "ne_110m_ocean.shp",
            LayerStyle {
                stroke: "black",
                fill: "silver",
                fill_opacity: "0.5",
                stroke_width: "1",
            },
        ),
        (
            "ne_110m_land.shp",
            LayerStyle {
                stroke: "none",
                fill: "dimgray",
                fill_opacity: "1.0",
                stroke_width: "0",
            },
        ),
    ]
}
