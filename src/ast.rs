use std::{fmt::Debug, rc::Rc};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Var {
    pub text: String,
    pub location: Location,
}

impl From<rinha::parser::Var> for Var {
    fn from(value: rinha::parser::Var) -> Self {
        Self {
            text: value.text,
            location: value.location.into(),
        }
    }
}

/// File definition, it contains all the statements,
/// the module name, and a base location for it as anchor
/// for the statements.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct File {
    pub name: String,
    pub expression: Term,
    pub location: Location,
}

impl From<rinha::ast::File> for File {
    fn from(value: rinha::ast::File) -> Self {
        Self {
            name: value.name,
            expression: Term::from(value.expression),
            location: value.location.into(),
        }
    }
}

impl<T: Element> Element for Rc<T> {
    fn location(&self) -> &Location {
        self.as_ref().location()
    }
}

impl<T: Element> Element for Box<T> {
    fn location(&self) -> &Location {
        self.as_ref().location()
    }
}

#[derive(Default, Hash, PartialEq, Eq, Clone, serde::Serialize, serde::Deserialize)]
pub struct Location {
    pub start: usize,
    pub end: usize,
    pub filename: String,
}

impl From<rinha::ast::Location> for Location {
    fn from(value: rinha::ast::Location) -> Self {
        Self {
            start: value.start,
            end: value.end,
            filename: value.filename,
        }
    }
}

impl Location {
    /// Creates a new instance of [`Location`].
    pub fn new(start: usize, end: usize, filename: &str) -> Self {
        Self {
            start,
            end,
            filename: filename.into(),
        }
    }
}

impl Debug for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Location")
    }
}

impl From<Location> for miette::SourceSpan {
    fn from(value: Location) -> Self {
        Self::from(value.start..value.end)
    }
}

/// An element. It can be a declaration, or a term.
pub trait Element {
    fn location(&self) -> &Location;
}

/// Error node, it does contains an error.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Error {
    /// The error message.
    pub message: String,

    /// The original text that originated the error.
    pub full_text: String,

    /// The location of the error.
    pub location: Location,
}

impl From<rinha::ast::Error> for Error {
    fn from(value: rinha::ast::Error) -> Self {
        Self {
            message: value.message,
            full_text: value.full_text,
            location: value.location.into(),
        }
    }
}

