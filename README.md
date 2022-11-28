# Divertido

## Description

Divertido is a dynamically typed interpreted prorgaming language. In fronend, it has a [Lexical Analyzer](https://en.wikipedia.org/wiki/Lexical_analysis) and a [Recursive Descent Parser](https://en.wikipedia.org/wiki/Recursive_descent_parser) and in backend it has a Tree Walk Interpreter.

---

## Syntax

Divertido's syntax is similer to [Rust](https://www.rust-lang.org/)'s. See [examples](https://github.com/utshowmh/divertido/tree/main/examples) for more.

---

## Inspiration

Most of algorithms and techniques I used to implement Divertido was acquired from [Crafting Interpreter](https://www.craftinginterpreters.com/).

---

## Building Divertido

In order to build this project, you need to have [git](https://git-scm.com/downloads) and [rust](https://www.rust-lang.org/tools/install) installed in your system. Then you'll be able to [clone this repo](https://docs.github.com/en/repositories/creating-and-managing-repositories/cloning-a-repository). After cloning, you'll have to [cd](https://en.wikipedia.org/wiki/Cd_(command)) to 'divertido' and [build the project with cargo](https://doc.rust-lang.org/cargo/commands/cargo-build.html). Or, if you want some strait forward commands, you can run these:

```
sudo apt install git -y
sudo apt install rust -y
git clone https://github.com/utshowmh/divertido.git
cd divertido
cargo build --release
```

After building the project, you'll find a [binary](https://en.wikipedia.org/wiki/Executable) named 'divertido' in './target/release'.
