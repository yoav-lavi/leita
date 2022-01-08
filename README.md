# Leita <img height=32px width=32px src=https://raw.githubusercontent.com/yoav-lavi/leita/main/leita.svg alt="leita">
</svg>

A universal search query language that can transpile into specific search provider syntax (currently only Google syntax is supported, parts of Bing and Kagi syntax are supported since they are equivalent to Google)

A personal project to explore Rust and lexers

## Name

Leita means _"to seek"_ or _"to search"_ in Old Norse and many descendant languages

## Install

### Cargo

Clone this repository and run the following command in the repository root:

```sh
cargo install --path .
```

## Usage

```sh
leita <query>
```

## Syntax

| **Concept**   | **Leita**                            | **Google**                      |  **DuckDuckGo**  | **Bing** and **Kagi**      |
| ------------- | ------------------------------------ | ------------------------------- | ---------------- | -------------------------- |
| exact         | "term"                               | "term"                          |                  | "term"                     |
| and           | first & second                       | first AND second                |                  | first AND second           |
| or            | first \| second                      | first OR second                 |                  | first OR second            |
| not           | !term                                | \-term                          |                  | (first OR second OR third) |
| one of        | (first \| second \| third)           | (first OR second OR third)      |                  | \-term                     |
| wildcard      | first * second                       | first * second                  |                  |                            |
| numeric range | start-end                            | start..end                      |                  |                            |
| distance      | first ~count second                  | first AROUND(count) second      |                  |                            |
| escaped       | \token                               |                                 |                  |                            |
| more          |                                      |                                 | +term            |                            |
| less          |                                      |                                 | -term            |                            |
| required      |                                      |                                 |                  | +term                      |

Note: Bing and Kagi support `|` in place of `OR` and `&` in place of `AND`

### Examples

- `(dog | cat | wolf | yeti) shampoo 2021-2022 "great!" !lavender home-made` → `(dog OR cat OR wolf OR yeti) shampoo 2021..2022 "great!" -lavender home-made`
  - Tokens in quotes need not be escaped
  - Ranges are `number-number` only
- `good ~5 dog` → `good AROUND(5) dog`

## Not Supported

- At the moment an exact term (`"term"`) cannot have escaped quotes (`"\"term\""`)

## Acknowledgements

Leita uses the following:

- [Logos](https://github.com/maciejhirsz/logos)
- [Clap](https://github.com/clap-rs/clap)

Assets:

- The search icon is from [Heroicons](https://heroicons.com)
