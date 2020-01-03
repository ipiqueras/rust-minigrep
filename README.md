# Learning Rust: minigrep

Implement [minigrep](https://doc.rust-lang.org/book/ch12-00-an-io-project.html) 
and slowly add some functionality: logging, better argument passing, integration
tests and documentation.

This is just a small private project to learn Rust.

Logging can be enabled by setting env variable `RUST_LOG` and is provided by
[env_logger](https://docs.rs/env_logger/0.7.1/env_logger/) crate.

```
RUST_LOG=info cargo run How poem.txt
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/minigrep How poen.txt`
[2020-01-03T14:42:22Z INFO  minigrep] searching: How
[2020-01-03T14:42:22Z INFO  minigrep] in: poen.txt
[2020-01-03T14:42:22Z INFO  minigrep] Performing case sensitive search
How dreary to be somebody!
How public, like a frog
```

## Links and docs

See https://www.rust-lang.org/what/cli and the [CLI book](https://rust-cli.github.io/book/index.html)
from where I took the implementation of the integration tests.

Also see [Rust by Example](https://doc.rust-lang.org/rust-by-example/testing/integration_testing.html)
