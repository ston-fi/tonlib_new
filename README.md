# autoreturn-pool

This pool automatically manages the return of objects.

The primary difference from competitors is the interface - implementation follows the RAII pattern, meaning the user must provide objects when creating the pool.

The `Pool` class has only three public methods:
- Constructor `with_config(items)`: Creates a new pool with a custom configuration.
- Constructor `new(items)`: An alias for `with_config` that uses the default configuration.
- `take(&self)`: Extracts an object from the pool.

The configuration allows you to set the wait duration for the `take` method (default is `Duration::MAX`, which essentially means "Wait until you receive it").


Examples:
```rust

fn main() -> Result<(), autoreturn_pool::Error> {
    // basic usage
    let pool = autoreturn_pool::Pool::new([1, 2])?;
    let item = pool.take()?.unwrap();

    // with custom config
    let config = autoreturn_pool::Config {
        wait_duration: std::time::Duration::from_millis(5),
    };
    let pool = autoreturn_pool::Pool::with_config(config, [1, 2])?;
    let item = pool.take()?.unwrap();
}
```

```rust
// with custom object:
#[derive(Default)]
struct MyObject {
    value: i32,
}
fn main() -> Result<(), autoreturn_pool::Error> {
    let pool_objects = [
        MyObject::default(),
        MyObject::default()
    ];
    let pool = autoreturn_pool::Pool::new(pool_objects)?;
    let mut item = pool.take()?.unwrap();
    Ok(())
}
```
