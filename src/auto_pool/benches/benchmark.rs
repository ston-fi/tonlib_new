use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::sync::Arc;

struct DummyObject {
    id: usize,
    text: String,
}

const POOL_SIZE: usize = 1000;
const OPERATIONS_PER_THREAD: usize = 100;
const THREADS_COUNT: usize = 64;

fn run_tests<P: Send + Sync + 'static>(arc_pool: Arc<P>, pool_op: fn(&P)) {
    let threads: Vec<_> = (0..THREADS_COUNT)
        .map(|_| {
            let pool = arc_pool.clone();
            std::thread::spawn(move || {
                for _ in 0..OPERATIONS_PER_THREAD {
                    pool_op(&pool);
                }
            })
        })
        .collect();
    for thread in threads {
        thread.join().unwrap();
    }
}

fn perf_autoreturn_pool(pool: Arc<autoreturn_pool::Pool<DummyObject>>) {
    run_tests(pool, |pool| {
        let obj = pool.take().unwrap();
        let _id = &obj.id;
        let _text = &obj.text;
    });
}

fn perf_lockfree_pool(pool: Arc<lockfree_object_pool::LinearObjectPool<DummyObject>>) {
    run_tests(pool, |pool| {
        let obj = pool.pull();
        let _id = &obj.id;
        let _text = &obj.text;
    });
}

fn perf_object_pool(pool: Arc<object_pool::Pool<DummyObject>>) {
    run_tests(pool, |pool| {
        let obj = pool.pull(|| DummyObject {
            id: 0,
            text: "text".to_string(),
        });
        let _id = &obj.id;
        let _text = &obj.text;
    });
}

fn benchmark_functions(c: &mut Criterion) {
    let autoreturn_pool = Arc::new(autoreturn_pool::Pool::new((0..POOL_SIZE).map(|id| DummyObject {
        id,
        text: "text".to_string(),
    })));
    c.bench_function("autoreturn_pool", |b| b.iter(|| perf_autoreturn_pool(black_box(autoreturn_pool.clone()))));

    // Nice interface...
    let lockfree_pool = {
        let pool = lockfree_object_pool::LinearObjectPool::new(
            || DummyObject {
                id: 0,
                text: "text".to_string(),
            },
            |_| {},
        );
        {
            let mut items = Vec::with_capacity(POOL_SIZE);
            for _ in 0..POOL_SIZE {
                items.push(pool.pull());
            }
        }
        pool
    };
    let lockfree_pool = Arc::new(lockfree_pool);
    c.bench_function("lockfree_pool", |b| b.iter(|| perf_lockfree_pool(black_box(lockfree_pool.clone()))));

    let object_pool = object_pool::Pool::new(POOL_SIZE, || DummyObject {
        id: 0,
        text: "text".to_string(),
    });
    let object_pool = Arc::new(object_pool);
    c.bench_function("object_pool", |b| b.iter(|| perf_object_pool(black_box(object_pool.clone()))));
}

criterion_group!(benches, benchmark_functions);
criterion_main!(benches);
