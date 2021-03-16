use std::vec::Vec;
use std::str::Chars;
use std::iter::Peekable;
use std::result::Result;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use std::rc::Rc;
use crate::compiler::ast::BramaAstType;

pub type ParseResult        = Result<(), BramaError>;
pub type AstResult          = Result<BramaAstType, BramaError>;
pub type CompilerResult     = Result<(), &'static str>;

pub const TAG_NULL        : u64 = 0;
pub const TAG_FALSE       : u64 = 1;
pub const TAG_TRUE        : u64 = 2;

pub const QNAN:         u64 = 0x7ffc_0000_0000_0000;
pub const POINTER_FLAG: u64 = 0x8000_0000_0000_0000;
pub const POINTER_MASK: u64 = 0x0000_FFFF_FFFF_FFFF;
pub const FALSE_FLAG:   u64 = QNAN | TAG_FALSE;
pub const TRUE_FLAG:    u64 = QNAN | TAG_TRUE;
pub const EMPTY_FLAG:   u64 = QNAN | TAG_NULL;

#[derive(PartialEq, Debug, Hash, Clone, Copy)]
#[repr(transparent)]
pub struct VmObject(pub u64);


#[derive(Copy)]
#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
pub enum BramaError {
    SyntaxError = 100,
    InvalidExpression,
    MoreThan1ArgumentPassed,
    RightParanthesesMissing,
    AssertFailed,
    NumberNotParsed,
    MissingStringDeliminator,
    CharNotValid,
    RightSideOfExpressionNotFound,
    ReturnMustBeUsedInFunction,
    FunctionCallSyntaxNotValid,
    FunctionNameNotDefined,
    ArgumentMustBeText,
    IfConditionBodyNotFound,
    ParenthesesNotClosed,
    InvalidUnaryOperation,
    UnaryWorksWithNumber,
    ArgumentNotFound,
    MultipleElseUsageNotValid,
    BreakAndContinueBelongToLoops,
    FunctionConditionBodyNotFound,
    ColonMarkMissing,
    ElseIsUsed,
    IndentationIssue,
    DictNotClosed,
    ArrayNotClosed,
    InvalidListItem,
    DictionaryKeyNotValid,
    DictionaryValueNotValid,
    CommentNotFinished,
    WhileStatementNotValid
}


impl BramaError {
    pub fn as_text(&self) -> String {
        let message = match self {
            BramaError::SyntaxError => "Sozdizimi hatasi",
            BramaError::MoreThan1ArgumentPassed => "Birden fazla degisken kullanilamaz",
            BramaError::RightParanthesesMissing => "Sağ parantaz eksik",
            BramaError::AssertFailed => "Doğrulanamadı",
            BramaError::NumberNotParsed => "Sayı ayrıştırılamadı",
            BramaError::MissingStringDeliminator => "Yazı sonlandırıcısı bulunamadı",
            BramaError::CharNotValid => "Karakter geçerli değil",
            BramaError::RightSideOfExpressionNotFound => "İfadenin sağ tarafı bulunamadı",
            BramaError::ReturnMustBeUsedInFunction => "Döndür komutu fonksiyon içinde kullanılmalıdır",
            BramaError::FunctionCallSyntaxNotValid => "Fonksiyon çağırma sözdizimi geçerli değil",
            BramaError::FunctionNameNotDefined => "Fonksiyon adı tanımlanmamış",
            BramaError::ArgumentMustBeText => "Değişken yazı olmalıdır",
            BramaError::IfConditionBodyNotFound => "Koşul gövdesi eksik",
            BramaError::ParenthesesNotClosed => "Parantez kapatılmamış",
            BramaError::InvalidUnaryOperation => "Geçersiz tekli işlem",
            BramaError::UnaryWorksWithNumber => "Tekli numara ile çalışmaktadır",
            BramaError::InvalidExpression => "Geçersiz ifade",
            BramaError::ArgumentNotFound => "Parametre bulunamadı",
            BramaError::MultipleElseUsageNotValid => "Birden fazla yada ifadesi kullanılamaz",
            BramaError::BreakAndContinueBelongToLoops => "'kır' ve 'devamet' ifadeleri döngü içinde kullanılabilir",
            BramaError::FunctionConditionBodyNotFound => "Fonksiyon içi kodlar bulunamadı",
            BramaError::ColonMarkMissing => "':' eksik",
            BramaError::ElseIsUsed => "'yada' zaten kullanıldı",
            BramaError::IndentationIssue => "Girinti sorunu",
            BramaError::DictNotClosed => "Sözlük düzgün kapatılmamış",
            BramaError::ArrayNotClosed => "Dizi düzgün kapatılmadı",
            BramaError::InvalidListItem => "Dizi elemanı geçersiz",
            BramaError::DictionaryKeyNotValid => "Sözlük anahtarı geçersiz",
            BramaError::DictionaryValueNotValid => "Sözlük geçeri geçersiz",
            BramaError::CommentNotFinished => "Yorum bilgisi düzgün kapatılmadı",
            BramaError::WhileStatementNotValid => "Döngü düzgün tanımlanmamış"
        };
        format!("(#{}) {}", *self as u8, message)
    }
}

