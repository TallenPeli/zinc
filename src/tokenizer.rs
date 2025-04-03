use std::{result::Result, sync::OnceLock, usize};

use crate::zlog;

static SOURCE: OnceLock<String> = OnceLock::new();
static mut INDEX: usize = 0;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum TokenType {
    TokTypeChar,      // 'char'
    TokTypeString,    // 'String'
    TokTypei64,       // 'i64'
    TokTypeu64,       // 'u64'
    TokTypei32,       // 'i32'
    TokTypeu32,       // 'u32'
    TokTypei16,       // 'i16'
    TokTypeu16,       // 'u16'
    TokTypei8,        // 'i8'
    TokTypeu8,        // 'u8'
    TokStringLiteral, // A string literal i.e. "Hello, world!\n"
    TokNumLiteral,    // For number literals i.e. '1' or '1.5'
    TokCharLiteral,   // for character literals i.e. 'a'
    TokIdentifier,    // The name of the variable
    TokAssign,        // the '=' sign
    TokLParen,        // '('
    TokRParen,        // ')'
    TokLBrace,        // '{'
    TokRBrace,        // '}'
    TokLBracket,      // '[
    TokRBracket,      // ']'
    TokMain,          // A program entry 'main'
    TokSemi,          // ';'
    TokPeriod,        // '.'
    TokComma,         // ','
    TokIf,            // an if statement 'if'
    TokElse,          // an else statement 'else'
    TokDo,            // A do statement 'do'
    TokWhen,          // A when statement `x = y when z == true`
    TokWhile,         // a while statement 'while'
    TokFor,           // A for statement 'for'
    TokReturn,        // A return operation 'return'
    TokBreak,         // A break operation. Breaks out of a loop 'break;'
    TokContinue,      // A continue operation. Continues to the next iteration 'continue;'
    TokTry,           // For try blocks
    TokCatch,         // To catch errors
    TokThrow,         // Throw an exception to catch
    TokFun,           // A 'fun' function declaration
    TokStruct,        // A struct keyword 'struct'
    TokEnum,          // A enum keyword 'enum'
    TokLet,           // A let keyword 'let'
    TokConst,         // A const keyword. Makes variable immutable 'const'
    TokType,          // For user defined types
    TokPound,         // '#' for macros and directives
    TokDive,          // 'dive' used for imports
    TokBellyflop,     // 'bellyflop' used for C immports
    TokFrom,          // 'from' token to get an import
    TokAlias,         // 'type' for type aliases
    TokMacro,         // Used to tell the compiler to get a macro's value. `@macro`

    // Operators
    TokAmpersand,   // '&'  (reference operator)
    TokAsterisk,    // '*'  (dereference or multiplication)
    TokArrow,       // '->' (used for return types)
    TokColon,       // ':'  (for type annotations)
    TokDoubleColon, // '::' (alternative for scope resolution)
    TokModulus,     // '%'  (modulo operator)
    TokIncrement,   // '++' (increment)
    TokDecrement,   // '--' (decrement)
    TokEquals,      // '==' (comparison)
    TokNotEquals,   // '!=' (comparison)
    TokLAngle,      // '<'  (comparison)
    TokRAngle,      // '>'  (comparison)
    TokLessEq,      // '<=' (comparison)
    TokGreaterEq,   // '>=' (comparison)
    TokBitAnd,      // '&'  (bitwise AND)
    TokBitOr,       // '|'  (bitwise OR)
    TokBitXor,      // '^'  (bitwise XOR)
    TokBitNot,      // '~'  (bitwise NOT)
    TokLShift,      // '<<' (bitwise left shift)
    TokRShift,      // '>>' (bitwise right shift)
    TokEllipsis,    // '...' (variadic functions or range)
    TokQuestion,    // '?'  (optional types or ternary operator)
    TokDollar,      // '$'  (if used in macros or templates)
    TokRange,       // '..' (used for range. i.e. 1..10 '1 to 10')

    // Logical Operators
    TokAnd, // '&&'  (logical AND)
    TokOr,  // '||'  (logical OR)
    TokNot, // '!'   (logical NOT)

    // Control flow
    TokSwitch,  // C-Style switch statements
    TokCase,    // cases for switch statements
    TokDefault, // the default case for a switch statement

    // Misc.
    TokEOF, // End of file token to stop the parser
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Token {
    pub tok_type: TokenType,
    pub value: Option<String>,
    // pub line: u32,
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
pub fn tokenize(src: String) -> Result<Vec<Token>, String> {
    match SOURCE.set(src) {
        Ok(()) => {}
        Err(e) => {
            zlog::err(&format!(
                "Failed to set the source code buffer in Tokenizer due to error: {}",
                e
            ));
            return Err(e.to_string());
        }
    }
    let mut tokens: Vec<Token> = Vec::new();
    let mut line: i32 = 0;
    while let Some(ch) = peek(0) {
        print!("{ch}");
        let mut tok_buf: String;

        if ch.is_whitespace() {
            consume(1).expect("Failed to consume character");
            continue;
        } else if ch.is_alphabetic() {
            tok_buf = String::new();
            tok_buf.push(ch);
            consume(1).expect("Failed to consume character");
            while let Some(next_char) = peek(0) {
                if next_char.is_alphanumeric() || "_".contains(next_char) {
                    tok_buf.push(next_char);
                    consume(1).expect("Failed to consume the next char.");
                } else {
                    break;
                }
            }

            let token_type: TokenType = match tok_buf.as_str() {
                "char" => TokenType::TokTypeChar,
                "String" => TokenType::TokTypeString,
                "i64" => TokenType::TokTypei64,
                "u64" => TokenType::TokTypeu64,
                "i32" => TokenType::TokTypei32,
                "u32" => TokenType::TokTypeu32,
                "i16" => TokenType::TokTypei16,
                "u16" => TokenType::TokTypeu16,
                "i8" => TokenType::TokTypei8,
                "u8" => TokenType::TokTypeu8,
                _ => TokenType::TokIdentifier,
            };

            tokens.push(Token {
                tok_type: token_type.clone(),
                value: if matches!(token_type, TokenType::TokIdentifier) {
                    Some(tok_buf)
                } else {
                    None
                },
            });
        } else if ch.is_numeric() {
            tok_buf = String::new();
            tok_buf.push(ch);
            consume(1).expect("Failed to consume char");
            while let Some(next_char) = peek(0) {
                if next_char.is_numeric() {
                    tok_buf.push(next_char);
                    consume(1).expect("Failed to consume the next char");
                } else {
                    break;
                }
            }
            tokens.push(Token {
                tok_type: TokenType::TokNumLiteral,
                value: Some(tok_buf),
            });
        } else if ch == '"' {
            tok_buf = String::new();
            tok_buf.push(ch);
            consume(1).expect("Failed to consume character");
            while let Some(next_char) = peek(0) {
                if next_char == '"' {
                    tok_buf.push(next_char);
                    consume(1).expect("Failed to consume the next char");
                    break;
                } else {
                    tok_buf.push(next_char);
                    consume(1).expect("Failed to consume the next char");
                }
            }
            tokens.push(Token {
                tok_type: TokenType::TokStringLiteral,
                value: Some(tok_buf),
            });
        } else if ch == '@' {
            tok_buf = String::from("@");
            consume(1).expect("Failed to consume the next char");

            while let Some(next_char) = peek(0) {
                if next_char == ' ' {
                    break;
                } else {
                    tok_buf.push(next_char);
                    consume(1)?;
                }
            }

            if tok_buf.len() == 1 {
                println!("Must have an identifier after an '@' Exiting now...");
                return Err("Invalid '@' usage".into());
            }
        } else if ch == '=' {
            if peek(1) == Some('=') {
                tokens.push(Token {
                    tok_type: TokenType::TokEquals,
                    value: None,
                });
                consume(2)?;
                break;
            }
            tokens.push(Token {
                tok_type: TokenType::TokAssign,
                value: None,
            });
            consume(1).expect("Failed to consume character");
        } else if ch == ';' {
            tokens.push(Token {
                tok_type: TokenType::TokSemi,
                value: None,
            });
            consume(1).expect("Faled to consume character");
        // [TODO]: More tokens
        } else if ch == '/' {
            if Some('/') == peek(1) {
                consume(2)?;
                while Some('\n') != peek(1) {
                    consume(1)?;
                }
                line += 1;
                consume(1)?;
            }
            if Some('*') == peek(1) {
                consume(2)?;
                while let Some(next_char) = peek(1) {
                    if next_char == '*' {
                        if peek(2) == 
                    }
                }
            }
        } else {
            println!("Unknown token");
            consume(1).expect("Failed to consume char.");
        }
    }

    println!("Consumed all the chars");
    Ok(tokens)
}

fn peek(forward: usize) -> Option<char> {
    unsafe {
        let source = SOURCE.get()?;

        if INDEX + forward < source.len() {
            source.chars().nth(INDEX + forward)
        } else {
            None
        }
    }
}

fn consume(amount: usize) -> Result<(), String> {
    unsafe {
        if let Some(source) = SOURCE.get() {
            if INDEX + amount < source.len() {
                INDEX += amount;
                Ok(())
            } else {
                Err(format!(
                    "Unable to consume {amount} of characters. Out of bounds."
                ))
            }
        } else {
            Err("SOURCE not initialized".to_string())
        }
    }
}
