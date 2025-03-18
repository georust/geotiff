# Benchmarks

[![CodSpeed Badge](https://img.shields.io/endpoint?url=https://codspeed.io/badge.json)](https://codspeed.io/georust/geotiff)

Measure the throughput of reading GeoTIFF files using this crate!

Steps to run:

1. Download the tar archive of GeoTIFF files, placing them in the `resources/` folder

   ```bash
   wget https://s3.us-east-2.amazonaws.com/geotiff-benchmark-sample-files/geotiff_sample_files.tar.gz -P resources
   ```

2. Extract the tar archive

   ```bash
   tar --extract --verbose --file resources/geotiff_sample_files.tar.gz
   ```

3. Run the benchmarks

    ```bash
    cargo bench
    ```

## Results

TODO

## References
- https://github.com/kokoalberti/geotiff-benchmark
