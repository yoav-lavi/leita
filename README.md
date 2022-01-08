# Leita

A search query language that can transpile into specific search provider syntax (currently Google only)

A personal project to explore Rust and lexers

## Name

Leita means "to seek" or "to search" in Old Norse and many descendant languages

## Tokens

| **Concept**   | **Leita**                            | **Google**                             |
| ------------- | ------------------------------------ | -------------------------------------- |
| exact         | "term"                               | "term"                                 |
| and           | first & second                       | first AND second                       |
| or            | first | second                       | first OR second                        |
| not           | !term                                | \-term                                 |
| one of        | (first | second | third)             | (first OR second OR third)             |
| wildcard      | first * second                       | first * second                         |
| numeric range | start-end                            | start..end                             |
| distance      | first ~count second                  | first AROUND(count) second             |
| escaped       | \token                               |                                        |

## Acknowledgements

Leita uses the following:

- [Logos](https://github.com/maciejhirsz/logos)
- [Clap](https://github.com/clap-rs/clap)