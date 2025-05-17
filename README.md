# Natural Earth Basemap

A Rust library for generating beautiful basemaps using Natural Earth data. This project provides tools to create high-quality map visualizations with customizable styling and rendering options.

## Features

- Process and render Natural Earth vector data
- Generate SVG and raster map outputs
- Customizable styling and visualization options
- Built with Rust for performance and reliability
- Uses modern graphics libraries (tiny-skia, resvg) for high-quality rendering

## Prerequisites

- Rust 1.75 or later
- Cargo (Rust's package manager)

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
natural-earth-basemap = "0.1.2"
```

## Usage

```rust
use natural_earth_basemap::visualization;

// Example code will be added as the library matures
```

## Project Structure

- `src/` - Source code
  - `visualization/` - Map visualization and rendering modules
  - `bin/example.rs` - Example usage
- `data/` - Natural Earth data files

## Dependencies

- `clap` - Command-line argument parsing
- `geo` - Geospatial data structures
- `image` - Image processing
- `resvg` - SVG rendering
- `shapefile` - Shapefile parsing
- `svg` - SVG generation
- `tiny-skia` - 2D graphics rendering

## Development

To build the project:

```bash
cargo build
```

To run the example:

```bash
cargo run --bin example
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License.

## Acknowledgments

- [Natural Earth](https://www.naturalearthdata.com/) for providing the base map data
- All the Rust ecosystem contributors whose work made this project possible
