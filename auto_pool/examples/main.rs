use anyhow::bail;
use auto_pool::config::AutoPoolConfig;
use auto_pool::pool::AutoPool;
use std::sync::Arc;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    single_thread()?;
    multi_thread()?;
    add_release()?;
    add_release_async().await?;
    Ok(())
}

#[derive(Debug)]
struct MyObject {
    value: i32,
}

impl MyObject {
    fn print_me(&self, tag: &str) { println!("[{tag}]: MyObject: val={}", self.value) }
}

fn multi_thread() -> anyhow::Result<()> {
    let objects = [MyObject { value: 1 }, MyObject { value: 2 }];
    let pool = Arc::new(AutoPool::new(objects));

    let pool_th = pool.clone();
    let join_handler = std::thread::spawn(move || {
        pool_th.get().unwrap().print_me("thread_obj1");
        pool_th.get().unwrap().print_me("thread_obj2");
        Ok(())
    });

    let main_th_obj = pool.get().unwrap();
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

    let config = AutoPoolConfig {
        wait_duration: std::time::Duration::from_millis(5),
        ..Default::default()
    };

    let pool = AutoPool::new_with_config(config, objects);

    let obj1_val = {
        let obj1 = pool.get().unwrap();
        obj1.print_me("obj1");
        obj1.value
    };
    let obj2 = pool.get().unwrap();
    obj2.print_me("obj2");

    assert_eq!(obj1_val, obj2.value);

    let obj3 = pool.get().unwrap();
    obj3.print_me("obj3");
    assert_ne!(obj2.value, obj3.value);

    let obj4 = pool.get();
    assert!(obj4.is_none());
    Ok(())
}

fn add_release() -> anyhow::Result<()> {
    let objects = [MyObject { value: 1 }];
    let pool = AutoPool::new(objects);

    {
        let obj1 = pool.get().unwrap();
        obj1.print_me("obj1");
        obj1.release();
    }
    // no objects in pool despite of obj1 drop
    assert_eq!(pool.size(), 0);

    pool.add(MyObject { value: 2 });
    assert_eq!(pool.size(), 1);
    pool.get().unwrap().print_me("obj2");

    Ok(())
}

async fn add_release_async() -> anyhow::Result<()> {
    let objects = [MyObject { value: 1 }];
    let pool = AutoPool::new(objects);

    {
        let obj1 = pool.get_async().await.unwrap();
        obj1.print_me("obj1");
        obj1.release();
    }
    // no objects in pool despite of obj1 drop
    assert_eq!(pool.size(), 0);

    pool.add(MyObject { value: 2 });
    assert_eq!(pool.size(), 1);
    pool.get_async().await.unwrap().print_me("obj2");

    Ok(())
}
