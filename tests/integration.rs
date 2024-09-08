use std::fs::File;
use std::path::Path;

use geotiff::GeoTiff;

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
}
