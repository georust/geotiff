# A GeoTIFF library for Rust

[![geotiff on crates.io](https://img.shields.io/crates/v/geotiff.svg)](https://crates.io/crates/geotiff)

> [!IMPORTANT]
> This crate is currently undergoing a significant refactoring process to be built on
> top of the [`tiff`](https://crates.io/crates/tiff) crate, so expect breaking changes
> as we work towards a v0.1.0 release sometime in 2024 (contributions are welcome!). See
> the thread at https://github.com/georust/geotiff/issues/7 for more details.

## Motivation (pre-2020)

I needed this library to import elevation models for a routing library. As elevation models usually come in GeoTIFF format, but no such library was available for Rust, I created this library, taking other libraries as inspiration:

* Gavin Baker's TIFF (discontinued, it seems: https://github.com/gavinb/rust-tiff).
* The GeoTIFF library from Whitebox (https://github.com/jblindsay/whitebox-tools/tree/fe9c5be29b01d74d1e135b79cc6328f088755b1d/src/raster/geotiff).

The purpose of this library is to simply read GeoTIFFs, nothing else. It should work for other TIFFs as well, I guess, but TIFFs come in many flavors, and it's not intended to cover them all.

In its current state, it works for very basic GeoTIFFs, which sufficed to extract elevation data for use in the routing library. In case you want to extend the library or have suggestions for improvement, feel free to contact me, open an issue ticket or send a pull request.

You might also consider the [GDAL bindings](https://github.com/georust/gdal) for Rust. Depending on your usecase, it might be easier to use.

## Library Usage

The library exposes a `TIFF` struct that can be used to open GeoTIFFs and interact with them. Its use is simple:

```rust
TIFF::open("geotiff.tif");
```

`TIFF::open(...)` returns an `Option`, depending if the open operation was successful or not. Individual values can then be read (for the moment, only at pixels) using:

```rust
x.get_value_at(longitude, latitude);
```

Where `longitude` corresponds to the `image_length` and `latitude` to the `image_width`. This might be a bit counter intuitive, but seems consistent with GDAL (have to look into this).

Caution: the `longitude` and `latitude` are only in pixels, no coordinate transformations are applied!

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
