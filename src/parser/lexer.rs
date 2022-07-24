use std::str::FromStr;
use unic_ucd_ident::{is_xid_continue, is_xid_start};
use crate::error::{LexicalError, LexicalErrorType};
use crate::location::Location;
use crate::parser::token::Tok;

pub type Spanned = (Location, Tok, Location);
pub type LexResult = Result<Spanned, LexicalError>;

pub(crate) struct Lexer<T: Iterator<Item=char>> {
    chars: T,
    location: Location,
    chr: Option<char>,
}

pub(crate) fn make_tokenizer(source: &'_ str) -> impl Iterator<Item=LexResult> + '_ {
    Lexer::new(source.chars())
}

fn keyword_token(key_str: String) -> Tok {
    match key_str.as_str() {
        "uint256" => Tok::UInt256,
        "uint" => Tok::UInt256,
        "contract" => Tok::Contract,
        "function" => Tok::Function,
        "if" => Tok::If,
        "else" => Tok::Else,
        "for" => Tok::For,
        "returns" => Tok::Returns,
        "return" => Tok::Return,
        "private" => Tok::Private,
        _ => Tok::Identifier { name: key_str }
    }
}

impl<T> Lexer<T>
    where
        T: Iterator<Item=char>,
{
    fn new(input: T) -> Self {
        Lexer {
            chars: input,
            location: Location::new(0, 0),
            chr: None,
        }
    }

    fn next_spanned(&mut self) -> LexResult {
        if self.chr.is_none() {
            self.next_char();
        }
        let start = self.location;
        let tok = self.next_token()?;
        let end = self.location;
        Ok((start, tok, end))
    }

    fn next_token(&mut self) -> Result<Tok, LexicalError> {
        if let Some(c) = self.chr {
            let token = if self.is_identifier_start(c) {
                self.consume_identifier(c)?
            } else {
                self.consume_special_character(c)?
            };
            self.skip_blank();
            Ok(token)
        } else {
            // End Of File
            Ok(Tok::EOF)
        }
    }

    fn next_char(&mut self) {
        let next = self.chars.next();
        self.chr = next;
        if let Some(c) = self.chr {
            if c == '\n' {
                self.location.new_line();
            } else {
                self.location.go_right();
            }
        }
    }

    fn is_identifier_start(&self, c: char) -> bool {
        c == '_' || is_xid_start(c)
    }

    fn is_blank(&self, c: char) -> bool {
        c == ' ' || c == '\n' || c == '\t'
    }

    fn is_identifier_continue(&self, c: char) -> bool {
        match c {
            '_' | '0'..='9' => true,
            c => is_xid_continue(c),
        }
    }

    fn skip_blank(&mut self) {
        loop {
            if let Some(c) = self.chr {
                if self.is_blank(c) {
                    self.next_char();
                    continue;
                }
            }
            break;
        }
    }

    fn consume_identifier(&mut self, c: char) -> Result<Tok, LexicalError> {
        let mut text = String::new();
        text.push(c);

        loop {
            self.next_char();
            if let Some(c) = self.chr {
                if self.is_identifier_continue(c) {
                    text.push(c);
                    continue;
                }
            }
            break;
        }
        Ok(keyword_token(text))
    }

    fn consume_special_character(&mut self, c: char) -> Result<Tok, LexicalError> {
        match c {
            '0'..='9' => self.lex_number(c),
            '"' | '\'' => self.lex_literal(c),
            _ => self.consume_multiple_special_character(),
        }
    }

    fn consume_multiple_special_character(&mut self) -> Result<Tok, LexicalError> {
        let mut text = String::new();
        let mut token = None;
        while let Some(c) = self.chr {
            text.push(c);
            match text.as_str() {
                "<" => token = Some(Tok::Lt),
                ">" => token = Some(Tok::Gt),
                "=" => token = Some(Tok::Assign),
                "+" => token = Some(Tok::Plus),
                "-" => token = Some(Tok::Minus),
                "*" => token = Some(Tok::Mul),
                "/" => token = Some(Tok::Div),
                "%" => token = Some(Tok::Mod),
                "&" => token = Some(Tok::BitAnd),
                "|" => token = Some(Tok::BitOr),
                "^" => token = Some(Tok::BitXor),
                "<<" => token = Some(Tok::LShift),
                ">>" => token = Some(Tok::RShift),
                "," => {
                    token = Some(Tok::Comma);
                    break;
                }
                "{" => {
                    token = Some(Tok::LBrace);
                    break;
                }
                "}" => {
                    token = Some(Tok::RBrace);
                    break;
                }
                "(" => {
                    token = Some(Tok::LPar);
                    break;
                }
                ")" => {
                    token = Some(Tok::RPar);
                    break;
                }
                ";" => {
                    token = Some(Tok::Semi);
                    break;
                }
                "==" => {
                    token = Some(Tok::Eq);
                    break;
                }
                "!=" => {
                    token = Some(Tok::NotEq);
                    break;
                }
                "<=" => {
                    token = Some(Tok::Le);
                    break;
                }
                ">=" => {
                    token = Some(Tok::Ge);
                    break;
                }
                "&&" => {
                    token = Some(Tok::And);
                    break;
                }
                "||" => {
                    token = Some(Tok::Or);
                    break;
                }
                _ => return self.check_token(token),
            }
            self.next_char();
        }
        self.next_char();
        self.check_token(token)
    }

    fn check_token(&self, token: Option<Tok>) -> Result<Tok, LexicalError> {
        if let Some(t) = token {
            Ok(t)
        } else {
            Err(LexicalError {
                error: LexicalErrorType::UnrecognizedToken(self.chr.unwrap()),
                location: self.location,
            })
        }
    }

    fn lex_number(&mut self, c: char) -> Result<Tok, LexicalError> {
        let mut text = String::new();
        text.push(c);
        loop {
            self.next_char();
            if let Some(c) = self.chr {
                match c {
                    '0'..='9' => text.push(c),
                    '_' => {
                        self.next_char();
                        if let Some(c) = self.chr {
                            if let '0'..='9' = c {
                                text.push(c);
                            } else {
                                return Err(LexicalError {
                                    error: LexicalErrorType::UnrecognizedToken(c),
                                    location: self.location,
                                });
                            }
                        } else {
                            return Err(LexicalError {
                                error: LexicalErrorType::UnrecognizedToken(c),
                                location: self.location,
                            });
                        }
                    }
                    _ => break,
                }
            } else {
                break;
            }
        }
        Ok(Tok::Num {
            number: u64::from_str(&text)?,
        })
    }

    fn lex_literal(&mut self, c: char) -> Result<Tok, LexicalError> {
        let mut text = String::new();
        let first = c;
        loop {
            self.next_char();
            if let Some(c) = self.chr {
                if first == c {
                    break;
                }
                text.push(c);
            } else {
                break;
            }
        }
        Ok(Tok::Literal { literal: text })
    }
}

impl<T> Iterator for Lexer<T>
    where
        T: Iterator<Item=char>,
{
    type Item = LexResult;
    fn next(&mut self) -> Option<Self::Item> {
        let token = self.next_spanned();
        match token {
            Ok((_, Tok::EOF, _)) => None,
            r => Some(r),
        }
    }
}
