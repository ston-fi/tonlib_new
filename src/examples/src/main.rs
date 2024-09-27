use anyhow::bail;
use autoreturn_pool::{Config, Pool};
use std::sync::Arc;

fn main() -> anyhow::Result<()> {
    single_thread()?;
    multi_thread()?;
    add_release()?;
    Ok(())
}

#[derive(Debug)]
struct MyObject {
    value: i32,
}

impl MyObject {
    fn print_me(&self, tag: &str) {
        println!("[{tag}]: MyObject: val={}", self.value)
    }
}

fn multi_thread() -> anyhow::Result<()> {
    let objects = [MyObject { value: 1 }, MyObject { value: 2 }];
    let pool = Arc::new(Pool::new(objects));

    let pool_th = pool.clone();
    let join_handler = std::thread::spawn(move || {
        pool_th.take().unwrap().print_me("thread_obj1");
        pool_th.take().unwrap().print_me("thread_obj2");
        Ok(())
    });

    let main_th_obj = pool.take().unwrap();
    main_th_obj.print_me("main_th_obj");
    drop(main_th_obj);

    match join_handler.join() {
        Ok(Ok(())) => {}
        Ok(Err(err)) => return Err(err),
        Err(err) => bail!("Thread panicked: {:?}", err),
    }
    Ok(())
}

fn single_thread() -> anyhow::Result<()> {
    let objects = [MyObject { value: 1 }, MyObject { value: 2 }];

    let config = Config {
        wait_duration: std::time::Duration::from_millis(5),
    };

    let pool = Pool::with_config(config, objects);

    let obj1_val = {
        let obj1 = pool.take().unwrap();
        obj1.print_me("obj1");
        obj1.value
    };
    let obj2 = pool.take().unwrap();
    obj2.print_me("obj2");

    assert_eq!(obj1_val, obj2.value);

    let obj3 = pool.take().unwrap();
    obj3.print_me("obj3");
    assert_ne!(obj2.value, obj3.value);

    let obj4 = pool.take();
    assert!(obj4.is_none());
    Ok(())
}

fn add_release() -> anyhow::Result<()> {
    let objects = [MyObject { value: 1 }];
    let pool = Pool::new(objects);

    {
        let obj1 = pool.take().unwrap();
        obj1.print_me("obj1");
        obj1.release();
    }
    // no objects in pool despite of obj1 drop
    assert_eq!(pool.size(), 0);

    pool.add(MyObject { value: 2 });
    assert_eq!(pool.size(), 1);
    pool.take().unwrap().print_me("obj2");

    Ok(())
}
