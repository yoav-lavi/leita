use clap::Parser;
use logos::Logos;

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    query: String,
}

#[derive(Logos, Debug, PartialEq)]
enum Token {
    #[token("*")]
    Wildcard,

    #[regex("[0-9]+-[0-9]+")]
    Range,

    #[regex("\"([^\"]*)\"")]
    Quoted,

    #[regex("\\\\.")]
    Escaped,

    #[regex("~[0-9]+")]
    Distance,

    #[token("(")]
    OpenParen,

    #[token(")")]
    CloseParen,

    #[token("&")]
    And,

    #[token("|")]
    Or,

    #[regex("![a-zA-Z0-9]+")]
    Not,

    #[regex("[a-zA-Z]+")]
    Text,

    #[regex("[0-9]+")]
    Number,

    #[regex(r"[ \t\n\f]+")]
    Whitespace,

    #[error]
    Unidentified,
}

macro_rules! lex_slice_format_remove_leading {
    ($format_string: expr, $lex: expr) => {{
        let slice = $lex.slice();
        let mut chars = slice.chars();
        chars.next();
        format!($format_string, chars.as_str())
    }};
}

macro_rules! lex_slice_replace {
    ($from: expr, $to: expr, $lex: expr) => {{
        let slice = $lex.slice();
        slice.replace($from, $to)
    }};
}

fn main() {
    let args = Args::parse();

    let mut lex = Token::lexer(&args.query);

    let mut output = String::new();

    while let Some(token) = lex.next() {
        let formatted_token = match token {
            Token::Escaped => lex_slice_format_remove_leading!("{}", lex),
            Token::And => String::from("AND"),
            Token::Or => String::from("OR"),
            Token::Not => lex_slice_format_remove_leading!("-{}", lex),
            Token::Range => lex_slice_replace!("-", "..", lex),
            Token::Distance => lex_slice_format_remove_leading!("AROUND({})", lex),
            _ => lex.slice().to_string(),
        };

        output.push_str(&formatted_token);
    }

    println!("{output}")
}
