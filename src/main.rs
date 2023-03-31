use criterion::{black_box, criterion_group, criterion_main, Criterion};
use solana_program::retransmit::{retransmit, ErasureConfig, ErasureInput};

fn erasure_bench(c: &mut Criterion) {
    let config = ErasureConfig {
        data: 128,
        parity: 32,
        chunk_size: 1024,
    };
    let input = ErasureInput::new(vec![0u8; config.data * config.chunk_size]);
    c.bench_function("erasure_128_32_1024", |b| {
        b.iter(|| {
            let mut output = vec![0u8; config.chunks() * config.chunk_size];
            retransmit(&config, &input, &mut output);
        })
    });
}

criterion_group!(benches, erasure_bench);
criterion_main!(benches);

