# Leita

A universal search query language that can transpile into specific search provider syntax (currently Google only)

A personal project to explore Rust and lexers

## Name

Leita means _"to seek"_ or _"to search"_ in Old Norse and many descendant languages

## Usage

```sh
leita <query>
```

## Syntax

| **Concept**   | **Leita**                            | **Google**                             |
| ------------- | ------------------------------------ | -------------------------------------- |
| exact         | "term"                               | "term"                                 |
| and           | first & second                       | first AND second                       |
| or            | first \| second                      | first OR second                        |
| not           | !term                                | \-term                                 |
| one of        | (first \| second \| third)           | (first OR second OR third)             |
| wildcard      | first * second                       | first * second                         |
| numeric range | start-end                            | start..end                             |
| distance      | first ~count second                  | first AROUND(count) second             |
| escaped       | \token                               |                                        |


### Examples

- `(dog | cat | wolf | yeti) shampoo 2021-2022 "great!" !lavender home-made` → `(dog OR cat OR wolf OR yeti) shampoo 2021..2022 "great!" -lavender home-made`
  - Tokens in quotes need not be escaped
  - Ranges are `number-number` only

## Not Supported

- At the moment an exact term (`"term"`) cannot have escaped quotes (`"\"term\""`)

## Acknowledgements

Leita uses the following:

- [Logos](https://github.com/maciejhirsz/logos)
- [Clap](https://github.com/clap-rs/clap)
