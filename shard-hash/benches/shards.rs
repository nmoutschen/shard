use criterion::{black_box, criterion_group, criterion_main, Criterion};
use shard_hash::{get_first_shard, get_shards};

fn benchmark(c: &mut Criterion) {
    c.bench_function("get_first_shard", |b| {
        b.iter(|| {
            let _shards = get_first_shard(black_box(2237), 7);
        })
    });
    c.bench_function("get_shards 7/1", |b| {
        b.iter(|| {
            let _shards = get_shards(black_box(2237), 7, 1);
        })
    });
    c.bench_function("get_shards 7/3", |b| {
        b.iter(|| {
            let _shards = get_shards(black_box(2237), 7, 3);
        })
    });
    c.bench_function("get_shards 7/7", |b| {
        b.iter(|| {
            let _shards = get_shards(black_box(2237), 7, 7);
        })
    });
}

criterion_group!(benches, benchmark);
criterion_main!(benches);