#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(PartialEq, Eq)]
pub enum BramaKeywordType {
    None=0,
    True,
    False,
    Use,
    Until,
    Loop,
    If,
    Else,
    And,
    Or,
    Empty,
    Modulo,
    Not,
    GreaterThan,
    LessThan,
    GreaterEqualThan,
    LessEqualThan,
    Equal,
    NotEqual,
    Fn,
    Return,
    Endless,
    Break,
    Continue,
    WhileStartPart,
    WhileEndPart
}

impl BramaKeywordType {
    pub fn to_operator(&self) -> BramaOperatorType {
        match &self {
            BramaKeywordType::And              => BramaOperatorType::And,
            BramaKeywordType::Or               => BramaOperatorType::Or,
            BramaKeywordType::Modulo           => BramaOperatorType::Modulo,
            BramaKeywordType::Not              => BramaOperatorType::Not,
            BramaKeywordType::Equal            => BramaOperatorType::Equal,
            BramaKeywordType::NotEqual         => BramaOperatorType::NotEqual,
            BramaKeywordType::GreaterThan      => BramaOperatorType::GreaterThan,
            BramaKeywordType::GreaterEqualThan => BramaOperatorType::GreaterEqualThan,
            BramaKeywordType::LessThan         => BramaOperatorType::LessThan,
            BramaKeywordType::LessEqualThan    => BramaOperatorType::LessEqualThan,
            _                                  => BramaOperatorType::None
        }
    }
}

pub static KEYWORDS: &[(&str, BramaKeywordType)] = &[
    ("true",   BramaKeywordType::True),
    ("false",  BramaKeywordType::False),
    ("use",    BramaKeywordType::Use),
    ("until",  BramaKeywordType::Until),
    ("loop",   BramaKeywordType::Loop),
    ("if",     BramaKeywordType::If),
    ("else",   BramaKeywordType::Else),
    ("and",    BramaKeywordType::And),
    ("or",     BramaKeywordType::Or),
    ("empty",  BramaKeywordType::Empty),
    ("not",    BramaKeywordType::Not),
    ("equal",         BramaKeywordType::Equal),
    ("notequal",      BramaKeywordType::NotEqual),
    ("greater",       BramaKeywordType::GreaterThan),
    ("greaterequal",  BramaKeywordType::GreaterEqualThan),
    ("less",          BramaKeywordType::LessThan),
    ("lessequal",     BramaKeywordType::LessEqualThan),
    ("return",        BramaKeywordType::Return),
    ("endless",       BramaKeywordType::Endless),
    ("break",         BramaKeywordType::Break),
    ("continue",      BramaKeywordType::Continue),
    ("do",            BramaKeywordType::WhileStartPart),
    ("while",         BramaKeywordType::WhileEndPart),

    ("doğru",  BramaKeywordType::True),
    ("yanlış", BramaKeywordType::False),
    ("kullan", BramaKeywordType::Use),
    ("kadar",  BramaKeywordType::Until),
    ("döngü",  BramaKeywordType::Loop),
    ("sonsuz", BramaKeywordType::Endless),
    ("eğer",   BramaKeywordType::If),
    ("yada",   BramaKeywordType::Else),
    ("ve",     BramaKeywordType::And),
    ("veya",   BramaKeywordType::Or),
    ("yok",    BramaKeywordType::Empty),
    ("mod",    BramaKeywordType::Modulo),
    ("eşittir",       BramaKeywordType::Equal),
    ("eşitdeğildir",  BramaKeywordType::NotEqual),
    ("büyüktür",      BramaKeywordType::GreaterThan),
    ("büyükeşittir",  BramaKeywordType::GreaterEqualThan),
    ("küçüktür",      BramaKeywordType::LessThan),
    ("küçükeşittir",  BramaKeywordType::LessEqualThan),
    ("değil",         BramaKeywordType::Not),
    ("fn",            BramaKeywordType::Fn),
    ("döndür",        BramaKeywordType::Return),
    ("kır",           BramaKeywordType::Break),
    ("devamet",       BramaKeywordType::Continue),
    ("döngü",         BramaKeywordType::WhileStartPart),
    ("iken",          BramaKeywordType::WhileEndPart)
];

