## Examples code of `<<Rust cookbook>>`

- Algorithm
- CommandLine
- Compress
- Concurrency
- Cryptography
- DataStructre
- Database
- Datetime
- Encoding
- Logging
- Web Programming

## How to run Examples

1. First change directory to examples directory of any crate, see the `mod name`(file name w/o `.rs` extension name).
2. Change to root directory and execute `cargo run --example [name]`, replace `[name]` with `mod name`.

For example:

Compute a digest of contents of a text file
```
cargo run --example hashing
   Compiling cryptography v0.1.0 (/Users/xxx/2022/languages/rust/rust-cookbook-code/cryptography)
    Finished dev [unoptimized + debuginfo] target(s) in 1.92s
     Running `target/debug/examples/hashing`
   File: timg.txt
SHA-256: 81700022B5CAB8EFC79F276B69D17251B03FFCDAB61C026B75F783B55E3953CB
```