// third-party imports
use criterion::{criterion_group, criterion_main, Criterion};
use regex::Regex;

fn benchmark(c: &mut Criterion) {
    let mut c = c.benchmark_group("regex");

    let re = Regex::new(r"^_").unwrap();

    c.bench_function("b1", |b| {
        b.iter(|| {
            assert_eq!(re.is_match("_TEST"), true);
        });
    });

    c.bench_function("b2", |b| {
        b.iter(|| {
            assert_eq!(re.is_match("_TEST_SOME_VERY_VERY_LONG_NAME"), true);
        });
    });

    c.bench_function("b3", |b| {
        b.iter(|| {
            assert_eq!(re.is_match("TEST"), false);
        });
    });

    c.bench_function("b4", |b| {
        b.iter(|| {
            assert_eq!(re.is_match("TEST_SOME_VERY_VERY_LONG_NAME"), false);
        });
    });
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
