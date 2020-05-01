/*
 * ******************************************************************************************
 * Copyright (c) 2019 Pascal Kuthe. This file is part of the VARF project.
 * It is subject to the license terms in the LICENSE file found in the top-level directory
 *  of this distribution and at  https://gitlab.com/DSPOM/VARF/blob/master/LICENSE.
 *  No part of VARF, including this file, may be copied, modified, propagated, or
 *  distributed except according to the terms contained in the LICENSE file.
 * *****************************************************************************************
 */

use logos::internal::LexerInternal;
use logos::Logos;

use crate::span::{Index, LineNumber, Range};
#[derive(Clone, Debug, PartialEq, Copy, Eq)]
pub struct FollowedByBracket(pub bool);

//in terms of api this just serves as a lexer token enum. however it actually is the real lexer generated by logos.
#[derive(Clone, Logos, Debug, PartialEq, Copy, Eq)]
pub enum Token {
    //Newline handling
    #[token("\\\n")]
    MacroDefNewLine,

    #[token("\n")]
    #[regex(r"//[^\n]*\n")]
    Newline,

    #[token("/*", ignore_multiline_comment)]
    Comment(LineNumber),

    //Mock tokens only used for error reporting
    CommentEnd,
    EOF,

    //Actual Tokens

    //required rules
    #[regex(r"[ \t\f]+", logos::skip)]
    #[error]
    Unexpected,

    UnexpectedEOF,

    #[regex(r"`[a-zA-Z_][a-zA-Z_0-9\$]*")]
    MacroReference,
    //Compiler directives
    #[token("`include")]
    Include,
    #[token("`ifdef")]
    MacroIf,
    #[token("`ifndef")]
    MacroIfn,
    #[token("`elsif")]
    MacroElsif,
    #[token("`else")]
    MacroElse,
    #[token("`endif")]
    MacroEndIf,
    #[token("`define")]
    MacroDef,

    //Identifiers
    #[regex(r"[a-zA-Z_][[:word:]\$]*", handle_simple_ident)]
    SimpleIdentifier(FollowedByBracket),
    #[regex(r"\\[[:print:]&&\S]+\s")]
    EscapedIdentifier,
    #[regex(r"\$[a-zA-Z0-9_\$][a-zA-Z0-9_\$]*")]
    SystemCall,

    //Constants
    #[regex(r#""([^\n"\\]|\\[\\tn")])*""#)]
    LiteralString,

    #[regex(r"[0-9][0-9_]*")]
    LiteralUnsignedNumber,
    #[regex(r"[0-9][0-9_]*\.[0-9][0-9_]*[TGMKkmupfa]")]
    LiteralRealNumberDotScaleChar,
    #[regex(r"[0-9][0-9_]*\.[0-9][0-9_]*[eE][+-]?[0-9][0-9_]*")]
    LiteralRealNumberDotExp,
    #[regex(r"[0-9][0-9_]*[TGMKkmupfa]")]
    LiteralRealNumberScaleChar,
    #[regex(r"[0-9][0-9_]*[eE][+-]?[0-9][0-9_]*")]
    LiteralRealNumberExp,
    #[regex(r"[0-9][0-9_]*\.[0-9][0-9_]*")]
    LiteralRealNumberDot,

    //Symbols
    #[token(".")]
    Accessor,
    #[token(";")]
    Semicolon,
    #[token(":")]
    Colon,
    #[token(",")]
    Comma,
    #[token("(")]
    ParenOpen,
    #[token(")")]
    ParenClose,
    #[token("(*")]
    AttributeStart,
    #[token("*)")]
    AttributeEnd,
    #[token("[")]
    SquareBracketOpen,
    #[token("]")]
    SquareBracketClose,
    #[token("<+")]
    Contribute,
    #[token("=")]
    Assign,
    #[token("#")]
    Hash,

