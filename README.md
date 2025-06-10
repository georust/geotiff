# A GeoTIFF library for Rust

[![geotiff on crates.io](https://img.shields.io/crates/v/geotiff.svg)](https://crates.io/crates/geotiff)
[![docs.rs](https://img.shields.io/docsrs/geotiff?label=docs.rs%20latest)](https://docs.rs/geotiff)

> [!IMPORTANT]
> This crate has went through a significant refactoring process to be built on top of
> the [`tiff`](https://crates.io/crates/tiff) crate in 2024/2025, but do expect breaking
> changes post v0.1.0, as we may decide to do another redesign to work towards
> asynchronous reading (see thread at https://github.com/georust/geotiff/issues/13).
> That said, there are still many features to add, so contributions are welcome!

## Motivation (pre-2020)

I needed this library to import elevation models for a routing library. As elevation models usually come in GeoTIFF format, but no such library was available for Rust, I created this library, taking other libraries as inspiration:

* Gavin Baker's TIFF (discontinued, it seems: https://github.com/gavinb/rust-tiff).
* The GeoTIFF library from Whitebox (https://github.com/jblindsay/whitebox-tools/tree/fe9c5be29b01d74d1e135b79cc6328f088755b1d/src/raster/geotiff).

The purpose of this library is to simply read GeoTIFFs, nothing else. It should work for other TIFFs as well, I guess, but TIFFs come in many flavors, and it's not intended to cover them all.

In its current state, it works for very basic GeoTIFFs, which sufficed to extract elevation data for use in the routing library. In case you want to extend the library or have suggestions for improvement, feel free to contact me, open an issue ticket or send a pull request.

You might also consider the [GDAL bindings](https://github.com/georust/gdal) for Rust. Depending on your usecase, it might be easier to use.

## Library Usage

The library exposes a `GeoTiff` struct that can be used to open GeoTIFFs and interact with them. Its use is simple:

```rust
use geotiff::GeoTiff;

let reader = GeoTiff::read("geotiff.tif")?;
```

`GeoTiff::read(...)` returns a `TiffResult<GeoTiff>`, and depending on whether the read
operation was successful or not, individual values can then be read (for the moment,
only at pixels) using:

```rust
use geo_types::Coord;

reader.get_value_at::<u8>(&Coord { x: 10, y: 20 }, 0);
```

Where `x` corresponds to Longitude/Eastings and `y` to Latitude/Northings, depending on
whether the GeoTIFF file uses a geographic or projected reference system. The `0` refers
to the band/channel number.

## Development and Testing

Simply run the tests using:

```
cargo test
```

## TIFF Basics

Several documents describe the structure of a (Geo)TIFF:

* The official TIFF specification: http://download.osgeo.org/geotiff/spec/tiff6.pdf.
* The official GeoTIFF specitication: http://download.osgeo.org/geotiff/spec/geotiff.rtf
* The article "GeoTIFF – A standard image file format for GIS applications" by Mahammad and Ramakrishnan: https://www.geospatialworld.net/article/geotiff-a-standard-image-file-format-for-gis-applications/
