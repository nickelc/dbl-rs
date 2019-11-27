# dbl-rs
[![crates.io][crates-badge]][crates-url]
[![docs.rs][docs-badge]][docs-url]
![License][license-badge]
[![build status][actions-badge]][actions-url]

[crates-badge]: https://img.shields.io/crates/v/dbl-rs.svg
[crates-url]: https://crates.io/crates/dbl-rs
[docs-badge]: https://docs.rs/dbl-rs/badge.svg
[docs-url]: https://docs.rs/dbl-rs
[actions-badge]: https://github.com/nickelc/dbl-rs/workflows/ci/badge.svg
[actions-url]: https://github.com/nickelc/dbl-rs/actions
[license-badge]: https://img.shields.io/crates/l/dbl-rs.svg

Rust bindings for the [discordbots.org](https://discordbots.org) API.

## Usage

Add this to your `Cargo.toml`
```toml
[dependencies]
dbl-rs = "0.1"
```

## Example

```rust
use dbl::types::ShardStats;
use dbl::Client;
use tokio::runtime::Runtime;

#[tokio::main]
async fn main() {
    let token = match std::env::var("DBL_TOKEN") {
        Ok(token) => token,
        _ => panic!("missing token"),
    };

    let client = Client::new(token).expect("failed client");

    let bot = 565_030_624_499_466_240;
    let stats = ShardStats::Cumulative {
        server_count: 1234,
        shard_count: None,
    };

    match client.update_stats(bot, stats).await {
        Ok(_) => println!("Update successful"),
        Err(e) => eprintln!("{}", e),
    }
}
```

### Examples

See [examples directory](examples/) for some getting started examples.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you,
as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