#[derive(Clone, Copy)]
#[derive(Debug)]
#[derive(PartialEq)]
pub enum BramaOperatorType {
    None,
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Modulo,
    Increment,
    Deccrement,
    Assign,
    AssignAddition,
    AssignSubtraction,
    AssignMultiplication,
    AssignDivision,
    Equal,
    NotEqual,
    Not,
    And,
    Or,
    GreaterThan,
    LessThan,
    GreaterEqualThan,
    LessEqualThan,
    QuestionMark,
    ColonMark,
    LeftParentheses,
    RightParentheses,
    SquareBracketStart,
    SquareBracketEnd,
    Comma,
    Semicolon,
    Dot,
    CommentLine,
    CommentMultilineStart,
    CommentMultilineEnd,
    CurveBracketStart,
    CurveBracketEnd
}

#[repr(C)]
#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
pub enum BramaTokenType {
    Integer(i64),
    Double(f64),
    Symbol(Rc<String>),
    Operator(BramaOperatorType),
    Text(Rc<String>),
    Keyword(BramaKeywordType),
    WhiteSpace(u8),
    NewLine(u8)
}

#[repr(C)]
#[derive(Clone)]
#[derive(Debug)]
#[derive(PartialEq)]
pub enum BramaNumberSystem {
    Binary      = 0,
    Octal       = 1,
    Decimal     = 2,
    Hexadecimal = 3
}

#[derive(Debug, Clone)]
pub struct Token {
    pub line      : u32,
    pub start    : u32,
    pub end    : u32,
    pub token_type: BramaTokenType
}

pub struct Tokinizer<'a> {
    pub line  : u32,
    pub column: u32,
    pub tokens: Vec<Token>,
    pub iter: Peekable<Chars<'a>>,
    pub iter_second: Peekable<Chars<'a>>,
    pub iter_third: Peekable<Chars<'a>>,
    pub data: String,
    pub index: u32
}

impl Tokinizer<'_> {
    pub fn is_end(&mut self) -> bool {
        return match self.iter.peek() {
            Some(_) => false,
            None => true
        };
    }

    pub fn get_char(&mut self) -> char {
        return match self.iter.peek() {
            Some(&c) => c,
            None => '\0'
        };
    }

    pub fn get_next_char(&mut self) -> char {
        return match self.iter_second.peek() {
            Some(&c) => c,
            None => '\0'
        };
    }

    pub fn add_token(&mut self, start: u32, token_type: BramaTokenType) {
        let token = Token {
            line: self.line,
            start,
            end: self.column,
            token_type
        };
        self.tokens.push(token);
    }

    pub fn increase_index(&mut self) {
        self.index  += self.get_char().len_utf8() as u32;
        self.column += 1;
        self.iter.next();
        self.iter_second.next();
        self.iter_third.next();
    }

    pub fn increate_line(& mut self) {
        self.line += 1;
    }

    pub fn reset_column(& mut self) {
        self.column = 0;
    }
}

pub trait TokenParser {
    fn check(&self, tokinizer: &mut Tokinizer) -> bool;
    fn parse(&self, tokinizer: &mut Tokinizer) -> Result<(), BramaError>;
}

pub trait CharTraits {
    fn is_new_line(&self) -> bool;
    fn is_whitespace(&self) -> bool;
    fn is_symbol(&self) -> bool;
    fn is_integer(&self) -> bool;
}

impl CharTraits for char {
    fn is_new_line(&self) -> bool {
        *self == '\n'
    }

    fn is_whitespace(&self) -> bool {
        match *self {
            ' ' | '\r' | '\t' => true,
            _ => false
        }
    }

    fn is_symbol(&self) -> bool {
        self.is_alphabetic() || *self == '_' ||  *self == '$'
    }

    fn is_integer(&self) -> bool {
        match *self {
            '0'..='9' => true,
            _ => false,
        }
    }
}

impl BramaTokenType {

    pub fn is_symbol(&self) -> bool {
        match self {
            BramaTokenType::Symbol(_) => true,
            _ => false
        }
    }

    #[allow(dead_code)]
    pub fn is_keyword(&self) -> bool {
        match self {
            BramaTokenType::Keyword(_) => true,
            _ => false
        }
    }

    pub fn get_symbol(&self) -> String {
        match self {
            BramaTokenType::Symbol(string) => string.to_string(),
            _ => String::from("")
        }
    }

    pub fn get_keyword(&self) -> BramaKeywordType {
        match self {
            BramaTokenType::Keyword(keyword) => *keyword,
            _ => BramaKeywordType::None
        }
    }
}

pub trait StrTrait {
    fn atom(&self) -> u64;
}

impl StrTrait for str {
    fn atom(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }
}