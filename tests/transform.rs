use std::path::Path;

use common::read_geotiff;
use geo_types::{Coord, Rect};
use geotiff::RasterType;
use proj::Proj;

mod common;

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

#[test]
fn test_transform_by_model_tie_points_pixel_is_area() {
    test_transform(
        "resources/austrian_capitals_model_tie_points_pixel_is_area.tif",
        RasterType::RasterPixelIsArea,
    )
}

#[test]
fn test_transform_by_model_tie_points_pixel_is_point() {
    test_transform(
        "resources/austrian_capitals_model_tie_points_pixel_is_point.tif",
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
