<div align="center">
    <h1>fmtex</h1>
    <p>
        Extra format adaptors
    </p>
    <p>
        <a href="https://crates.io/crates/fmtex"><img src="https://img.shields.io/crates/v/fmtex.svg"></img></a>
        <a href="https://docs.rs/fmtex"><img src="https://docs.rs/fmtex/badge.svg"></img></a>
        <a href="https://github.com/nanoqsh/fmtex/actions"><img src="https://github.com/nanoqsh/fmtex/workflows/ci/badge.svg"></img></a>
    </p>
</div>

## How to use
Add the dependency to your project with:
```sh
cargo add fmtex
```

Use library traits to format something:
```rust
use fmtex::prelude::*;

let s = [1, 2, 3].joined(", ").to_string();
assert_eq!(s, "1, 2, 3");
```