    //Arithmatic Operators
    #[token("*")]
    OpMul,
    #[token("/")]
    OpDiv,
    #[token("%")]
    OpModulus,
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("**")]
    OpExp,
    //UnaryOperators
    #[token("!")]
    OpLogicNot,
    #[token("~")]
    OpBitNot,

    #[token("<<")]
    OpArithmeticShiftLeft,
    #[token(">>")]
    OpArithmeticShiftRight,

    //Relational
    #[token("<")]
    OpLess,
    #[token("<=")]
    OpLessEqual,
    #[token(">")]
    OpGreater,
    #[token(">=")]
    OpGreaterEqual,
    #[token("==")]
    OpEqual,
    #[token("!=")]
    OpNotEqual,
    //Logic
    #[token("&&")]
    OpLogicAnd,
    #[token("||")]
    OpLogicalOr,

    //Bit
    #[token("&")]
    OpBitAnd,
    #[token("^")]
    OpBitXor,
    #[token("~^")]
    #[token("^~")]
    OpBitNXor,
    #[token("|")]
    OpBitOr,

    //Other
    #[token("?")]
    OpCondition,

    //Keywords
    #[token("if")]
    If,
    #[token("else")]
    Else,

    #[token("while")]
    While,

    #[token("begin")]
    Begin,
    #[token("end")]
    End,

    #[token("module")]
    Module,
    #[token("endmodule")]
    EndModule,
    #[token("discipline")]
    Discipline,
    #[token("enddiscipline")]
    EndDiscipline,

    #[token("nature")]
    Nature,
    #[token("endnature")]
    EndNature,

    #[token("branch")]
    Branch,
    #[token("parameter")]
    Parameter,
    #[token("localparam")]
    DefineParameter,
    #[token("defparam")]
    LocalParameter,

    #[token("analog")]
    Analog,
    #[token("initial")]
    AnalogInitial,

    #[token("input")]
    Input,
    #[token("inout")]
    Inout,
    #[token("output")]
    Output,

    #[token("signed")]
    Signed,
    #[token("vectored")]
    Vectored,
    #[token("scalared")]
    Scalared,

    //Types
    #[token("string")]
    String,
    #[token("time")]
    Time,
    #[token("realtime")]
    Realtime,
    #[token("integer")]
    Integer,
    #[token("real")]
    Real,
    #[token("reg")]
    Reg,
    #[token("wreal")]
    Wreal,
    #[token("supply0")]
    Supply0,
    #[token("supply1")]
    Supply1,
    #[token("tri")]
    Tri,
    #[token("triand")]
    TriAnd,
    #[token("trior")]
    TriOr,
    #[token("tri0")]
    Tri0,
    #[token("tri1")]
    Tri1,
    #[token("wire")]
    Wire,
    #[token("uwire")]
    Uwire,
    #[token("wand")]
    Wand,
    #[token("wor")]
    Wor,
    #[token("ground")]
    Ground,

    #[token("potential")]
    Potential,
    #[token("flow")]
    Flow,
    #[token("domain")]
    Domain,
    #[token("discrete")]
    Discrete,
    #[token("continuous")]
    Continuous,

    #[token("ddt")]
    TimeDerivative,
    #[token("ddx")]
    PartialDerivative,
    #[token("idt")]
    TimeIntegral,
    #[token("idtmod")]
    TimeIntegralMod,
    #[token("limexp")]
    LimExp,
    #[token("white_noise")]
    WhiteNoise,
    #[token("flicker_noise")]
    FlickerNoise,

    #[token("pow")]
    Pow,
    #[token("sqrt")]
    Sqrt,

    #[token("hypot")]
    Hypot,
    #[token("exp")]
    Exp,
    #[token("ln")]
    Ln,
    #[token("log")]
    Log,
    #[token("min")]
    Min,
    #[token("max")]
    Max,
    #[token("abs")]
    Abs,
    #[token("floor")]
    Floor,
    #[token("ceil")]
    Ceil,

    #[token("sin")]
    Sin,
    #[token("cos")]
    Cos,
    #[token("tan")]
    Tan,

    #[token("asin")]
    ArcSin,
    #[token("acos")]
    ArcCos,
    #[token("atan")]
    ArcTan,
    #[token("atan2")]
    ArcTan2,

    #[token("sinh")]
    SinH,
    #[token("cosh")]
    CosH,
    #[token("tanh")]
    TanH,

    #[token("asinh")]
    ArcSinH,
    #[token("acosh")]
    ArcCosH,
    #[token("atanh")]
    ArcTanH,

    #[token("from")]
    From,
    #[token("exclude")]
    Exclude,
    #[token("inf")]
    Infinity,
    #[token("-inf")]
    MinusInfinity,

    #[token("abstol")]
    Abstol,
    #[token("access")]
    Access,
    #[token("ddt_nature")]
    TimeDerivativeNature,
    #[token("idt_nature")]
    TimeIntegralNature,
    #[token("units")]
    Units,
}

