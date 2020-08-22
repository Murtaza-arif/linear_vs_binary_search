use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use rust_benchmark::fibonacci;
use rust_benchmark::linear_search;
use rust_benchmark::binary_search;

// pub fn criterion_benchmark(c: &mut Criterion) {

//     // c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
//     c.bench_function("linear search 20", |b|  b.iter(|| assert_eq!(linear_search(&v, 8000), Err(6400))));
//     c.bench_function("binary search 20", |b|  b.iter(|| assert_eq!(binary_search(&v, 8000), Err(6400))));

// }


fn bench_fibs(c: &mut Criterion) {
    let v = (0..100000000).filter(|&i| i % 5 != 0).collect::<Vec<usize>>();
    let mut group = c.benchmark_group("Search");
    for i in [20u64, 21u64].iter() {
        group.bench_with_input(BenchmarkId::new("Linear Search", i), i,
                               |b, _i| b.iter(|| assert_eq!(linear_search(&v, 8000), Err(6400))));
        group.bench_with_input(BenchmarkId::new("Binary Search", i), i,
                               |b, _i| b.iter(|| assert_eq!(linear_search(&v, 8000), Err(6400))));
    }
    group.finish();
}

criterion_group!(benches, bench_fibs);
criterion_main!(benches);
// criterion_group!(benches, criterion_benchmark);
// criterion_main!(benches);
