use core::fmt;

use logos::Logos;
#[derive(Logos, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[logos(skip r"[ \t\n\f\r]+")]
#[repr(u8)]
pub enum Token {
	// Keywords
	#[token("fn")]
	Fn,
	#[token("let")]
	Let,
	#[token("const")]
	Const,
	#[token("mut")]
	Mut,
	#[token("if")]
	If,
	#[token("for")]
	For,
	#[token("in")]
	In,
	#[token("while")]
	While,
	#[token("loop")]
	Loop,
	#[token("break")]
	Break,
	#[token("skip")]
	Skip,
	#[token("else")]
	Else,
	#[token("return")]
	Ret,
	#[token("null")]
	Null,
	#[token("match")]
	Match,
	#[token("import")]
	Import,
	#[token("pub")]
	Pub,
	#[token("mod")]
	Mod,

	// Operators
	#[token("+")]
	Plus,
	#[token("-")]
	Minus,
	#[token("*")]
	Star,
	#[token("/")]
	Slash,
	#[token("=")]
	Assign,
	#[token("^")]
	Pow,
	#[token("^=")]
	PowEq,
	#[token("%")]
	Rem,
	#[token("%=")]
	RemEq,
	#[token("+=")]
	PlusEq,
	#[token("-=")]
	MinusEq,
	#[token("*=")]
	StarEq,
	#[token("/=")]
	SlashEq,
	#[token("==")]
	Eq,
	#[token("!=")]
	NotEq,
	#[token("<")]
	Less,
	#[token(">")]
	Greater,
	#[token("<=")]
	LessEq,
	#[token(">=")]
	GreaterEq,
	#[token("?=")]
	Extract,
	#[token("->")]
	Arrow,
	#[token("&")]
	And,
	#[token("&&")]
	AndAnd,
	#[token(".")]
	Dot,
	#[token("..")]
	DotDot,
	#[token("..=")]
	DotDotEq,
	#[token("!")]
	Bang,
	#[token("?")]
	Quest,
	#[token(":")]
	Colon,
	#[token("::")]
	ColonColon,
	#[token("|>")]
	Pipe,
	#[token("|")]
	Bar,
	#[token("||")]
	BarBar,
	#[token("~")]
	Tilde,
	#[token("@")]
	At,

	// Delimiters
	#[token("(")]
	LParen,
	#[token(")")]
	RParen,
	#[token("{")]
	LBrace,
	#[token("}")]
	RBrace,
	#[token("[")]
	LBracket,
	#[token("]")]
	RBracket,
	#[token(";")]
	Semi,
	#[token(",")]
	Comma,
	// Identifiers
	#[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", priority = 1)]
	Ident,
	// Literals
	#[regex(r#""([^"\\]|\\.)*""#)]
	String,
	// char
	#[regex(r#"'([^'\\]|\\.)*'"#)]
	Char,
	#[regex(r"0x[0-9A-Fa-f](_?[0-9A-Fa-f])*")]
	Hex,
	#[regex(r"0b[01]+(_[01]+)*")]
	Bin,
	#[regex(r"[0-9]+(_[0-9]+)*(\.[0-9]+(_[0-9]+)*)?([eE][+-]?[0-9]+)?")]
	Decimal,
	#[token("true")]
	True,
	#[token("false")]
	False,

	// types
	//
	//
	//
	//
	//
	#[token("usize")]
	UsizeType,
	#[token("isize")]
	IsizeType,
	#[token("bool")]
	BoolType,
	#[token("i8")]
	I8Type,
	#[token("u8")]
	U8Type,
	#[token("i16")]
	I16Type,
	#[token("u16")]
	U16Type,
	#[token("i32")]
	I32Type,
	#[token("u32")]
	U32Type,
	#[token("i64")]
	I64Type,
	#[token("u64")]
	U64Type,
	#[token("f32")]
	F32Type,
	#[token("f64")]
	F64Type,
	#[token("char")]
	CharType,
	#[token("string")]
	StringType,
	// Comments //
	#[regex(r"//[^\n]*", logos::skip)]
	SkipLine,
	// block comment /*...*/
	#[regex(r"/\*[^*]*\*+(?:[^/*][^*]*\*+)*/", logos::skip)]
	SkipBlock,
}

