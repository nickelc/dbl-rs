# dbl-rs

Rust bindings for the [discordbots.org](https://discordbots.org) API.

## Usage

To use `dbl-rs`, add this to your `Cargo.toml`
```toml
[dependencies]
dbl-rs = { git = "https://github.com/nickelc/dbl-rs" }
```

## Example

```rust
use dbl::Client;
use tokio::runtime::Runtime;

fn main() {
    let token = match std::env::var("TOKEN") {
        Ok(token) => token,
        _ => panic!("missing token"),
    };

    let mut rt = Runtime::new().expect("failed rt");
    let client = Client::new(token);

    let task = client.get(565030624499466240);

    match rt.block_on(task) {
        Ok(bot) => println!("{:#?}", bot),
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
