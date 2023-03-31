use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use solana_reed_solomon_erasure::ReedSolomon;

fn bench_reed_solomon(c: &mut Criterion) {
    let mut group = c.benchmark_group("Reed Solomon");

    for &data_len in &[1024, 4096, 16384] {
        let rs = ReedSolomon::new(data_len, data_len / 2).unwrap();
        let data: Vec<u8> = (0..data_len).map(|i| i as u8).collect();
        let mut shards = rs.encode(&data).unwrap();

        group.bench_with_input(
            BenchmarkId::new("encode", data_len),
            &data_len,
            |b, &data_len| {
                b.iter(|| {
                    let mut shards = shards.clone();
                    rs.encode(&data).unwrap();
                })
            },
        );

        group.bench_with_input(
            BenchmarkId::new("decode", data_len),
            &data_len,
            |b, &data_len| {
                b.iter(|| {
                    let mut shards = shards.clone();
                    rs.decode(&mut shards).unwrap();
                })
            },
        );
    }

    group.finish();
}

criterion_group!(benches, bench_reed_solomon);
criterion_main!(benches);

