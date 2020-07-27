use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::hash::Hasher;
use shard_hash::{ShardHash, ShardIterator};

fn benchmark(c: &mut Criterion) {
    c.bench_function("ShardHash into_iter", |b| {
        b.iter(|| {
            let mut sh = ShardHash::new(black_box(7));
            sh.write_u64(black_box(2237));
            let _si = sh.into_iter();
        });
    });

    c.bench_function("ShardIterator new", |b| {
        b.iter(|| {
            let _si = ShardIterator::new(black_box(2237), 7, 7);
        });
    });

    c.bench_function("ShardHash 7/1", |b| {
        b.iter(|| {
            let mut sh = ShardHash::new(black_box(7));
            sh.write_u64(black_box(2237));
            let _shard = sh.into_iter().next();
        });
    });

    c.bench_function("ShardHash 7/3", |b| {
        b.iter(|| {
            let mut sh = ShardHash::new(black_box(7));
            sh.write_u64(black_box(2237));
            let mut si = sh.into_iter();

            for _ in 0..3 {
                let _shard = si.next();
            }
        });
    });

    c.bench_function("ShardHash 7/5", |b| {
        b.iter(|| {
            let mut sh = ShardHash::new(black_box(7));
            sh.write_u64(black_box(2237));
            let mut si = sh.into_iter();

            for _ in 0..5 {
                let _shard = si.next();
            }
        });
    });

    c.bench_function("ShardHash 7/7", |b| {
        b.iter(|| {
            let mut sh = ShardHash::new(black_box(7));
            sh.write_u64(black_box(2237));
            let mut si = sh.into_iter();

            for _ in 0..7 {
                let _shard = si.next();
            }
        });
    });

    c.bench_function("ShardIterator 7/1", |b| {
        b.iter(|| {
            let _shard = ShardIterator::new(black_box(2237), black_box(7), 1).next();
        })
    });

    c.bench_function("ShardIterator 7/3", |b| {
        b.iter(|| {
            let mut s = ShardIterator::new(black_box(2237), black_box(7), 3);
            for _ in 0..3 {
                let _shard = s.next();
            }
        })
    });

    c.bench_function("ShardIterator 7/5", |b| {
        b.iter(|| {
            let mut s = ShardIterator::new(black_box(2237), black_box(7), 5);
            for _ in 0..5 {
                let _shard = s.next();
            }
        })
    });

    c.bench_function("ShardIterator 7/7", |b| {
        b.iter(|| {
            let mut s = ShardIterator::new(black_box(2237), black_box(7), 7);
            for _ in 0..7 {
                let _shard = s.next();
            }
        })
    });
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
