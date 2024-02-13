# DToken — Design Tokens for Rust

A utility to generate Rust types based on your Design Tokens.

`dtoken` is included in your project as a `build.rs` step, which takes one or
more design token JSON files (based on the W3C spec), and generates a Rust
file with all values.

It is intended to help project maintainers use native Rust types to reference
their design tokens through their editor's LSP integration, to quickly iterate
on the design of a project.
