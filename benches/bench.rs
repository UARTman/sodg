#![feature(custom_test_frameworks)]
#![feature(once_cell)]
#![test_runner(criterion::runner)]

use criterion::{BatchSize, BenchmarkId, Criterion};
use criterion_macro::criterion;
use rand::rngs::OsRng;
use rand::Rng;
use sodg::Sodg;
use std::sync::Arc;
use std::sync::Mutex;

#[criterion]
fn bench_sodg_add(c: &mut Criterion) {
    let mut group = c.benchmark_group("Sodg::add (random)");

    for already_present in [0, 1, 10, 100, 1000, 10000] {
        group.bench_with_input(
            BenchmarkId::from_parameter(already_present),
            &already_present,
            |b, &already_present| {
                let os_rng = Arc::new(Mutex::new(OsRng::default()));

                b.iter_batched(
                    || {
                        let mut sodg = Sodg::empty();
                        let mut rng = os_rng.lock().unwrap();
                        for _ in 0..already_present {
                            let _ = sodg.add(rng.gen());
                        }
                        let id = loop {
                            let id = rng.gen();
                            if sodg.kids(id).is_err() {
                                break id;
                            }
                        };
                        (id, sodg)
                    },
                    |(id, mut sodg)| {
                        sodg.add(id).unwrap();
                    },
                    BatchSize::SmallInput,
                );
            },
        );
    }

    group.finish();

    let mut group = c.benchmark_group("Sodg::add (sequential)");

    for already_present in [0, 1, 10, 100, 1000, 10000] {
        group.bench_with_input(
            BenchmarkId::from_parameter(already_present),
            &already_present,
            |b, &already_present| {
                b.iter_batched(
                    || {
                        let mut sodg = Sodg::empty();
                        for i in 0..already_present {
                            let _ = sodg.add(i);
                        }
                        (already_present, sodg)
                    },
                    |(id, mut sodg)| {
                        sodg.add(id).unwrap();
                    },
                    BatchSize::SmallInput,
                );
            },
        );
    }

    group.finish();
}
