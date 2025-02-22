use crate::config::{AutoPoolConfig, PickStrategy};
use crate::pool::AutoPool;
use std::collections::HashMap;
use std::ops::Deref;

#[test]
fn test_create() {
    let pool = AutoPool::new([1, 2, 3]);
    assert_eq!(pool.size(), 3);
}

#[test]
fn test_take() {
    let pool = AutoPool::new([1, 2, 3]);
    let obj1 = pool.get();
    assert_eq!(pool.size(), 2);
    assert_eq!(*obj1.as_ref().unwrap().deref(), 3);
}

#[tokio::test]
#[cfg(feature = "async")]
async fn test_take_async() {
    let pool = AutoPool::new([1, 2, 3]);
    let obj1 = pool.get_async().await;
    assert_eq!(pool.size(), 2);
    assert_eq!(*obj1.as_ref().unwrap().deref(), 3);
}

#[test]
fn test_add() {
    let pool = AutoPool::new([1]);
    pool.add(2);
    assert_eq!(pool.size(), 2);
}

#[test]
fn test_wait() {
    let wait_time = std::time::Duration::from_millis(20);
    let config = AutoPoolConfig {
        wait_duration: wait_time,
        ..Default::default()
    };
    let pool = AutoPool::new_with_config(config, [1]);
    let _obj1 = pool.get();
    assert_eq!(pool.size(), 0);
    let start_time = std::time::Instant::now();
    let obj2 = pool.get();
    assert!(start_time.elapsed() >= wait_time);
    assert!(obj2.is_none());
}

#[test]
fn test_workflow() {
    let config = AutoPoolConfig {
        wait_duration: std::time::Duration::from_millis(5),
        ..Default::default()
    };
    let pool = AutoPool::new_with_config(config, [1, 2, 3]);
    assert_eq!(pool.size(), 3);

    let obj1 = pool.get();
    assert_eq!(pool.size(), 2);
    assert_eq!(*obj1.as_ref().unwrap().deref(), 3);

    let obj2 = pool.get();
    assert_eq!(*obj2.as_ref().unwrap().deref(), 2);
    let obj3 = pool.get();
    assert_eq!(pool.size(), 0);
    assert_eq!(*obj3.as_ref().unwrap().deref(), 1);

    let obj4 = pool.get();
    assert!(obj4.is_none());
}

#[test]
fn test_pick_strategy_lifo() {
    let config = AutoPoolConfig {
        wait_duration: std::time::Duration::from_millis(5),
        pick_strategy: PickStrategy::LIFO,
        ..Default::default()
    };
    let pool = AutoPool::new_with_config(config, [1, 2, 3]);
    for _ in 0..1000 {
        let obj1 = pool.get();
        assert_eq!(*obj1.as_ref().unwrap().deref(), 3);
    }
}

#[test]
fn test_pick_strategy_random() {
    let config = AutoPoolConfig {
        wait_duration: std::time::Duration::from_millis(5),
        pick_strategy: PickStrategy::RANDOM,
        ..Default::default()
    };
    let pool = AutoPool::new_with_config(config, [1, 2, 3]);
    let mut match_counter = HashMap::new();
    for _ in 0..3000 {
        let obj1 = pool.get();
        let value = *obj1.unwrap();
        match_counter.entry(value).and_modify(|v| *v += 1).or_insert(1);
    }

    // not guaranteed to pass, but should be close
    assert!(match_counter[&1] > 200);
    assert!(match_counter[&2] > 200);
    assert!(match_counter[&3] > 200);
}
