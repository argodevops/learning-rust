## Hello Rust

💡 Rust files should have `.rs` file extension and if you’re using more than one word for the file name, follow the [snake_case](https://en.wikipedia.org/wiki/Snake_case) convention.

Compile Rust file directly using `rustc`.
```
rustc main.rs
./main
```

## Hello Cargo

Cargo is Rust’s built-in package manager and the build system. It can be used to,

- Create a new project: `cargo new`
- Create a new project in an existing directory: `cargo init`
- Build the project: `cargo build`
- Run the project: `cargo run`
- Update project dependencies: `cargo update`
- Run tests: `cargo test`
- Run benchmarks: `cargo bench`
- Generate the project documentation via [rustdoc](https://doc.rust-lang.org/stable/rustdoc/): `cargo doc`
- Analyze the project to see it has any errors, without building it: `cargo check`

```
cargo new hello_cargo
cargo build
cargo run
```