#[inline]
fn ignore_multiline_comment<'source>(lex: &mut logos::Lexer<'source, Token>) -> Option<LineNumber> {
    let mut lines: LineNumber = 0;
    loop {
        match lex.read()? {
            b'*' => {
                lex.bump(1);
                if lex.read() == Some(b'/') {
                    lex.bump(1);
                    break;
                }
            }
            b'\n' => {
                lines += 1;
                lex.bump(1)
            }
            _ => lex.bump(1),
        }
    }
    Some(lines)
}
#[inline]
fn handle_simple_ident<'source>(lex: &mut logos::Lexer<'source, Token>) -> FollowedByBracket {
    FollowedByBracket(lex.read() == Some(b'('))
}

pub struct Lexer<'lt> {
    internal: logos::Lexer<'lt, Token>,
}
impl<'lt> Lexer<'lt> {
    pub fn new(source: &'lt str) -> Self {
        Self {
            internal: Token::lexer(source),
        }
    }

    #[cfg(test)]
    pub fn new_test(source: &'lt str) -> Self {
        let mut res = Self {
            internal: Token::lexer(source),
        };
        res
    }

    pub fn peek(&self) -> (Range, Option<Token>) {
        let mut lexer = self.internal.clone();
        let token = lexer.next();
        let range = lexer.span();
        let range = Range {
            start: range.start as Index,
            end: range.end as Index,
        };
        (range, token)
    }
    #[cfg(test)]
    pub fn test_next(&mut self) -> Option<Token> {
        loop {
            match self.internal.next() {
                Some(Token::Newline) | Some(Token::Comment(_)) => (),
                res => return res,
            }
        }
    }

    pub fn range(&self) -> Range {
        let internal_range = self.internal.span();
        Range {
            start: internal_range.start as Index,
            end: internal_range.end as Index,
        }
    }
    pub fn token_len(&self) -> Index {
        self.range().end - self.range().start
    }

