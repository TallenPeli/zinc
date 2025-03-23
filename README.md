# ZINC

[![GitHub](https://img.shields.io/github/license/Ananto30/cap-em)](/LICENSE)

ZINC is a programming language written in Rust that aims to provide memory safety and many other safety features without being in your face like Rust is.

ZINC uses LLVM for code compilation so it should work on Linux, MacOS, Windos, BSD, etc.


## Philosophy

I created ZINC as a middle ground between C and Rust. C is a very simple language and is really easy to read and write, but it's really easy to write unsafe or bad code. Rust is more complex, but provides a ton a safety features.

C comes with no tooling, and there is no centralized way to do things. Rust, on the other hand provides a really useful and powerful tools for building, docs, package management and so much more.

So, with ZINC I'm attempting to be a middle ground. A simple language with a ton of safety features and strong tooling.

## Dependencies

All you need to compile ZINC is `cargo` which can be installed via [rustup](https://rustup.rs/)

## Installing

#### Any OS

Clone and cd into the repository
```
git clone https://github.com/TallenPeli/zinc && cd zinc
```

Build and install
```
cargo build --release && cargo install --path .
```

## Documentation
[Read the docs](/docs/index.md)

## Q&A

Q? Is there an LSP?
A: Not yet, but one is planned.

Q? Can I use ZINC in production?
A: ZINC is still in early development, so I do not recommend using it in production.

Q? How can I contribute?
A: Clone the repository, make changes, and open a new PR. Any help is great!

Q? Do you take donations?
A: No, we do not take donations and I don't plan to take any soon. This is a fun hobby project, but this may change.


