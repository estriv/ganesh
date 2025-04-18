use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use ganesh::core::{CtrlCAbortSignal, Minimizer};
use ganesh::solvers::gradient_free::NelderMead;
use ganesh::test_functions::rosenbrock::Rosenbrock;

fn nelder_mead_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Nelder Mead");
    for n in [2, 3, 4, 5] {
        group.bench_with_input(BenchmarkId::new("Rosenbrock", n), &n, |b, ndim| {
            let problem = Rosenbrock { n: *ndim };
            let mut m = Minimizer::new(NelderMead::default()).setup(|m| {
                m.with_abort_signal(CtrlCAbortSignal::new())
                    .with_max_steps(10_000_000)
                    .on_status(|s| s.with_x0(vec![5.0; *ndim]))
            });
            b.iter(|| {
                m.minimize(&problem).unwrap();
                black_box(&m.status);
            });
        });
        group.bench_with_input(
            BenchmarkId::new("Rosenbrock (adaptive)", n),
            &n,
            |b, ndim| {
                let problem = Rosenbrock { n: *ndim };
                let mut m = Minimizer::new(NelderMead::default().with_adaptive(n)).setup(|m| {
                    m.with_abort_signal(CtrlCAbortSignal::new())
                        .with_max_steps(10_000_000)
                        .on_status(|s| s.with_x0(vec![5.0; *ndim]))
                });
                b.iter(|| {
                    m.minimize(&problem).unwrap();
                    black_box(&m.status);
                });
            },
        );
    }
    group.finish();
}

criterion_group!(benches, nelder_mead_benchmark);
criterion_main!(benches);
