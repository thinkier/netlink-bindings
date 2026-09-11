## strip-async

Strip `async` and `.await` tokens with zero transitive dependencies.

Take a look at [netlink-socket2] crate to see how this can be *ab*used to reuse
code between sync and async implementations, allowing implementations for
multiple runtimes to
[coexist](https://doc.rust-lang.org/nightly/cargo/reference/features.html#feature-unification)
while being individually toggleable.

[netlink-socket2]: https://github.com/one-d-wide/netlink-bindings

```rust
#[cfg(not(feature = "async"))]
use std::fs;
#[cfg(feature = "tokio")]
use tokio::fs;

use strip_async::strip_async;

#[strip_async]
async fn async_readme() -> String {
    fs::read_to_string("README.md").await.unwrap()
}
// ...will be transformed into... 
fn std_readme() -> String {
    fs::read_to_string("README.md").unwrap()
}

assert_eq!(async_readme(), std_readme());
```

There are also some trivial helpers that may be useful:

```rust
use strip_async::{keep_if, skip_if, skip, keep};

// Only this one will remain
#[keep]
fn helper() {}

#[skip]
fn helper() {}

// Same as `#[cfg_attr(not(...), skip)]`
#[keep_if(feature = "async")]
fn helper() {}

#[skip_if(not(feature = "async"))]
fn helper() {}
```

## Use as a dependency

```toml
[dependencies]
strip-async = "=1.0.0"
```
