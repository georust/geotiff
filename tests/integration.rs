extern crate rust_geotiff as tiff;

use tiff::TIFF;

//#[test]
fn test_load() {
    match TIFF::open("resources/marbles.tif") {
        Ok(x) => println!("Read tiff {}", x),
        Err(e) => println!("File I/O Error: {:?}", e),
    }
}

#[test]
fn test_load_2() {
    match TIFF::open("resources/zh_dem_25.tif") {
        Ok(x) => println!("Read tiff {}", x),
        Err(e) => println!("File I/O Error: {:?}", e),
    }
}
