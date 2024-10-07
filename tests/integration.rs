use std::fs::File;
use std::path::Path;

use geo_types::{Coord, Rect};
use geotiff::{GeoKeyDirectory, GeoTiff, RasterType};
use proj::Proj;

trait RectExt {
    fn rounded(self) -> Self;
}

impl RectExt for Rect {
    fn rounded(mut self) -> Self {
        let factor = 10f64.powi(8);
        let mut min = self.min();
        let mut max = self.max();
        min.x = (min.x * factor).round() / factor;
        min.y = (min.y * factor).round() / factor;
        max.x = (max.x * factor).round() / factor;
        max.y = (max.y * factor).round() / factor;
        self.set_min(min);
        self.set_max(max);
        self
    }
}

const BREGENZ: Coord = Coord {
    x: 9.74926,
    y: 47.50315,
};
const EISENSTADT: Coord = Coord {
    x: 15.43301,
    y: 47.06298,
};
const GRAZ: Coord = Coord {
    x: 15.43301,
    y: 47.06298,
};
const INNSBRUCK: Coord = Coord {
    x: 11.39960,
    y: 47.26239,
};
const KLAGENFURT: Coord = Coord {
    x: 14.31528,
    y: 46.62366,
};
const LINZ: Coord = Coord {
    x: 14.30571,
    y: 48.27532,
};
const SALZBURG: Coord = Coord {
    x: 13.05345,
    y: 47.80763,
};
const SANKT_POELTEN: Coord = Coord {
    x: 15.62291,
    y: 48.20440,
};
const VIENNA: Coord = Coord {
    x: 16.37499,
    y: 48.22158,
};

const WHITE: u8 = 255;
const BLACK: u8 = 0;

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

#[test]
fn test_transform_by_model_tie_point_and_pixel_scale_pixel_is_area() {
    test_transform(
        "resources/austrian_capitals_model_tie_point_and_pixel_scale_pixel_is_area.tif",
        RasterType::RasterPixelIsArea,
    )
}

#[test]
fn test_transform_by_model_tie_point_and_pixel_scale_pixel_is_point() {
    test_transform(
        "resources/austrian_capitals_model_tie_point_and_pixel_scale_pixel_is_point.tif",
        RasterType::RasterPixelIsPoint,
    )
}

#[test]
fn test_transform_by_model_transformation_pixel_is_area() {
    test_transform(
        "resources/austrian_capitals_model_transformation_pixel_is_area.tif",
        RasterType::RasterPixelIsArea,
    )
}

#[test]
fn test_transform_by_model_transformation_pixel_is_point() {
    test_transform(
        "resources/austrian_capitals_model_transformation_pixel_is_point.tif",
        RasterType::RasterPixelIsPoint,
    )
}

fn test_transform<P: AsRef<Path>>(path: P, raster_type: RasterType) {
    let geotiff = read_geotiff(path);

    let proj = Proj::new_known_crs(
        "EPSG:4326",
        &format!("EPSG:{}", geotiff.geo_key_directory.projected_type.unwrap()),
        None,
    )
    .unwrap();

    let mut bregenz = proj.project(BREGENZ, false).unwrap();
    let mut eisenstadt = proj.project(EISENSTADT, false).unwrap();
    let mut graz = proj.project(GRAZ, false).unwrap();
    let mut innsbruck = proj.project(INNSBRUCK, false).unwrap();
    let mut klagenfurt = proj.project(KLAGENFURT, false).unwrap();
    let mut linz = proj.project(LINZ, false).unwrap();
    let mut salzburg = proj.project(SALZBURG, false).unwrap();
    let mut sankt_poelten = proj.project(SANKT_POELTEN, false).unwrap();
    let mut vienna = proj.project(VIENNA, false).unwrap();
    let mut expected_model_extent = Rect::new(
        Coord {
            x: 4302000.0,
            y: 2621000.0,
        },
        Coord {
            x: 4809000.0,
            y: 2811000.0,
        },
    );

    // If raster type is RasterPixelIsPoint, shift coordinates accordingly
    if raster_type == RasterType::RasterPixelIsPoint {
        let mut min = expected_model_extent.min();
        let mut max = expected_model_extent.max();

        let mut coords = [
            &mut bregenz,
            &mut eisenstadt,
            &mut graz,
            &mut innsbruck,
            &mut klagenfurt,
            &mut linz,
            &mut salzburg,
            &mut sankt_poelten,
            &mut vienna,
            &mut min,
            &mut max,
        ];

        for coord in coords.iter_mut() {
            coord.x -= 500.0;
            coord.y += 500.0;
        }

        expected_model_extent.set_min(min);
        expected_model_extent.set_max(max);
    }

    assert_eq!(geotiff.model_extent().rounded(), expected_model_extent);

    // Non-capital location should return background color
    assert_eq!(
        geotiff.get_value_at::<u8>(&expected_model_extent.center(), 0),
        Some(WHITE)
    );

    // A location outside the model extent should return None

    let min = expected_model_extent.min();
    let max = expected_model_extent.max();

    assert_eq!(
        geotiff.get_value_at::<u8>(&Coord { x: min.x, y: min.y }, 0),
        None
    );
    assert_eq!(
        geotiff.get_value_at::<u8>(
            &Coord {
                x: max.x + 1.0,
                y: max.y + 1.0,
            },
            0
        ),
        None
    );

    // Each capital location should return Some(BLACK)
    assert_eq!(geotiff.get_value_at::<u8>(&bregenz, 0), Some(BLACK));
    assert_eq!(geotiff.get_value_at::<u8>(&eisenstadt, 0), Some(BLACK));
    assert_eq!(geotiff.get_value_at::<u8>(&graz, 0), Some(BLACK));
    assert_eq!(geotiff.get_value_at::<u8>(&innsbruck, 0), Some(BLACK));
    assert_eq!(geotiff.get_value_at::<u8>(&klagenfurt, 0), Some(BLACK));
    assert_eq!(geotiff.get_value_at::<u8>(&linz, 0), Some(BLACK));
    assert_eq!(geotiff.get_value_at::<u8>(&salzburg, 0), Some(BLACK));
    assert_eq!(geotiff.get_value_at::<u8>(&sankt_poelten, 0), Some(BLACK));
    assert_eq!(geotiff.get_value_at::<u8>(&vienna, 0), Some(BLACK));
}
