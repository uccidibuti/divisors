#[macro_use]
extern crate criterion;
use criterion::{black_box, Criterion};

extern crate divisors;

fn criterion_benchmark(c: &mut Criterion) {
    let n0: u64 = 12345678956u64;
    let n1: u64 = 93291493211u64;
    let n2: u64 = 11111111111u64;

    c.bench_function(
        &format!("finding divisors of {} with get_divisors", n0),
        |b| b.iter(|| divisors::get_divisors(black_box(12345678956u64))),
    );
    c.bench_function(
        &format!("finding divisors of {} with get_divisors_standard", n0),
        |b| b.iter(|| get_divisors_standard(black_box(12345678956u64))),
    );

    c.bench_function(
        &format!("finding divisors of {} with get_divisors", n1),
        |b| b.iter(|| divisors::get_divisors(black_box(193291493211u64))),
    );
    c.bench_function(
        &format!("finding divisors of {} with get_divisors_standard", n1),
        |b| b.iter(|| get_divisors_standard(black_box(93291493211u64))),
    );

    c.bench_function(
        &format!("finding divisors of {} with get_divisors", n2),
        |b| b.iter(|| divisors::get_divisors(black_box(11111111111u64))),
    );
    c.bench_function(
        &format!("finding divisors of {} with get_divisors_standard", n2),
        |b| b.iter(|| get_divisors_standard(black_box(11111111111u64))),
    );
}

/// geeksforgeeks solution: https://www.geeksforgeeks.org/find-divisors-natural-number-set-1/
fn get_divisors_standard(n: u64) -> Vec<u64> {
    let mut v = Vec::new();
    let n_sqrt = (n as f64).sqrt() as u64 + 1;

    for i in 2..n_sqrt {
        if n % i == 0 {
            if n / i == i {
                v.push(i);
            } else {
                v.push(i);
                v.push(n / i);
            }
        }
    }
    v.sort();
    v
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
