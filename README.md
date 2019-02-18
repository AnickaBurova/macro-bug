# Show a potential bug in macros in rust lang

Files macro.rs and manual.rs are nearly identical, tho the macro.rs is failing with two errors.

The difference is, the code which is failing to compile is generated using a macro.

Those two files were generated using [cargo-expand](https://github.com/dtolnay/cargo-expand)
