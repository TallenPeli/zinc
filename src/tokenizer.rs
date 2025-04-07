// Tokenizer
use crate::Settings;
use crate::zlog::{self};
use std::{result::Result, usize};

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
    TokTypeBool,      // 'bool' ✅
    TokTypef32,       // 'f32' ✅
    TokTypef64,       // 'f64' ✅
    TokStringLiteral, // A string literal i.e. "Hello, world!\n" ✅
    TokNumLiteral,    // For number literals i.e. '1' or '1.5' ✅
    TokCharLiteral,   // for character literals i.e. 'a'
    TokIdentifier,    // The name of the variable ✅
    TokAssign,        // the '=' sign ✅
    TokLeftParen,     // '(' ✅
    TokRightParen,    // ')' ✅
    TokLeftBrace,     // '{' ✅
    TokRightBrace,    // '}' ✅
    TokLeftBracket,   // '[' ✅
    TokRightBracket,  // ']' ✅
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
    TokPound,         // '#' directives
    TokDive,          // 'dive' used for imports ✅
    TokBellyflop,     // 'bellyflop' used for C immports ✅
    TokFrom,          // 'from' token to get an import ✅
    TokAlias,         // 'type' for type aliases ✅
    TokAt,            // '@' for calling a macro ✅

    // Operators
    TokAmpersand,       // '&'  (reference operator) ✅
    TokAsterisk,        // '*'  (dereference or multiplication) ✅
    TokArrow,           // '->' (used for return types) ✅
    TokColon,           // ':'  (for type annotations) ✅
    TokDoubleColon,     // '::' (alternative for scope resolution) ✅
    TokModulo,          // '%'  (modulo operator) ✅
    TokPlus,            // '+'  (addition or unary plus) ✅
    TokMinus,           // '-'  (subtraction or unary minus) ✅
    TokIncrement,       // '++' (increment) ✅
    TokDecrement,       // '--' (decrement) ✅
    TokEquals,          // '==' (comparison) ✅
    TokTimesEqual,      // '*=' (multiplication assignment) ✅
    TokDivideEqual,     // '/=' (division assignment) ✅
    TokPlusEqual,       // '+=' (addition assignment) ✅
    TokMinusEqual,      // '-=' (subtraction assignment) ✅
    TokModuloEqual,     // '%=' (modulo assignment) ✅
    TokNotEquals,       // '!=' (comparison) ✅
    TokBitAndEqual,     // '&=' (bitwise AND assignment)
    TokBitOrEqual,      // '|=' (bitwise OR assignment)
    TokBitXorEqual,     // '^=' (bitwise XOR assignment) ✅
    TokBitNotEqual,     // '~=' (bitwise NOT assignment)
    TokLeftShiftEqual,  // '<<=' (bitwise left shift assignment)
    TokRightShiftEqual, // '>>=' (bitwise right shift assignment)
    TokLeftAngle,       // '<'  (comparison) ✅
    TokRightAngle,      // '>'  (comparison) ✅
    TokLessEqual,       // '<=' (comparison) ✅
    TokGreaterEqual,    // '>=' (comparison) ✅
    TokBitOr,           // '|'  (bitwise OR) ✅
    TokBitXor,          // '^'  (bitwise XOR) ✅
    TokBitNot,          // '~'  (bitwise NOT) ✅
    TokLeftShift,       // '<<' (bitwise left shift) ✅
    TokRightShift,      // '>>' (bitwise right shift) ✅
    TokEllipsis,        // '...' (variadic functions or range) ✅
    TokQuestion,        // '?'  (optional types or ternary operator) ✅
    TokDollar,          // '$'  (Might use later) ✅
    TokRange,           // '..' (used for range. i.e. 1..10 '1 to 10') ✅
    TokDivide,          // '/'  (division) ✅

    // Logical Operators
    TokAnd,  // '&&'  (logical AND) ✅
    TokOr,   // '||'  (logical OR) ✅
    TokBang, // '!'   (logical NOT) ✅

    // Control flow
    TokSwitch,  // C-Style switch statements ✅
    TokCase,    // cases for switch statements ✅
    TokDefault, // the default case for a switch statement ✅

    // Misc.
    TokEOF,     // End of file token to stop the parser ✅
    TokNewline, // Newline token to track line numbers ✅
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Token {
    pub tok_type: TokenType,
    pub value: Option<String>,
    pub line: u32,
}

pub struct Tokenizer<'a> {
    src: String,
    index: usize,
    line: u32,
    settings: &'a Settings,
}

