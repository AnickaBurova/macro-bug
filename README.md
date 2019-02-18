# Show a potential bug in macros in rust lang

Files [macro.rs](https://github.com/AnickaBurova/macro-bug/blob/master/macro.rs) and [manual.rs](https://github.com/AnickaBurova/macro-bug/blob/master/manual.rs) are nearly identical, tho the macro.rs is failing with two errors.

The difference is the code which is failing to compile is generated using a macro.
``` bash
>>diff macro.rs manual.rs
178c178
<     #[cfg(feature = "manual")]
---
>     #[cfg(not(feature = "manual"))]

```

Those two files were generated using [cargo-expand](https://github.com/dtolnay/cargo-expand)