// display
impl fmt::Display for Token {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Token::Fn => write!(f, "fn"),
			Token::Let => write!(f, "let"),
			Token::Mut => write!(f, "mut"),
			Token::If => write!(f, "if"),
			Token::For => write!(f, "for"),
			Token::In => write!(f, "in"),
			Token::While => write!(f, "while"),
			Token::Loop => write!(f, "loop"),
			Token::Break => write!(f, "break"),
			Token::Skip => write!(f, "skip"),
			Token::Else => write!(f, "else"),
			Token::Ret => write!(f, "return"),
			Token::Null => write!(f, "null"),
			Token::Match => write!(f, "match"),
			Token::Import => write!(f, "import"),
			Token::Plus => write!(f, "+"),
			Token::Minus => write!(f, "-"),
			Token::Star => write!(f, "*"),
			Token::Slash => write!(f, "/"),
			Token::Assign => write!(f, "="),
			Token::Pow => write!(f, "^"),
			Token::PowEq => write!(f, "^="),
			Token::Rem => write!(f, "%"),
			Token::RemEq => write!(f, "%="),
			Token::PlusEq => write!(f, "+="),
			Token::MinusEq => write!(f, "-="),
			Token::StarEq => write!(f, "*="),
			Token::SlashEq => write!(f, "/="),
			Token::Eq => write!(f, "=="),
			Token::NotEq => write!(f, "!="),
			Token::Less => write!(f, "<"),
			Token::Greater => write!(f, ">"),
			Token::LessEq => write!(f, "<="),
			Token::GreaterEq => write!(f, ">="),
			Token::Extract => write!(f, "?="),
			Token::Arrow => write!(f, "=>"),
			Token::And => write!(f, "&"),
			Token::AndAnd => write!(f, "&&"),
			Token::BarBar => write!(f, "||"),
			Token::Dot => write!(f, "."),
			Token::DotDot => write!(f, ".."),
			Token::Bang => write!(f, "!"),
			Token::Quest => write!(f, "?"),
			Token::Colon => write!(f, ":"),
			Token::ColonColon => write!(f, "::"),
			Token::Pipe => write!(f, "|>"),
			Token::Bar => write!(f, "|"),
			Token::At => write!(f, "@"),
			Token::LParen => write!(f, "("),
			Token::RParen => write!(f, ")"),
			Token::LBrace => write!(f, "{{"),
			Token::RBrace => write!(f, "}}"),
			Token::LBracket => write!(f, "["),
			Token::RBracket => write!(f, "]"),
			Token::Semi => write!(f, ";"),
			Token::Comma => write!(f, ","),
			Token::Ident => write!(f, "ident"),
			Token::String => write!(f, "string"),
			Token::Char => write!(f, "char"),
			Token::Hex => write!(f, "hex"),
			Token::Bin => write!(f, "bin"),
			Token::Decimal => write!(f, "decimal"),
			Token::True => write!(f, "true"),
			Token::False => write!(f, "false"),
			Token::SkipLine => write!(f, "skip line"),
			Token::SkipBlock => write!(f, "skip block"),
			Token::UsizeType => write!(f, "usize"),
			Token::IsizeType => write!(f, "isize"),
			Token::BoolType => write!(f, "bool"),
			Token::I8Type => write!(f, "i8"),
			Token::U8Type => write!(f, "u8"),
			Token::I16Type => write!(f, "i16"),
			Token::U16Type => write!(f, "u16"),
			Token::I32Type => write!(f, "i32"),
			Token::U32Type => write!(f, "u32"),
			Token::I64Type => write!(f, "i64"),
			Token::U64Type => write!(f, "u64"),
			Token::F32Type => write!(f, "f32"),
			Token::F64Type => write!(f, "f64"),
			Token::CharType => write!(f, "char"),
			Token::StringType => write!(f, "string"),
			Token::Pub => write!(f, "pub"),
			Token::Mod => write!(f, "mod"),
			Token::DotDotEq => write!(f, "..="),
			Token::Tilde => write!(f, "~"),
			Token::Const => write!(f, "const"),
		}
	}
}
