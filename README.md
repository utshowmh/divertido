# DIVERTIDO

## Description

Divertido is a dynamically typed interpreted prorgaming language. In fronend, it has a [Lexical Analyzer](https://en.wikipedia.org/wiki/Lexical_analysis) and a [Recursive Descent Parser](https://en.wikipedia.org/wiki/Recursive_descent_parser) and in backend it has a Tree Walk Interpreter.

---

## Grammer

```
statement           ->  let 
                        | assignmet 
                        | print 
                        | expression

let                 ->  'let' identifier '=' object

assignment          ->  identifier '=' object

print               ->  'print' expression

expression          ->  binary_expression 
                        | unary_expression 
                        | literal_expression 
                        | grouping_expression 
                        | variable_expression 
                        ';'

binary_expression   ->  expression 
                        ('+' 
                        | '- ' 
                        | '*' 
                        | '/' 
                        | '!' 
                        | '!=' 
                        | '=' 
                        | '==' 
                        | '>' 
                        | '>=' 
                        | '<' 
                        | '<=') 
                        expression

unary_expression    ->  ('-' | '!') expression

literal_expression  ->  object

grouping_expression ->  '(' expression ')'

variable_expression ->  identifier


identifier          ->  (_|[a-z]|[A-Z])

object              ->  number | string | boolean | nil

number              -> ([0-9])

string              -> '"' (_|[a-z]|[A-Z]|[0-9]) '"'

boolean             -> 'true' | 'false'

nil                 -> 'nil'
```

---

## Building Divertido

In order to build this project, you need to have [git](https://git-scm.com/downloads) and [rust](https://www.rust-lang.org/tools/install) installed in your system. Then you'll be able to [clone this repo](https://docs.github.com/en/repositories/creating-and-managing-repositories/cloning-a-repository). After cloning, you'll have to [cd](https://en.wikipedia.org/wiki/Cd_(command)) to 'divertido' and [build the project with cargo](https://doc.rust-lang.org/cargo/commands/cargo-build.html). If you want some strait forward commands, here's that:

```
sudo apt install git -y
sudo apt install rust -y
git clone https://github.com/utshowmh/divertido.git
cd divertido
cargo build --release
```

After building the project, you'll find a [binary](https://en.wikipedia.org/wiki/Executable) named 'divertido' in './target/release'.
