use autoreturn_pool::{Config, Pool};

#[derive(Debug)]
struct MyObject {
    value: i32,
}

fn main() -> anyhow::Result<()> {
    let objects = [MyObject { value: 1 }, MyObject { value: 2 }];

    let config = Config {
        wait_duration: std::time::Duration::from_millis(5),
    };

    let pool = Pool::with_config(config, objects)?;

    let obj1_val = {
        let obj1 = pool.take()?.unwrap();
        println!("obj1: {:?}", *obj1);
        obj1.value
    };
    let obj2 = pool.take()?.unwrap();
    println!("obj2: {:?}", *obj2);

    assert_eq!(obj1_val, obj2.value);

    let obj3 = pool.take()?.unwrap();
    println!("obj3: {:?}", *obj3);
    assert_ne!(obj2.value, obj3.value);

    let obj4 = pool.take()?;
    assert!(obj4.is_none());
    Ok(())
}
