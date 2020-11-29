# Panic

When a panic occurs rust unwinds, meaning walk backs to the stack and cleans up data from each 
function it encounters or you can rely on the OS to do that, but for that we need to add one 
parameter to the `[profile]` section in `cargo.toml`

```toml
[profile.release]
panic = 'abort'
```

## Forcing panic
If you want to force a panic in your code use
```rust
fn main() {
    panic!("force panic");
}
```

## Getting stacktrace on panic
`RUST_BACKTRACE=1` environment variable needs to be set