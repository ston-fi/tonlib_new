use auto_pool::pool::AutoPool;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::sync::Arc;

struct DummyObject {
    id: usize,
    value: usize,
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

fn perf_auto_pool(pool: Arc<AutoPool<DummyObject>>) {
    run_tests(pool, |pool| {
        let obj = pool.get().unwrap();
        let _id = &obj.id;
        let _val = &obj.value;
    });
}

fn perf_lockfree_pool(pool: Arc<lockfree_object_pool::LinearObjectPool<DummyObject>>) {
    run_tests(pool, |pool| {
        let obj = pool.pull();
        let _id = &obj.id;
        let _val = &obj.value;
    });
}

fn perf_object_pool(pool: Arc<object_pool::Pool<DummyObject>>) {
    run_tests(pool, |pool| {
        let obj = pool.pull(|| DummyObject { id: 0, value: 1 });
        let _id = &obj.id;
        let _val = &obj.value;
    });
}

fn benchmark_functions(c: &mut Criterion) {
    let auto_pool = Arc::new(AutoPool::new((0..POOL_SIZE).map(|id| DummyObject { id, value: 1 })));
    c.bench_function("auto_pool", |b| b.iter(|| perf_auto_pool(black_box(auto_pool.clone()))));

    // Nice interface...
    let lockfree_pool = {
        let pool = lockfree_object_pool::LinearObjectPool::new(|| DummyObject { id: 0, value: 1 }, |_| {});
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

    let object_pool = object_pool::Pool::new(POOL_SIZE, || DummyObject { id: 0, value: 1 });
    let object_pool = Arc::new(object_pool);
    c.bench_function("object_pool", |b| b.iter(|| perf_object_pool(black_box(object_pool.clone()))));
}

criterion_group!(benches, benchmark_functions);
criterion_main!(benches);
