use std::{result::Result, sync::OnceLock, usize};

// use crate::zlog::{self};

static SOURCE: OnceLock<String> = OnceLock::new();
static mut INDEX: usize = 0;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum TokenType {
    TokTypeChar,      // 'char' ✅
    TokTypeString,    // 'string' ✅
    TokTypei64,       // 'i64' ✅
    TokTypeu64,       // 'u64' ✅
    TokTypei32,       // 'i32' ✅
    TokTypeu32,       // 'u32' ✅
    TokTypei16,       // 'i16' ✅
    TokTypeu16,       // 'u16' ✅
    TokTypei8,        // 'i8' ✅
    TokTypeu8,        // 'u8' ✅
    TokTypeBool,      // 'bool'
    TokTypef32,       // 'f32'
    TokTypef64,       // 'f64'
    TokStringLiteral, // A string literal i.e. "Hello, world!\n" ✅
    TokNumLiteral,    // For number literals i.e. '1' or '1.5'
    TokCharLiteral,   // for character literals i.e. 'a'
    TokIdentifier,    // The name of the variable
    TokAssign,        // the '=' sign ✅
    TokLeftParen,     // '(' ✅
    TokRightParen,    // ')' ✅
    TokLeftBrace,     // '{' ✅
    TokRightBrace,    // '}' ✅
    TokLeftBracket,   // '['
    TokRightBracket,  // ']'
    TokMain,          // A program entry 'main' ✅
    TokSemi,          // ';' ✅
    TokDot,           // '.' ✅
    TokComma,         // ',' ✅
    TokIf,            // an if statement 'if' ✅
    TokElse,          // an else statement 'else' ✅
    TokDo,            // A do statement 'do' ✅
    TokWhen,          // A when statement `x = y when z == true` ✅
    TokWhile,         // a while statement 'while' ✅
    TokFor,           // A for statement 'for' ✅
    TokIn,            // A statement used for iteration 'in' ✅
    TokReturn,        // A return operation 'return' ✅
    TokBreak,         // A break operation. Breaks out of a loop 'break' ✅
    TokContinue,      // A continue operation. Continues to the next iteration 'continue' ✅
    TokTry,           // For try blocks ✅
    TokCatch,         // To catch errors ✅
    TokThrow,         // Throw an exception to catch ✅
    TokFun,           // A 'fun' function declaration ✅
    TokStruct,        // A struct keyword 'struct' ✅
    TokEnum,          // A enum keyword 'enum' ✅
    TokConst,         // A const keyword. Makes variable immutable 'const' ✅
    TokType,          // For user defined types
    TokPound,         // '#' for macros and directives
    TokDive,          // 'dive' used for imports ✅
    TokBellyflop,     // 'bellyflop' used for C immports ✅
    TokFrom,          // 'from' token to get an import ✅
    TokAlias,         // 'type' for type aliases
    TokMacro,         // Used to tell the compiler to get a macro's value. `@macro`

    // Operators
    TokAmpersand,    // '&'  (reference operator)
    TokAsterisk,     // '*'  (dereference or multiplication)
    TokArrow,        // '->' (used for return types)
    TokColon,        // ':'  (for type annotations)
    TokDoubleColon,  // '::' (alternative for scope resolution)
    TokModulus,      // '%'  (modulo operator)
    TokPlus,         // '+'  (addition or unary plus)
    TokMinus,        // '-'  (subtraction or unary minus)
    TokIncrement,    // '++' (increment)
    TokDecrement,    // '--' (decrement)
    TokEquals,       // '==' (comparison)
    TokTimesEqual,   // '*=' (multiplication assignment)
    TokDivideEqual,  // '/=' (division assignment)
    TokPlusEqual,    // '+=' (addition assignment)
    TokMinusEqual,   // '-=' (subtraction assignment)
    TokNotEquals,    // '!=' (comparison)
    TokLeftAngle,    // '<'  (comparison)
    TokRightAngle,   // '>'  (comparison)
    TokLessEqual,    // '<=' (comparison)
    TokGreaterEqual, // '>=' (comparison)
    TokBitOr,        // '|'  (bitwise OR) ✅
    TokBitXor,       // '^'  (bitwise XOR) ✅
    TokTilde,        // '~'  (bitwise NOT) ✅
    TokLeftShift,    // '<<' (bitwise left shift)
    TokRightShift,   // '>>' (bitwise right shift)
    TokEllipsis,     // '...' (variadic functions or range)
    TokQuestion,     // '?'  (optional types or ternary operator)
    TokDollar,       // '$'  (Might use later)
    TokRange,        // '..' (used for range. i.e. 1..10 '1 to 10')
    TokDivide,       // '/'  (division)

    // Logical Operators
    TokAnd,  // '&&'  (logical AND)
    TokOr,   // '||'  (logical OR)
    TokBang, // '!'   (logical NOT)

    // Control flow
    TokSwitch,  // C-Style switch statements
    TokCase,    // cases for switch statements
    TokDefault, // the default case for a switch statement

    // Misc.
    TokEOF,     // End of file token to stop the parser
    TokNewline, // Newline token to track line numbers
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
            return Err(e.to_string());
        }
    }
    let mut tokens: Vec<Token> = Vec::new();
    let mut line: i32 = 0;
    while let Some(ch) = peek(0) {
        let mut tok_buf: String;

        if ch.is_whitespace() {
            consume(1)?;
            continue;
        } else if ch.is_alphabetic() {
            tok_buf = String::new();
            tok_buf.push(ch);
            consume(1)?;
            while let Some(next_char) = peek(0) {
                if next_char.is_alphanumeric() || "_".contains(next_char) {
                    tok_buf.push(next_char);
                    consume(1)?;
                } else {
                    break;
                }
            }

            let token_type: TokenType = match tok_buf.as_str() {
                "char" => TokenType::TokTypeChar,
                "string" => TokenType::TokTypeString,
                "f32" => TokenType::TokTypef32,
                "f64" => TokenType::TokTypef64,
                "i64" => TokenType::TokTypei64,
                "u64" => TokenType::TokTypeu64,
                "i32" => TokenType::TokTypei32,
                "u32" => TokenType::TokTypeu32,
                "i16" => TokenType::TokTypei16,
                "u16" => TokenType::TokTypeu16,
                "i8" => TokenType::TokTypei8,
                "u8" => TokenType::TokTypeu8,
                "struct" => TokenType::TokStruct,
                "fun" => TokenType::TokFun,
                "enum" => TokenType::TokEnum,
                "const" => TokenType::TokConst,
                "try" => TokenType::TokTry,
                "catch" => TokenType::TokCatch,
                "throw" => TokenType::TokThrow,
                "return" => TokenType::TokReturn,
                "bellyflop" => TokenType::TokBellyflop,
                "dive" => TokenType::TokDive,
                "from" => TokenType::TokFrom,
                "switch" => TokenType::TokSwitch,
                "case" => TokenType::TokCase,
                "default" => TokenType::TokDefault,
                "if" => TokenType::TokIf,
                "else" => TokenType::TokElse,
                "type" => TokenType::TokAlias,
                "do" => TokenType::TokDo,
                "while" => TokenType::TokWhile,
                "break" => TokenType::TokBreak,
                "continue" => TokenType::TokContinue,
                "for" => TokenType::TokFor,
                "in" => TokenType::TokIn,
                "when" => TokenType::TokWhen,
                "main" => TokenType::TokMain,
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
                    consume(1)?;
                } else {
                    break;
                }
            }
            tokens.push(Token {
                tok_type: TokenType::TokNumLiteral,
                value: Some(tok_buf),
            });
        } else if ch == '(' {
            tokens.push(Token {
                tok_type: TokenType::TokLeftParen,
                value: None,
            });
            consume(1)?;
        } else if ch == ')' {
            tokens.push(Token {
                tok_type: TokenType::TokRightParen,
                value: None,
            });
            consume(1)?;
        } else if ch == ',' {
            tokens.push(Token {
                tok_type: TokenType::TokComma,
                value: None,
            });
            consume(1)?;
        } else if ch == ':' {
            if Some(':') == peek(1) {
                tokens.push(Token {
                    tok_type: TokenType::TokDoubleColon,
                    value: None,
                });
                consume(2)?;
            } else {
                tokens.push(Token {
                    tok_type: TokenType::TokColon,
                    value: None,
                });
                consume(1)?;
            }
        } else if ch == '!' {
            tokens.push(Token {
                tok_type: TokenType::TokBang,
                value: None,
            });
            consume(1)?;
        } else if ch == '^' {
            tokens.push(Token {
                tok_type: TokenType::TokBitXor,
                value: None,
            });
            consume(1)?;
        } else if ch == '~' {
            tokens.push(Token {
                tok_type: TokenType::TokTilde,
                value: None,
            });
            consume(1)?;
        } else if ch == '-' {
            if Some('>') == peek(1) {
                tokens.push(Token {
                    tok_type: TokenType::TokArrow,
                    value: None,
                });
                consume(2)?;
            } else if Some('-') == peek(1) {
                tokens.push(Token {
                    tok_type: TokenType::TokDecrement,
                    value: None,
                });
                consume(2)?;
            } else if Some('=') == peek(1) {
                tokens.push(Token {
                    tok_type: TokenType::TokMinusEqual,
                    value: None,
                });
                consume(2)?;
            } else {
                tokens.push(Token {
                    tok_type: TokenType::TokMinus,
                    value: None,
                });
                consume(1)?;
            }
        } else if ch == '+' {
            if Some('+') == peek(1) {
                tokens.push(Token {
                    tok_type: TokenType::TokIncrement,
                    value: None,
                });
                consume(2)?;
            } else if Some('=') == peek(1) {
                tokens.push(Token {
                    tok_type: TokenType::TokPlusEqual,
                    value: None,
                });
                consume(2)?;
            } else {
                tokens.push(Token {
                    tok_type: TokenType::TokPlus,
                    value: None,
                });
                consume(1)?;
            }
        } else if ch == '*' {
            if Some('=') == peek(1) {
                tokens.push(Token {
                    tok_type: TokenType::TokTimesEqual,
                    value: None,
                });
                consume(2)?;
            } else {
                tokens.push(Token {
                    tok_type: TokenType::TokAsterisk,
                    value: None,
                });
                consume(1)?;
            }
        } else if ch == '<' {
            // [TODO]: Implement less than or equal to token
            if Some('=') == peek(1) {
                tokens.push(Token {
                    tok_type: TokenType::TokLessEqual,
                    value: None,
                });
                consume(2)?;
            } else if Some('<') == peek(1) {
                tokens.push(Token {
                    tok_type: TokenType::TokLeftShift,
                    value: None,
                });
                consume(2)?;
            } else {
                tokens.push(Token {
                    tok_type: TokenType::TokLeftAngle,
                    value: None,
                });
                consume(1)?;
            }
        } else if ch == '>' {
            if Some('=') == peek(1) {
                tokens.push(Token {
                    tok_type: TokenType::TokGreaterEqual,
                    value: None,
                });
                consume(2)?;
            } else if Some('>') == peek(1) {
                tokens.push(Token {
                    tok_type: TokenType::TokRightShift,
                    value: None,
                });
                consume(2)?;
            } else {
                tokens.push(Token {
                    tok_type: TokenType::TokRightAngle,
                    value: None,
                });
                consume(1)?;
            }
        } else if ch == '?' {
            tokens.push(Token {
                tok_type: TokenType::TokQuestion,
                value: None,
            });
            consume(1)?;
        } else if ch == '.' {
            if Some('.') == peek(1) {
                consume(2)?;
                if Some('.') == peek(0) {
                    tokens.push(Token {
                        tok_type: TokenType::TokEllipsis,
                        value: None,
                    });
                    consume(1)?;
                } else {
                    tokens.push(Token {
                        tok_type: TokenType::TokRange,
                        value: None,
                    });
                }
            } else {
                tokens.push(Token {
                    tok_type: TokenType::TokDot,
                    value: None,
                });
                consume(1)?;
            }
        } else if ch == '"' {
            tok_buf = String::new();
            tok_buf.push(ch);
            consume(1)?;
            while let Some(next_char) = peek(0) {
                if next_char == '"' {
                    tok_buf.push(next_char);
                    consume(1)?;
                    break;
                } else {
                    tok_buf.push(next_char);
                    consume(1)?;
                }
            }
            tokens.push(Token {
                tok_type: TokenType::TokStringLiteral,
                value: Some(tok_buf),
            });
        } else if ch == '@' {
            tok_buf = String::from("@");
            consume(1)?;

            while let Some(next_char) = peek(0) {
                if next_char == ' ' {
                    break;
                } else {
                    tok_buf.push(next_char);
                    consume(1)?;
                }
            }

            if tok_buf.len() == 1 {
                return Err(format!("Invalid '@' usage on line {}", line).into());
            }

            tokens.push(Token {
                tok_type: TokenType::TokMacro,
                value: Some(tok_buf),
            });
        } else if ch == '=' {
            if peek(1) == Some('=') {
                tokens.push(Token {
                    tok_type: TokenType::TokEquals,
                    value: None,
                });
                consume(2)?;
            }
            tokens.push(Token {
                tok_type: TokenType::TokAssign,
                value: None,
            });
            consume(1)?;
        } else if ch == ';' {
            tokens.push(Token {
                tok_type: TokenType::TokSemi,
                value: None,
            });
            consume(1)?;
        } else if ch == '{' {
            tokens.push(Token {
                tok_type: TokenType::TokLeftBrace,
                value: None,
            });
            consume(1)?;
        } else if ch == '}' {
            tokens.push(Token {
                tok_type: TokenType::TokRightBrace,
                value: None,
            });
            consume(1)?;
        } else if ch == '[' {
            tokens.push(Token {
                tok_type: TokenType::TokLeftBracket,
                value: None,
            });
            consume(1)?;
        } else if ch == ']' {
            tokens.push(Token {
                tok_type: TokenType::TokRightBracket,
                value: None,
            });
            consume(1)?;
        } else if ch == '|' {
            if Some('|') == peek(1) {
                tokens.push(Token {
                    tok_type: TokenType::TokOr,
                    value: None,
                });
                consume(2)?;
            } else {
                tokens.push(Token {
                    tok_type: TokenType::TokBitOr,
                    value: None,
                });
                consume(1)?;
            }
        } else if ch == '/' {
            if Some('/') == peek(1) {
                consume(2)?;
                while let Some(next_char) = peek(0) {
                    if next_char == '\n' {
                        consume(1)?;
                        line += 1;
                        break;
                    } else {
                        consume(1)?;
                    }
                }
            } else if Some('*') == peek(1) {
                consume(2)?;
                while let Some(next_char) = peek(0) {
                    if next_char == '*' && peek(1) == Some('/') {
                        consume(2)?;
                        break;
                    } else {
                        if next_char == '\n' {
                            line += 1;
                        }
                        consume(1)?;
                    }
                }
            } else {
                tokens.push(Token {
                    tok_type: TokenType::TokDivide,
                    value: None,
                });
                consume(1)?;
            }
        } else if ch == '&' {
            consume(1)?;
            if Some('&') == peek(1) {
                consume(1)?;
                tokens.push(Token {
                    tok_type: TokenType::TokAnd,
                    value: None,
                });
            } else {
                tokens.push(Token {
                    tok_type: TokenType::TokAmpersand,
                    value: None,
                });
            }
        } else {
            println!("Unknown character {}", ch);
            consume(1)?;
        }
    }
    println!("Consumed all the chars {}", line);
    Ok(tokens)
}

/// # Peek
///
/// Peeks at a character in the source code without consuming it.
///
/// # Arguments
///
/// * `forward` - A usize of how far forward to look. 0 is the current character.
///
/// # Returns
///
/// * `Option<char>` - Returns Some(char) if there is a character to peek at, otherwise None.
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

/// # Consume
///
/// Consumes a given amount of characters from the source.
///
/// # Arguments
///
/// * `amount` - A usize of how many characters to consume
///
/// # Returns
///
/// * `Result<(), String>` - Returns Ok if successful or an error message if not.
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
