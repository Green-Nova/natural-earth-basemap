use clap::Parser;
use natural_earth_basemap::basemap::{ draw_map, styles,Map};

/// Visualize Example
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Map cols (Width of map in pixels )
    #[arg(long, default_value_t = 16_000)]
    map_cols: u32,

    /// Map rows (Width of map in pixels )
    #[arg(long, default_value_t = 8_000)]
    map_rows: u32,

    /// Minimum longitude for map in decimal degrees
    #[arg(long, default_value_t = -170.0, allow_hyphen_values = true)]
    lon_min: f64,

    /// Minimum latitude for map in decimal degrees
    #[arg(long, default_value_t = -80.0, allow_hyphen_values = true)]
    lat_min: f64,

    /// Maximum longitude for map in decimal degrees
    #[arg(long, default_value_t = 170.0, allow_hyphen_values = true)]
    lon_max: f64,

    /// Maximum latitude for map in decimal degrees
    #[arg(long, default_value_t = 80.0, allow_hyphen_values = true)]
    lat_max: f64,

    /// Output file path
    #[arg(long, default_value = "Map.svg")]
    output_path: std::path::PathBuf,
}

pub fn main() {
    let args = Args::parse();

    let map = Map {
        rows: args.map_rows as i32,
        cols: args.map_cols as i32,
        lon_min: args.lon_min,
        lon_max: args.lon_max,
        lat_min: args.lat_min,
        lat_max: args.lat_max,
    };

    let output_path = args.output_path;

    draw_map(&map,&styles::ocean_style(), &output_path);
}


