# learning-rust

Repository of examples to help with learning Rust.

The Rust Lang Book https://doc.rust-lang.org/stable/book/title-page.html.

The YouTube series https://www.youtube.com/playlist?list=PLai5B987bZ9CoVR-QEIN9foz4QCJ0H2Y8 which goes over every chapter of the Rust book.

## Rust Goals

The goal of Rust is to be a good programming language for creating highly concurrent, safe and performant systems.

> **"Rust is a systems programming language focused on three goals: safety, speed, and concurrency."**
> \_\_ Rust Documentation

Rust is very young and very modern language. It is a **[compiled programming language](https://en.wikipedia.org/wiki/Compiled_language)** and it uses [LLVM](https://en.wikipedia.org/wiki/LLVM) on the backend. Also, Rust is a **[multi-paradigm programming language](https://en.wikipedia.org/wiki/Comparison_of_multi-paradigm_programming_languages)**, which supports imperative procedural, concurrent actor, object-oriented and pure functional styles. It also supports generic programming and metaprogramming, in both static and dynamic styles.

> 🔎 One of Rust’s most unique and compelling features is [Ownership](/docs/ownership), which is used to achieve memory safety. Rust creates memory pointers optimistically, checks memory pointers’ limited accesses at compile-time with the usage of [References and Borrowing](/docs/borrowing). And it does automatic compile-time memory management by checking the [Lifetimes](/docs/lifetimes).

## Influences
Its design elements came from a wide range of sources.

- Abstract Machine Model: **C**
- Data types: **C, SML, OCaml, Lisp, Limbo**
- Optional Bindings: **Swift**
- Functional Programming: **Haskell, OCaml, F\#**
- Attributes: **ECMA**-335
- Memory Model and Memory Management: **C++, ML Kit, Cyclone**
- Type Classes: **Haskell**

Rust **doesn't use an automated garbage collection** system\(GC\) by default.

Rust compiler observes the code **at compile-time** and helps to [**prevent many types of errors**](https://doc.rust-lang.org/error-index.html) that are possible to write in C, C++ like programming languages.

## Installing Rust

`curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh`

## Sections

* [Hello Cargo](content/README.md)

