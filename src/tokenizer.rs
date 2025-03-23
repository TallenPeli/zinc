enum _TokenTypes {
    TokKeyword, // Things like `int` `string` `return` etc...
    TokIdentifier, // Variable names i.e. `int x = 1;`, `x` would be the identifier.
    TokOperator, // Operations, such as `+`, `-`, `*`, `/`, etc...
    TokPunctuator, // Punctuation, such as `;`, ',', '.', '->', etc...
    TokLiteral, // The literal value of something. i.e. `int x = 1;`, '1' would be the identifier.
    TokComment, // The entirety of a comment from `//` to the end of the line `\n`
}

struct Token {
    tok_type: u8,
    tok_identifier: Option<String>,
    tok_literal: Option<String>,
}

/// # Tokenizer
///
/// Tokenizes a sting of source code
///
/// # Arguments
///
/// * `src` - A string containing the source code to tokenize
///
/// # Example
/// ```
/// tokens: Vec<Token> = tokenize(src_code);
/// ```
pub fn tokenize(src: String) -> Result<Vec<Token>,String>
{
    let mut tokens: Vec<Token> = Vec::new();
    for line in src.lines() {
        let tok_buf: String = String::new();
        
    }
    Ok(tokens)
}
