# rust-examples
Repository of examples to help with learning Rust.

The Rust Lang Book https://doc.rust-lang.org/stable/book/title-page.html.

The YouTube series https://www.youtube.com/playlist?list=PLai5B987bZ9CoVR-QEIN9foz4QCJ0H2Y8 which goes over every chapter of the Rust book.

## Install Rust

`curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh`

## Rust compiler

Compile a file using `rustc`

```
cd hello_world
rusc main.rs
./main
```

## Cargo

Compile a file using Rust build tool, Cargo.

```
cd hello_cargo
cargo check
cargo build
cargo run
```