# `s-macro`

A basic Rust library for conveniently making a `String`. Like so:

```rust
use s_macro::s;

assert!(s!()                   == String::new());
assert!(s!("hello, world")     == String::from("hello, world"));
assert!(s!(123 + 321)          == format!("{}", 123 + 321));

let world = "world";
assert!(s!("hello, {}", world) == format!("hello, {}", world));
assert!(s!("hello, {world}")   == format!("hello, {world}"));
```
