use crate::error;
use error::PrintableError;
use num_bigint::{BigInt, BigUint};
use num_rational::BigRational;
use num_traits::identities::zero;
use std::convert::TryFrom;

pub struct Error<K> {
    column: u16,
    kind: K,
}

pub type LexError = Error<LexErrorKind>;

pub enum LexErrorKind {
    TooManyTabs,
    TooManyBytes,
    LeadingSpace,
}
use LexErrorKind::*;

pub fn lex_line(line: &str) -> Result<TokenStream, LexError> {
    Ok(TokenStream::try_from(indentation(line)?)?)
}

impl PrintableError for LexErrorKind {
    fn id(&self) -> error::ID {
        match self {
            TooManyTabs => 0,
            TooManyBytes => 1,
            LeadingSpace => 3,
        }
    }
    fn msg(&self) -> String {
        match self {
            TooManyTabs => String::from(
                "\
There are more than 255 tabs in front of this line.
It's likely that you should split up your logic into more functions.",
            ),
            TooManyBytes => String::from(
                "\
There are more than 2^16-1 bytes in this line.
Please split it up.",
            ),
            LeadingSpace => String::from(
                "You may not indent with spaces (yet), and putting spaces behind indentation gives you nicer formatting but can confuse the reader."
            )
        }
    }
}

#[derive(Clone, Copy)]
struct IndentedLine<'a> {
    tabs: u8, //If you have more than 255 tabs at the start of a line, there is something wrong with your code.
    text: &'a str,
}

fn indentation(line: &str) -> Result<IndentedLine, LexError> {
    let tab_count = line
        .bytes()
        .enumerate()
        .find_map(|(i, b)| if b != b'\t' { Some(i) } else { None })
        .unwrap_or(0);

    if line.as_bytes()[tab_count] == b' ' {
        return Err(LexError {
            column: tab_count as u16,
            kind: LeadingSpace,
        });
    }

    Ok(IndentedLine {
        tabs: match u8::try_from(tab_count) {
            Ok(count) => count,
            Err(_) => {
                return Err(Error {
                    column: 0,
                    kind: TooManyTabs,
                })
            }
        },
        text: &line[tab_count..],
    })
}

pub enum Token<'a> {
    TypeIdentifier(&'a str),      //Uppercase, then alphanumeric
    ValueIdentifier(&'a str),     //Lowercase, then alphanumeric
    StringLiteral(&'a str),       //C style
    IntegerLiteral(BigInt),       //also C style
    RationalLiteral(BigRational), //0.9, 7.3, 2.4e-15...
    Type,
    Const,
    Val,
    Var,
    Func,
    Overload,
    Has,
    With,
    OpeningRoundBrackets,
    ClosingRoundBrackets,
    AdditionSign,
    AdditionEqualSign,
    SubtractionSign,
    SubtractionEqualSign,
    MultiplicationSign,
    MultiplicationEqualSign,
    DivisionSign,
    DivisionEqualSign,
    EqualSign,
    Colon,
    Dot,
    Comma,
}

pub type TokenError = Error<TokenErrorKind>;

pub enum TokenErrorKind {
    Tab,
    UnknownChar(char),
}

use TokenErrorKind::*;

pub struct TokenStream<'a> {
    line: &'a str,
    position: u16,
}

impl<'a> TryFrom<IndentedLine<'a>> for TokenStream<'a> {
    type Error = LexError;
    fn try_from(line: IndentedLine<'a>) -> Result<Self, LexError> {
        //ensure that indices can be u16
        if let Err(_) = u16::try_from(line.text.len()) {
            return Err(Error {
                column: 0,
                kind: TooManyBytes,
            });
        };
        Ok(Self {
            line: line.text,
            position: line.tabs as u16,
        })
    }
}

impl<'a> TokenStream<'a> {
    fn error(&self, kind: TokenErrorKind) -> TokenError {
        TokenError {
            column: self.position,
            kind,
        }
    }
    fn advance(&mut self, n: usize) {
        self.position += n as u16;
        self.line = &self.line[n..];
    }
    fn parse_num(&mut self, radix: u8) -> Token<'a> {
        let mut result: BigUint = zero();
        for &digit in self
            .line
            .as_bytes()
            .iter()
            .take_while(|&&byte| byte > b'0' && byte < b'0' + radix)
        {
            result *= radix;
            result += digit;
        }
        Token::IntegerLiteral(result.into())
    }
    fn symbol_match(&mut self, symbol: &str) -> bool {
        if
        //special case for 1 byte
        (symbol.len() == 1 && self.line.as_bytes()[0] == symbol.as_bytes()[0])
            //general case
            || (self.line.len() >= symbol.len() && &self.line[..symbol.len()] == symbol)
        {
            self.advance(symbol.len());
            return true;
        } else {
            return false;
        }
    }
    fn some_next(&mut self) -> Result<Token<'a>, TokenError> {
        use Token::*;

        macro_rules! symbol_check {
            ($($literal:expr=>$token:expr,)*) =>
                {$( if self.symbol_match($literal) { return Ok($token); } )*};
        }
        symbol_check!(
            "("        => OpeningRoundBrackets,
            ")"        => ClosingRoundBrackets,
            "+"        => AdditionSign,
            "+="       => AdditionEqualSign,
            "-"        => SubtractionSign,
            "-="       => SubtractionEqualSign,
            "*"        => MultiplicationSign,
            "*="       => MultiplicationEqualSign,
            "/"        => DivisionSign,
            "/="       => DivisionEqualSign,
            "="        => EqualSign,
            ":"        => Colon,
            "."        => Dot,
            ","        => Comma,
        );

        let mut chars = self.line.chars();
        let first_char = chars.next().unwrap();
        //identifier
        if first_char.is_alphabetic() {
            let count = chars.take_while(|c| c.is_alphanumeric()).count() + 1; //We already got the first char.
            let result = &self.line[..count];
            self.advance(count);
            return Ok(if first_char.is_lowercase() {
                ValueIdentifier(match result {
                    "type" => return Ok(Type),
                    "val" => return Ok(Val),
                    "const" => return Ok(Const),
                    "var" => return Ok(Var),
                    "func" => return Ok(Func),
                    "overload" => return Ok(Overload),
                    "has" => return Ok(Has),
                    "with" => return Ok(With),
                    _ => result,
                })
            } else {
                TypeIdentifier(result)
            });
        }
        let radix = match first_char {
            '0' => {
                let second_char = chars.next().unwrap();
                match second_char {
                    'b' => {
                        //binary
                        self.advance(2);
                        2
                    }
                    'x' => {
                        //hexadecimal
                        self.advance(2);
                        16
                    }
                    _ if second_char.is_numeric() => {
                        //leading zero
                        self.advance(1);
                        10
                    }
                    _ => {
                        self.advance(1);
                        return Ok(IntegerLiteral(zero())); //decimal zero
                    }
                }
            }
            '1'..='9' => 10,
            '\t' => return Err(self.error(Tab)),
            c => return Err(self.error(UnknownChar(c))),
        };

        Ok(self.parse_num(radix))
    }
}

impl<'a> Iterator for TokenStream<'a> {
    type Item = Result<Token<'a>, TokenError>;
    fn next(&mut self) -> Option<Self::Item> {
        //skip whitespace between tokens
        self.advance(self.line.chars().take_while(|&c| c == ' ').count());

        //If we parsed the whole slice, it's empty.
        if self.line.is_empty() {
            return None;
        }
        Some(self.some_next())
    }
}