impl<'a> Tokenizer<'a> {
    /// # New
    ///
    /// Create a new tokenizer with the given source code and settings.
    ///
    /// # Arguments
    ///
    /// * `src` - The source code to tokenize.
    /// * `&settings` - The settings to use for tokenization.
    ///
    /// # Usage
    ///
    /// ```
    /// let src = String::from("fun main() -> i32 { return 0; }");
    /// let settings = Settings::default();
    /// let mut tokenizer = Tokenizer::new(src, &settings);
    /// let tokens = tokenizer.tokenize().unwrap();
    /// ```
    ///
    pub fn new(src: String, settings: &'a Settings) -> Self {
        Tokenizer {
            src,
            index: 0,
            line: 1,
            settings,
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens: Vec<Token> = Vec::new();
        while let Some(ch) = self.peek(0) {
            let mut tok_buf: String;

            if ch.is_whitespace() {
                if ch == '\n' {
                    tokens.push(Token {
                        tok_type: TokenType::TokNewline,
                        value: None,
                        line: self.line,
                    });
                    self.line += 1;
                }
                self.consume(1);
                continue;
            } else if ch.is_alphabetic() {
                tok_buf = String::new();
                tok_buf.push(ch);
                self.consume(1);
                while let Some(next_char) = self.peek(0) {
                    if next_char.is_alphanumeric() || "_".contains(next_char) {
                        tok_buf.push(next_char);
                        self.consume(1);
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
                    "bool" => TokenType::TokTypeBool,
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
                    line: self.line,
                });
            } else if ch.is_numeric() {
                tok_buf = String::new();
                tok_buf.push(ch);
                self.consume(1);
                let mut is_float = false;
                while let Some(next_char) = self.peek(0) {
                    if next_char.is_numeric() {
                        tok_buf.push(next_char);
                        self.consume(1);
                    } else if '.' == next_char {
                        // Check if there is a decimal point
                        self.consume(1);
                        if Some('.') == self.peek(0) {
                            tokens.push(Token {
                                tok_type: TokenType::TokNumLiteral,
                                value: Some(tok_buf.clone()),
                                line: self.line,
                            });
                            if Some('.') == self.peek(1) {
                                self.consume(2);
                                tokens.push(Token {
                                    tok_type: TokenType::TokEllipsis,
                                    value: None,
                                    line: self.line,
                                });
                            } else {
                                self.consume(1);
                                tokens.push(Token {
                                    tok_type: TokenType::TokRange,
                                    value: None,
                                    line: self.line,
                                });
                            }
                            tok_buf = String::new();
                        } else {
                            if !is_float {
                                is_float = true;
                                self.consume(1);
                            } else {
                                return Err(format!(
                                    "[Line {}] Cannot put two decimal points in a float literal",
                                    self.line
                                ));
                            }
                        }
                    } else {
                        break;
                    }
                }
                tokens.push(Token {
                    tok_type: TokenType::TokNumLiteral,
                    value: Some(tok_buf),
                    line: self.line,
                });
            } else if ch == '/' {
                if Some('/') == self.peek(1) {
                    self.consume(2);
                    while let Some(next_char) = self.peek(0) {
                        if next_char == '\n' {
                            self.consume(1);
                            self.line += 1;
                            break;
                        } else {
                            self.consume(1);
                        }
                    }
                } else if Some('*') == self.peek(1) {
                    self.consume(2);
                    while let Some(next_char) = self.peek(0) {
                        if next_char == '*' && self.peek(1) == Some('/') {
                            self.consume(2);
                            break;
                        } else {
                            if next_char == '\n' {
                                self.line += 1;
                            }
                            self.consume(1);
                        }
                    }
                } else {
                    tokens.push(Token {
                        tok_type: TokenType::TokDivide,
                        value: None,
                        line: self.line,
                    });
                    self.consume(1);
                }
            } else if ch == '"' {
                tok_buf = String::new();
                tok_buf.push(ch);
                self.consume(1);
                while let Some(next_char) = self.peek(0) {
                    if next_char == '"' {
                        tok_buf.push(next_char);
                        self.consume(1);
                        break;
                    } else {
                        tok_buf.push(next_char);
                        self.consume(1);
                    }
                }
                tokens.push(Token {
                    tok_type: TokenType::TokStringLiteral,
                    value: Some(tok_buf),
                    line: self.line,
                });
            } else {
                self.consume(1);
                let tok_type: TokenType = match ch {
                    ':' => {
                        if Some(':') == self.peek(0) {
                            self.consume(1);
                            TokenType::TokDoubleColon
                        } else {
                            TokenType::TokColon
                        }
                    }
                    '!' => {
                        if Some('=') == self.peek(0) {
                            self.consume(1);
                            TokenType::TokNotEquals
                        } else {
                            TokenType::TokBang
                        }
                    }
                    '^' => {
                        if Some('=') == self.peek(0) {
                            self.consume(1);
                            TokenType::TokBitXorEqual
                        } else {
                            TokenType::TokBitXor
                        }
                    }
                    '~' => {
                        if Some('=') == self.peek(0) {
                            self.consume(1);
                            TokenType::TokBitNotEqual
                        } else {
                            TokenType::TokBitNot
                        }
                    }
                    '-' => {
                        if Some('>') == self.peek(0) {
                            self.consume(1);
                            TokenType::TokArrow
                        } else if Some('-') == self.peek(0) {
                            self.consume(1);
                            TokenType::TokDecrement
                        } else {
                            TokenType::TokMinus
                        }
                    }
                    '+' => {
                        if Some('+') == self.peek(0) {
                            self.consume(1);
                            TokenType::TokIncrement
                        } else if Some('=') == self.peek(0) {
                            self.consume(1);
                            TokenType::TokPlusEqual
                        } else {
                            TokenType::TokPlus
                        }
                    }
                    '*' => {
                        if Some('=') == self.peek(0) {
                            self.consume(1);
                            TokenType::TokTimesEqual
                        } else {
                            TokenType::TokAsterisk
                        }
                    }
                    '%' => {
                        if Some('=') == self.peek(0) {
                            self.consume(1);
                            TokenType::TokModuloEqual
                        } else {
                            TokenType::TokModulo
                        }
                    }
                    '<' => {
                        if Some('=') == self.peek(0) {
                            self.consume(1);
                            TokenType::TokLessEqual
                        } else if Some('<') == self.peek(0) {
                            self.consume(1);
                            TokenType::TokLeftShift
                        } else {
                            TokenType::TokLeftAngle
                        }
                    }
                    '>' => {
                        if Some('=') == self.peek(0) {
                            self.consume(1);
                            TokenType::TokGreaterEqual
                        } else if Some('>') == self.peek(0) {
                            self.consume(1);
                            TokenType::TokRightShift
                        } else {
                            TokenType::TokRightAngle
                        }
                    }
                    '.' => {
                        if Some('.') == self.peek(0) {
                            self.consume(1);
                            TokenType::TokRange
                        } else if Some('.') == self.peek(1) {
                            self.consume(1);
                            TokenType::TokEllipsis
                        } else {
                            TokenType::TokDot
                        }
                    }
                    '=' => {
                        if Some('=') == self.peek(0) {
                            self.consume(1);
                            TokenType::TokEquals
                        } else {
                            TokenType::TokAssign
                        }
                    }
                    '|' => {
                        if Some('|') == self.peek(0) {
                            self.consume(1);
                            TokenType::TokBitOr
                        } else if Some('=') == self.peek(0) {
                            self.consume(1);
                            TokenType::TokBitOrEqual
                        } else {
                            TokenType::TokOr
                        }
                    }
                    '&' => {
                        if Some('&') == self.peek(0) {
                            self.consume(1);
                            TokenType::TokAnd
                        } else if Some('=') == self.peek(0) {
                            self.consume(1);
                            TokenType::TokBitAndEqual
                        } else {
                            TokenType::TokAmpersand
                        }
                    }
                    '$' => TokenType::TokDollar,
                    '?' => TokenType::TokQuestion,
                    '@' => TokenType::TokAt,
                    '#' => TokenType::TokPound,
                    ';' => TokenType::TokSemi,
                    '{' => TokenType::TokLeftBrace,
                    '}' => TokenType::TokRightBrace,
                    '(' => TokenType::TokLeftParen,
                    ')' => TokenType::TokRightParen,
                    '[' => TokenType::TokLeftBracket,
                    ']' => TokenType::TokRightBracket,
                    ',' => TokenType::TokComma,
                    _ => {
                        continue;
                    }
                };
                // Push the token
                tokens.push(Token {
                    tok_type,
                    value: None,
                    line: self.line,
                });
            }
        }
        tokens.push(Token {
            tok_type: TokenType::TokEOF,
            value: None,
            line: self.line,
        });
        zlog::verbose(
            &format!(
                "Tokenization Completed. Lines: {}, Tokens: {}",
                self.line,
                tokens.len()
            ),
            self.settings,
        );
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
    fn peek(&self, forward: usize) -> Option<char> {
        if self.index + forward < self.src.len() {
            self.src.chars().nth(self.index + forward)
        } else {
            None
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
    fn consume(&mut self, amount: usize) {
        if self.index + amount <= self.src.len() {
            self.index += amount;
        } else {
            self.index = self.src.len();
        }
    }
}
