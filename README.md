# ZINC

[![GitHub](https://img.shields.io/github/license/Ananto30/cap-em)](/LICENSE)

!["Zinc's Sally](/assets/sally_banner.png)
Artist: [@htsu.jj](https://www.instagram.com/htsu.jj/)

ZINC is a programming language (re)written in Rust that aims to provide memory safety and many other safety features without being in your face about it like Rust is.

ZINC uses LLVM for code compilation so it should work on Linux, MacOS, Windos, BSD, etc.

## Philosophy

I created ZINC as a combination of all the great features from C, Rust, JavaScript, and C#.

C is a very simple language and is really easy to read and write, but it's really easy to write unsafe or bad code. Rust is more complex, but provides a ton a safety features.

C comes with no tooling, and there is no centralized way to do things. Rust, on the other hand provides a really useful and powerful tools for building, docs, package management and so much more.

I find that JavaScript has really nice syntax and it feels _fun_ to write, unlike many other languages. (rust)

My first _real_ programming language that I learned (not including Python or Scratch) was C#, and I really enjoyed the syntax and the object oriented nature of it. I wrote C# for Unity and everything was super easy to implement, development time was fast, and the language felt organized.

I want to copy this aspect of C#. I really want Zinc to feel organized and I want it to be fun to write.

Zinc aims to fix many of the issues I have with Rust. Although you may call it a skill issue (maybe it is), I feel that my critisicm of the language is justified. Rust holds your hand too much and doesn't allow for the programmer to do what they really want to. Of course, you can tell the compiler to ignore your "unsafe" code, but this feels like a band-aid solution. Rust's bindings implementation has (some) overhead and I want the programmers to be able to directly use C libraries without having much of a performance hit.

So, with ZINC I'm attempting to be a middle ground. A simple and fun language with a ton of safety features and strong tooling.

## Dependencies

All you need to compile ZINC is `cargo` which can be installed via [rustup](https://rustup.rs/)

## Installing

Clone and `cd` into the repository

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

Q: Is there an LSP?

> A: Not yet, but one is planned.

Q: Can I use ZINC in production?

> A: ZINC is still in early development, so I do not recommend using it in production.

Q: How can I contribute?

> A: Clone the repository, make changes, and open a new PR. Any help is great!

Q: Do you take donations?

> A: No, I do not take donations and I don't plan to take any soon. This is a fun hobby project, but this may change.