impl Element for Error {
    fn location(&self) -> &Location {
        &self.location
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct If {
    pub condition: Box<Term>,
    pub then: Box<Term>,
    pub otherwise: Box<Term>,
    pub location: Location,
}

impl From<rinha::ast::If> for If {
    fn from(value: rinha::ast::If) -> Self {
        Self {
            condition: Box::new(Term::from(*value.condition)),
            then: Box::new(Term::from(*value.then)),
            otherwise: Box::new(Term::from(*value.otherwise)),
            location: value.location.into(),
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Let {
    pub name: Var,
    pub value: Box<Term>,
    pub next: Box<Term>,
    pub location: Location,
}

impl From<rinha::ast::Let> for Let {
    fn from(value: rinha::ast::Let) -> Self {
        Self {
            name: Var {
                text: value.name.text,
                location: value.name.location.into(),
            },
            value: Box::new(Term::from(*value.value)),
            next: Box::new(Term::from(*value.next)),
            location: value.location.into(),
        }
    }
}

/// Int is a integer value like `0`, `1`, `2`, etc.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Str {
    pub value: String,

    /// The location of the source in the source code.
    pub location: Location,
}

impl From<rinha::ast::Str> for Str {
    fn from(value: rinha::ast::Str) -> Self {
        Self {
            value: value.value,
            location: value.location.into(),
        }
    }
}

impl Element for Str {
    fn location(&self) -> &Location {
        &self.location
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Bool {
    pub value: bool,
    pub location: Location,
}

impl From<rinha::ast::Bool> for Bool {
    fn from(value: rinha::ast::Bool) -> Self {
        Self {
            value: value.value,
            location: value.location.into(),
        }
    }
}

impl Element for Bool {
    fn location(&self) -> &Location {
        &self.location
    }
}

/// Int is a integer value like `0`, `1`, `2`, etc.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Int {
    /// The value of the integer.
    pub value: i32,

    /// The location of the integer in the source code.
    pub location: Location,
}

impl Element for Int {
    fn location(&self) -> &Location {
        &self.location
    }
}

impl Default for Int {
    fn default() -> Self {
        Self {
            value: 0,
            location: Location::default(),
        }
    }
}

impl From<rinha::ast::Int> for Int {
    fn from(value: rinha::ast::Int) -> Self {
        Self {
            value: value.value,
            location: value.location.into(),
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum BinaryOp {
    Add, // Add
    Sub, // Subtract
    Mul, // Multiply
    Div, // Divide
    Rem, // Rem
    Eq,  // Equal
    Neq, // Not equal
    Lt,  // Less than
    Gt,  // Greater than
    Lte, // Less than or equal to
    Gte, // Greater than or equal to
    And, // And
    Or,  // Or
}

impl From<rinha::ast::BinaryOp> for BinaryOp {
    fn from(value: rinha::ast::BinaryOp) -> Self {
        match value {
            rinha::ast::BinaryOp::Add => Self::Add,
            rinha::ast::BinaryOp::Sub => Self::Sub,
            rinha::ast::BinaryOp::Mul => Self::Mul,
            rinha::ast::BinaryOp::Div => Self::Div,
            rinha::ast::BinaryOp::Rem => Self::Rem,
            rinha::ast::BinaryOp::Eq => Self::Eq,
            rinha::ast::BinaryOp::Neq => Self::Neq,
            rinha::ast::BinaryOp::Lt => Self::Lt,
            rinha::ast::BinaryOp::Gt => Self::Gt,
            rinha::ast::BinaryOp::Lte => Self::Lte,
            rinha::ast::BinaryOp::Gte => Self::Gte,
            rinha::ast::BinaryOp::And => Self::And,
            rinha::ast::BinaryOp::Or => Self::Or,
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Binary {
    pub lhs: Box<Term>,
    pub op: BinaryOp,
    pub rhs: Box<Term>,
    pub location: Location,
}

impl Element for Binary {
    fn location(&self) -> &Location {
        &self.location
    }
}

impl From<rinha::ast::Binary> for Binary {
    fn from(value: rinha::ast::Binary) -> Self {
        Self {
            lhs: Box::new(Term::from(*value.lhs)),
            op: BinaryOp::from(value.op),
            rhs: Box::new(Term::from(*value.rhs)),
            location: value.location.into(),
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Call {
    pub callee: Box<Term>,
    pub arguments: Vec<Term>,
    pub location: Location,
}

impl From<rinha::ast::Call> for Call {
    fn from(value: rinha::ast::Call) -> Self {
        Self {
            callee: Box::new(Term::from(*value.callee)),
            arguments: value.arguments.into_iter().map(Term::from).collect(),
            location: value.location.into(),
        }
    }
}

impl Element for Call {
    fn location(&self) -> &Location {
        &self.location
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Function {
    pub parameters: Vec<Var>,
    pub value: Box<Term>,
    pub location: Location,
}

impl From<rinha::ast::Function> for Function {
    fn from(value: rinha::ast::Function) -> Self {
        Self {
            parameters: value
                .parameters
                .into_iter()
                .map(|v| Var {
                    text: v.text,
                    location: v.location.into(),
                })
                .collect(),
            value: Box::new(Term::from(*value.value)),
            location: value.location.into(),
        }
    }
}

impl Element for Function {
    fn location(&self) -> &Location {
        &self.location
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Print {
    pub value: Box<Term>,
    pub location: Location,
}

impl From<rinha::ast::Print> for Print {
    fn from(value: rinha::ast::Print) -> Self {
        Self {
            value: Box::new(Term::from(*value.value)),
            location: value.location.into(),
        }
    }
}

impl Element for Print {
    fn location(&self) -> &Location {
        &self.location
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct First {
    pub value: Box<Term>,
    pub location: Location,
}

impl From<rinha::ast::First> for First {
    fn from(value: rinha::ast::First) -> Self {
        Self {
            value: Box::new(Term::from(*value.value)),
            location: value.location.into(),
        }
    }
}

impl Element for First {
    fn location(&self) -> &Location {
        &self.location
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Second {
    pub value: Box<Term>,
    pub location: Location,
}

impl From<rinha::ast::Second> for Second {
    fn from(value: rinha::ast::Second) -> Self {
        Self {
            value: Box::new(Term::from(*value.value)),
            location: value.location.into(),
        }
    }
}

impl Element for Second {
    fn location(&self) -> &Location {
        &self.location
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Tuple {
    pub first: Box<Term>,
    pub second: Box<Term>,
    pub location: Location,
}

impl From<rinha::ast::Tuple> for Tuple {
    fn from(value: rinha::ast::Tuple) -> Self {
        let first = Term::from(*value.first);
        let second = Term::from(*value.second);
        let location = Location::from(value.location);
        Self {
            first: Box::new(first),
            second: Box::new(second),
            location: location,
        }
    }
}

impl Element for Tuple {
    fn location(&self) -> &Location {
        &self.location
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "kind")]
pub enum Term {
    Error(Error),
    Int(Int),
    Str(Str),
    Call(Call),
    Binary(Binary),
    Function(Function),
    Let(Let),
    If(If),
    Print(Print),
    First(First),
    Second(Second),
    Bool(Bool),
    Tuple(Tuple),
    Var(Var),
}

impl From<rinha::ast::Term> for Term {
    fn from(value: rinha::ast::Term) -> Self {
        match value {
            rinha::ast::Term::Error(e) => Self::Error(e.into()),
            rinha::ast::Term::Int(i) => Self::Int(i.into()),
            rinha::ast::Term::Str(s) => Self::Str(s.into()),
            rinha::ast::Term::Call(c) => Self::Call(c.into()),
            rinha::ast::Term::Binary(b) => Self::Binary(b.into()),
            rinha::ast::Term::Function(f) => Self::Function(f.into()),
            rinha::ast::Term::Let(l) => Self::Let(l.into()),
            rinha::ast::Term::If(i) => Self::If(i.into()),
            rinha::ast::Term::Print(p) => Self::Print(p.into()),
            rinha::ast::Term::First(f) => Self::First(f.into()),
            rinha::ast::Term::Second(s) => Self::Second(s.into()),
            rinha::ast::Term::Bool(b) => Self::Bool(b.into()),
            rinha::ast::Term::Tuple(t) => Self::Tuple(t.into()),
            rinha::ast::Term::Var(v) => Self::Var(v.into()),
        }
    }
}

impl Element for Term {
    fn location(&self) -> &Location {
        match self {
            Term::Error(arg0) => &arg0.location,
            Term::Int(arg0) => &arg0.location,
            Term::Str(arg0) => &arg0.location,
            Term::Function(arg0) => &arg0.location,
            Term::Call(arg0) => arg0.location(),
            Term::Var(arg0) => &arg0.location,
            Term::Binary(arg0) => &arg0.location,
            Term::Print(arg0) => &arg0.location,
            Term::First(arg0) => &arg0.location,
            Term::Second(arg0) => &arg0.location,
            Term::Let(arg0) => &arg0.location,
            Term::If(arg0) => &arg0.location,
            Term::Bool(arg0) => &arg0.location,
            Term::Tuple(arg0) => arg0.location(),
        }
    }
}
