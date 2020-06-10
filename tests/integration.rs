extern crate rust_geotiff as tiff;

use tiff::TIFF;

/*#[test]
fn test_load() {
    match TIFF::open("resources/marbles.tif") {
        Ok(x) => println!("Read tiff {}", x),
        Err(e) => println!("File I/O Error: {:?}", e),
    }
}
*/

#[test]
fn test_load_2() -> Result<(), std::io::Error> {
    let x = TIFF::open("resources/zh_dem_25.tif")?;
    assert_eq!(x.image_data.len(), 366);
    assert_eq!(x.image_data[0].len(), 399);

    assert_eq!(x.get_value_at(0, 0).round() as usize, 551);
    assert_eq!(x.get_value_at(45, 67).round() as usize, 530);
    assert_eq!(x.get_value_at(142, 325).round() as usize, 587);
    Ok(())
}

/* TODO Not supported yet, as this uses TileByteCounts instead of StripByteCounts.
#[test]
fn test_load_3() {
    match TIFF::open("resources/large_tif/DEM_ZH.tif") {
        Ok(x) => {
            assert_eq!(x.image_data.len(), 366);
            assert_eq!(x.image_data[0].len(), 399);

            assert_eq!(x.get_value_at(0, 0), 551);
            assert_eq!(x.get_value_at(45, 67), 530);
            assert_eq!(x.get_value_at(142, 325), 587);
        },
        Err(e) => println!("File I/O Error: {:?}", e),
    }
}
*/

#[test]
fn test_load_4() -> Result<(), std::io::Error> {
    let t = TIFF::open("resources/mapzen-geotiff-14-10348-7801.tif")?;
    assert_eq!(t.image_data.len(), 512);
    assert_eq!(t.image_data[0].len(), 512);
    //println!("{:?}", t);

    assert_eq!(t.get_value_at(0,0).round() as usize, 467);

    Ok(())
}
