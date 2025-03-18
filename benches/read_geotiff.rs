use std::{fs::File, time::Duration};

use criterion::{criterion_group, criterion_main, Criterion, Throughput};
use geotiff::GeoTiff;
use tiff::TiffResult;

fn read_geotiff(fpath: &str) -> TiffResult<()> {
    let file = File::open(fpath)?;
    let _geotiff = GeoTiff::read(file)?;
    Ok(())
}

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("read_geotiff_50MB");
    group.throughput(Throughput::BytesDecimal(50e6 as u64)); // 50MB filesize
    group.measurement_time(Duration::from_secs(10));

    group.bench_function("u8_dtype", move |b| {
        b.iter(|| read_geotiff("resources/byte_50m.tif"))
    });
    group.bench_function("i16_dtype", move |b| {
        b.iter(|| read_geotiff("resources/int16_50m.tif"))
    });
    group.bench_function("f32_dtype", move |b| {
        b.iter(|| read_geotiff("resources/float32_50m.tif"))
    });
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