    pub fn slice(&self) -> &str {
        self.internal.slice()
    }
}
impl<'source> Iterator for Lexer<'source> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.internal.next()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn macro_if() {
        let mut lexer = Lexer::new("`ifdef");
        assert_eq!(lexer.next(), Some(Token::MacroIf));
    }
    #[test]
    pub fn macro_ifn() {
        let mut lexer = Lexer::new("`ifndef");
        assert_eq!(lexer.next(), Some(Token::MacroIfn));
    }
    #[test]
    pub fn macro_else() {
        let mut lexer = Lexer::new("`else");
        assert_eq!(lexer.next(), Some(Token::MacroElse));
    }
    #[test]
    pub fn macro_elsif() {
        let mut lexer = Lexer::new("`elsif");
        assert_eq!(lexer.next(), Some(Token::MacroElsif));
    }
    #[test]
    pub fn macro_definition() {
        let mut lexer = Lexer::new("`define x(y) \\\n test");
        assert_eq!(lexer.test_next(), Some(Token::MacroDef));
        assert_eq!(
            lexer.test_next(),
            Some(Token::SimpleIdentifier(FollowedByBracket(true)))
        );
        assert_eq!(lexer.test_next(), Some(Token::ParenOpen));
        assert_eq!(
            lexer.test_next(),
            Some(Token::SimpleIdentifier(FollowedByBracket(false)))
        );
        assert_eq!(lexer.test_next(), Some(Token::ParenClose));
        assert_eq!(lexer.test_next(), Some(Token::MacroDefNewLine));
        assert_eq!(
            lexer.test_next(),
            Some(Token::SimpleIdentifier(FollowedByBracket(false)))
        );
    }
    #[test]
    pub fn include() {
        assert_eq!(Lexer::new("`include").next(), Some(Token::Include));
    }
    #[test]
    pub fn simple_ident() {
        let mut lexer = Lexer::new_test("test _test  egta  test$\ntest2_$ iftest");
        assert_eq!(
            lexer.test_next(),
            Some(Token::SimpleIdentifier(FollowedByBracket(false)))
        );
        assert_eq!(lexer.slice(), "test");
        assert_eq!(
            lexer.test_next(),
            Some(Token::SimpleIdentifier(FollowedByBracket(false)))
        );
        assert_eq!(lexer.slice(), "_test");
        assert_eq!(
            lexer.test_next(),
            Some(Token::SimpleIdentifier(FollowedByBracket(false)))
        );
        assert_eq!(lexer.slice(), "egta");
        assert_eq!(
            lexer.test_next(),
            Some(Token::SimpleIdentifier(FollowedByBracket(false)))
        );
        assert_eq!(lexer.slice(), "test$");
        assert_eq!(
            lexer.test_next(),
            Some(Token::SimpleIdentifier(FollowedByBracket(false)))
        );
        assert_eq!(lexer.slice(), "test2_$");
        assert_eq!(
            lexer.test_next(),
            Some(Token::SimpleIdentifier(FollowedByBracket(false)))
        );
        assert_eq!(lexer.slice(), "iftest");
    }
    #[test]
    pub fn escaped_ident() {
        let mut lexer = Lexer::new("\\lel\\\\lel \\if ");
        assert_eq!(lexer.test_next(), Some(Token::EscapedIdentifier));
        assert_eq!(&lexer.slice()[1..9], "lel\\\\lel");
        assert_eq!(lexer.test_next(), Some(Token::EscapedIdentifier));
        assert_eq!(&lexer.slice()[1..3], "if");
    }
    #[test]
    pub fn comment() {
        let mut lexer = Lexer::new_test("//jdfjdfjw4$%\r%&/**#.,|\ntest");
        assert_eq!(
            lexer.test_next(),
            Some(Token::SimpleIdentifier(FollowedByBracket(false)))
        );
        assert_eq!(lexer.slice(), "test")
    }
    #[test]
    pub fn block_comment() {
        let mut lexer = Lexer::new_test("/*A\nB\n*C*/`test");
        assert_eq!(lexer.test_next(), Some(Token::MacroReference));
        assert_eq!(lexer.slice(), "`test")
    }
    #[test]
    pub fn string() {
        let mut lexer = Lexer::new(r#""lel\"dsd%§.,-032391\t    ""#);
        assert_eq!(lexer.test_next(), Some(Token::LiteralString));
    }
    #[test]
    pub fn unsigned_number() {
        let mut lexer = Lexer::new("1_2345_5678_9");
        assert_eq!(lexer.test_next(), Some(Token::LiteralUnsignedNumber));
    }
    #[test]
    pub fn macro_ref() {
        let test = "`egta";

        let mut lexer = Lexer::new_test(test);
        assert_eq!(lexer.test_next(), Some(Token::MacroReference))
    }
    #[test]
    pub fn real_number() {
        let mut lexer = Lexer::new_test(
            "1.2
            0.1
            2394.26331
            1.2E12 // the exponent symbol can be e or E
            1.30e-2
            0.1e-0
            236.123_763_e-12 // underscores are ignored
            1.3u
            23E10
            29E-2
            7k",
        );
        assert_eq!(lexer.test_next(), Some(Token::LiteralRealNumberDot));
        assert_eq!(lexer.test_next(), Some(Token::LiteralRealNumberDot));
        assert_eq!(lexer.test_next(), Some(Token::LiteralRealNumberDot));
        assert_eq!(lexer.test_next(), Some(Token::LiteralRealNumberDotExp));
        assert_eq!(lexer.test_next(), Some(Token::LiteralRealNumberDotExp));
        assert_eq!(lexer.test_next(), Some(Token::LiteralRealNumberDotExp));
        assert_eq!(lexer.test_next(), Some(Token::LiteralRealNumberDotExp));
        assert_eq!(
            lexer.test_next(),
            Some(Token::LiteralRealNumberDotScaleChar)
        );
        assert_eq!(lexer.test_next(), Some(Token::LiteralRealNumberExp));
        assert_eq!(lexer.test_next(), Some(Token::LiteralRealNumberExp));
        assert_eq!(lexer.test_next(), Some(Token::LiteralRealNumberScaleChar));
    }
}
