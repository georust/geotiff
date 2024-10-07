use std::fs::File;
use std::path::Path;

use geotiff::{GeoKeyDirectory, GeoTiff, RasterType};

fn read_geotiff<P: AsRef<Path>>(path: P) -> GeoTiff {
    GeoTiff::read(File::open(path).expect("File I/O error")).expect("File I/O error")
}

#[test]
fn test_load_marbles() {
    let geotiff = read_geotiff("resources/marbles.tif");

    println!("{geotiff:?}");
    assert_eq!(geotiff.raster_width, 1419);
    assert_eq!(geotiff.raster_height, 1001);
    assert_eq!(geotiff.num_samples, 3);
    assert_eq!(geotiff.get_value_at::<u8>(761, 599, 0), 147);
    assert_eq!(geotiff.get_value_at::<u8>(761, 599, 1), 128);
    assert_eq!(geotiff.get_value_at::<u8>(761, 599, 2), 165);
}

#[test]
fn test_load_zh_dem_25() {
    let geotiff = read_geotiff("resources/zh_dem_25.tif");

    println!("{geotiff:?}");
    assert_eq!(geotiff.raster_width, 399);
    assert_eq!(geotiff.raster_height, 366);
    assert_eq!(geotiff.num_samples, 1);
    assert_eq!(geotiff.get_value_at::<i16>(0, 0, 0), 551);
    assert_eq!(geotiff.get_value_at::<i16>(67, 45, 0), 530);
    assert_eq!(geotiff.get_value_at::<i16>(325, 142, 0), 587);

    assert_eq!(
        geotiff.geo_key_directory,
        GeoKeyDirectory {
            ..Default::default()
        }
    );
}

#[test]
fn test_load_merc() {
    let geotiff = read_geotiff("resources/merc.tif");

    println!("{geotiff:?}");
    assert_eq!(geotiff.raster_width, 200);
    assert_eq!(geotiff.raster_height, 200);
    assert_eq!(geotiff.num_samples, 1);

    assert_eq!(
        geotiff.geo_key_directory,
        GeoKeyDirectory {
            key_directory_version: 1,
            key_revision: 1,
            minor_revision: 2,
            model_type: Some(1),
            raster_type: Some(RasterType::RasterPixelIsArea),
            geog_geodetic_datum: Some(6267),
            geog_ellipsoid: Some(7008),
            projected_type: Some(32767),
            proj_citation: Some("Mercator North American 1927".into()),
            projection: Some(32767),
            proj_coord_trans: Some(7),
            proj_linear_units: Some(9001),
            proj_nat_origin_long: Some(-90.0),
            proj_nat_origin_lat: Some(30.0),
            proj_false_easting: Some(0.001),
            proj_false_northing: Some(0.002),
            proj_center_lat: Some(34.0),
            proj_scale_at_nat_origin: Some(0.829916312080482),
            ..Default::default()
        }
    );
}
