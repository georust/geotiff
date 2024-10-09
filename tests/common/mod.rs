use std::fs::File;
use std::path::Path;

use geotiff::GeoTiff;

pub fn read_geotiff<P: AsRef<Path>>(path: P) -> GeoTiff {
    GeoTiff::read(File::open(path).expect("File I/O error")).expect("File I/O error")
}
