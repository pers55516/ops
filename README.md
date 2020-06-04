# Ops

[![Build Status]][actions] [![Latest Version]][crates.io] [![Latest Docs]][docs.rs]

[Build Status]: https://img.shields.io/github/workflow/status/utilitywarehouse/rust-ops/Rust/master?style=flat-square
[actions]: https://github.com/utilitywarehouse/rust-ops/actions?query=branch%3Amaster
[Latest Version]: https://img.shields.io/crates/v/ops.svg?style=flat-square
[crates.io]: https://crates.io/crates/ops
[Latest Docs]: https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square
[docs.rs]: https://docs.rs/ops

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

## License

Licensed under either of:

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
