use common::read_geotiff;
use geo_types::{Coord, Rect};
use geotiff::{GeoKeyDirectory, RasterType};

mod common;

#[test]
fn test_load_marbles() {
    let geotiff = read_geotiff("resources/marbles.tif");

    println!("{geotiff:?}");
    assert_eq!(geotiff.raster_width, 1419);
    assert_eq!(geotiff.raster_height, 1001);
    assert_eq!(geotiff.num_samples, 3);
    assert_eq!(
        geotiff.model_extent(),
        Rect::new(
            Coord { x: 0.0, y: 0.0 },
            Coord {
                x: 1419.0,
                y: 1001.0
            }
        )
    );
    assert_eq!(
        geotiff.get_value_at::<u8>(&Coord { x: 761.0, y: 599.0 }, 0),
        Some(147)
    );
    assert_eq!(
        geotiff.get_value_at::<u8>(&Coord { x: 761.0, y: 599.0 }, 1),
        Some(128)
    );
    assert_eq!(
        geotiff.get_value_at::<u8>(&Coord { x: 761.0, y: 599.0 }, 2),
        Some(165)
    );
}

#[test]
fn test_load_zh_dem_25() {
    let geotiff = read_geotiff("resources/zh_dem_25.tif");

    println!("{geotiff:?}");
    assert_eq!(geotiff.raster_width, 399);
    assert_eq!(geotiff.raster_height, 366);
    assert_eq!(geotiff.num_samples, 1);
    assert_eq!(
        geotiff.model_extent(),
        Rect::new(
            Coord {
                x: 677562.5,
                y: 243862.5
            },
            Coord {
                x: 687537.5,
                y: 253012.5
            }
        )
    );
    assert_eq!(
        geotiff.get_value_at::<i16>(
            &Coord {
                x: 677575.0,
                y: 253000.0
            },
            0
        ),
        Some(551)
    );
    assert_eq!(
        geotiff.get_value_at::<i16>(
            &Coord {
                x: 679250.0,
                y: 251875.0
            },
            0
        ),
        Some(530)
    );
    assert_eq!(
        geotiff.get_value_at::<i16>(
            &Coord {
                x: 685700.0,
                y: 249450.0
            },
            0
        ),
        Some(587)
    );

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

    assert_eq!(
        geotiff.model_extent(),
        Rect::new(
            Coord {
                x: 1871032.9538880002,
                y: 662408.6726400064
            },
            Coord {
                x: 1901982.949391994,
                y: 693358.6681440001
            }
        )
    );
}
