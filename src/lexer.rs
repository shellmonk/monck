use logos::Logos;

/**

    data Library n (
        id uuid!,
        name ('lib1' | 'lib2' | 'lib2' | name!),
        address (ORACLE.string |> STD.string.length 5 50),
        open false,
        books (Book (lid id) |> STD.repeat n)
    )

    data Book lid (
        id (ORACLE.integer |> STD.integer.range 100 200),
        name (name 10 20),
        lib_id lid,
        author Author
    )

    data Author (
        id uuid!,
        name name!,
        type 'author',
        books 10,
        height 1.80
    )

    var a (1)

    gen sequence (STD.inc a)
    gen uuid (ORACLE.uuid.4 |> STD.uuid.to_string |> STD.string.uppercase)
    gen name min_len max_len (ORACLE.string.names.british |> STD.string.length min_len max_len |> STD.string.uppercase)

    main (Library 5 |> STD.repeat 10)

**/

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\r\f]+")]
#[logos(skip r"//.*")]
pub enum Token {

    // Single char tokens
    #[token("(")]
    LtParen,
    #[token(")")]
    RtParen,
    #[token("[")]
    LtBracket,
    #[token("]")]
    RtBracket,
    #[token(",")]
    Comma,
    #[token(".")]
    Dot,
    #[token("-")]
    Minus,
    #[token("+")]
    Plus,
    #[token("!")]
    Exclamation,
    #[token("|")]
    Union,

    // Two char tokens
    #[token("|>")]
    Pipe,

    #[regex("[a-zA-Z_][a-zA-Z0-9_]*", monck_identifier)]
    Identifier(String),
    #[regex(r#""[^"]*""#, monck_string)]
    String(String),
    #[regex(r#"[0-9]+"#, monck_integer)]
    Integer(i64),
    #[regex(r#"[0-9]+(\.[0-9]+)"#, monck_float)]
    Float(f64),
    #[token("false", |_| false)]
    #[token("true", |_| true)]
    Bool(bool),

    #[token("null")]
    Null,

    // Keywords
    #[token("var")]
    Var,
    #[token("data")]
    Data,
    #[token("gen")]
    Gen,
    #[token("main")]
    Main,
    #[token("const")]
    Const,

}

fn monck_integer(lexer: &mut logos::Lexer<Token>) -> i64 {
    let slice = lexer.slice();
    slice.parse::<i64>().expect(format!("Error parsing int {}:{}", lexer.span().start, lexer.span().end).as_str())
}

fn monck_float(lexer: &mut logos::Lexer<Token>) -> f64 {
    let slice = lexer.slice();
    slice.parse::<f64>().expect(format!("Error parsing float {}:{}", lexer.span().start, lexer.span().end).as_str())
}

fn monck_string(lexer: &mut logos::Lexer<Token>) -> String {
    let slice = lexer.slice();
    slice[1..slice.len() - 1].to_string()
}

fn monck_identifier(lexer: &mut logos::Lexer<Token>) -> String {
    let slice = lexer.slice();
    slice.to_string()
}

#[cfg(test)]
mod tests {
    use logos::Logos;
    use crate::lexer::Token;
    use std::fs;

    #[test]
    fn it_works() {

        let content = fs::read_to_string("examples/v3.monck").unwrap();

        let mut lexer = Token::lexer(content.as_str());
        let result = 2 + 2;
        while let Some(token) = lexer.next() {
            println!("token: {:?}", token);
        }
        assert_eq!(result, 5);
    }
}