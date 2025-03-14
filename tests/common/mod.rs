use std::error::Error;
use std::fs::File;
use std::path::Path;

use geotiff::GeoTiff;

pub fn read_geotiff<P: AsRef<Path>>(path: P) -> GeoTiff {
    match GeoTiff::read(File::open(path).expect("File I/O error")) {
        Ok(g) => g,
        Err(e) => {
            eprintln!("Error reading GeoTIFF: {}", e);
            if let Some(source) = e.source() {
                eprintln!("Caused by: {}", source);
            }
            panic!();
        }
    }
}
