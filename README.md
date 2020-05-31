# Ops

Rust implementation of [operational-endpoints-spec](https://github.com/utilitywarehouse/operational-endpoints-spec) making it easy to add the standard endpoints to your application.

## Usage

``` rust
use ops::{StatusBuilder, server};

#[tokio::main]
async fn main() {
    let status = StatusBuilder::always("my app", "a description");

    let server = server("0.0.0.0:3000".parse().unwrap(), status);

    server.await.unwrap();
}
```

## Examples

See the [examples](/examples) folder for runnable examples.

## Documentation

``` shell
cargo doc --all --no-deps
open target/doc/ops/index.html
```

## License

Licensed under either of:

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
