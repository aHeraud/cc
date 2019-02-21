use std::convert::From;
use std::default::Default;
use std::fmt;
use std::fmt::{Display, Formatter};

use crate::StructID;
use ast::{Location, Node};
use errors::{InvalidTypeSpecifierCombination, InvalidStorageClassSpecifierCombination};

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct TypeQualifiers {
    pub constant: bool,
    pub volatile: bool,
    pub restrict: bool
}

impl Display for TypeQualifiers {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut s = String::new();
        if self.constant {
            s.push_str("const");
        }
        if self.volatile {
            if s.len() > 0 {
                s.push(' ');
            }
            s.push_str("volatile");
        }
        if self.restrict {
            if s.len() > 0 {
                s.push(' ');
            }
            s.push_str("restrict");
        }
        write!(f, "{}", s)
    }
}

impl Default for TypeQualifiers {
    fn default() -> TypeQualifiers {
        TypeQualifiers {
            constant: false,
            volatile: false,
            restrict: false
        }
    }
}

impl<'a, Iter: Iterator<Item=ast::TypeQualifier>> From<Iter> for TypeQualifiers {
    fn from(qualifier_list: Iter) -> TypeQualifiers {
        use ast::TypeQualifier;

        let mut constant = false;
        let mut volatile = false;
        let mut restrict = false;

        for qualifier in qualifier_list {
            match qualifier {
                TypeQualifier::Const => constant = true,
                TypeQualifier::Restrict => restrict = true,
                TypeQualifier::Volatile => volatile = true
            }
        }

        TypeQualifiers {
            constant,
            volatile,
            restrict
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum StorageClass {
    Auto,
    Register,
    Static,
    Extern,
    Typedef
}

impl Display for StorageClass {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        use StorageClass::*;
        match self {
            Auto => write!(f, "auto"),
            Register => write!(f, "register"),
            Static => write!(f, "static"),
            Extern => write!(f, "extern"),
            Typedef => write!(f, "typedef")
        }
    }
}

impl From<ast::StorageClassSpecifier> for StorageClass {
    fn from(class: ast::StorageClassSpecifier) -> StorageClass {
        use ast::StorageClassSpecifier::*;
        match class {
            Auto => StorageClass::Auto,
            Register => StorageClass::Register,
            Static => StorageClass::Static,
            Extern => StorageClass::Extern,
            Typedef => StorageClass::Typedef
        }
    }
}

impl Into<ast::StorageClassSpecifier> for StorageClass {
    fn into(self) -> ast::StorageClassSpecifier {
        use StorageClass::*;
        match self {
            Auto => ast::StorageClassSpecifier::Auto,
            Register => ast::StorageClassSpecifier::Register,
            Static => ast::StorageClassSpecifier::Static,
            Extern => ast::StorageClassSpecifier::Extern,
            Typedef => ast::StorageClassSpecifier::Typedef
        }
    }
}

impl StorageClass {
    // temporarily not using TryFrom because it doesn't work on the nightly version
    // of the rust compiler im using.
    pub fn try_from_specifier_list(specifier_list: &Node<ast::DeclarationSpecifiers>) -> Result<StorageClass, InvalidStorageClassSpecifierCombination> {
        let mut current: Option<StorageClass> = None;
        let mut iter = specifier_list.value.iter().filter_map(|el| {
            match el {
                ast::DeclarationSpecifier::StorageClassSpecifier(node) => Some(node),
                _ => None
            }
        });

        for class in iter {
            if let Some(current) = current {
                if current != class.value.into() {
                    // a declaration can only have 1 storage class specifier (duplicates are okay though)
                    return Err(InvalidStorageClassSpecifierCombination::new(specifier_list.clone(), class.clone(), current.into()))
                }
            }
            else {
                current = Some(class.value.into());
            }
        }
        Ok(current.unwrap_or(StorageClass::Auto))
    }
}

#[derive(Debug, Clone)]
pub struct QualifiedType {
    pub qualifiers: TypeQualifiers,
    pub type_: Type
}

impl QualifiedType {
    pub fn new(qualifiers: TypeQualifiers, type_: Type) -> QualifiedType {
        QualifiedType {
            qualifiers,
            type_
        }
    }

    pub fn from_declaration_specifier_list(specifier_list: &Node<ast::DeclarationSpecifiers>) -> Result<QualifiedType, InvalidTypeSpecifierCombination> {
        let qualifiers = TypeQualifiers::from(specifier_list.value.iter().filter_map(|v| {
            match v {
                ast::DeclarationSpecifier::TypeQualifier(node) => Some(node),
                _ => None
            }
        }).map(|v| v.value));

        let type_ = Type::make_type(specifier_list)?;

        Ok(QualifiedType::new(qualifiers, type_))
    }
}

impl Display for QualifiedType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let qualifiers = format!("{}", self.qualifiers);
        if qualifiers.len() == 0 {
            write!(f, "{}", self.type_)
        }
        else {
            write!(f, "{} {}", qualifiers, self.type_)
        }
    }
}

struct DisplayVec<'a, T: Display>(&'a Vec<T>);

// TODO: put this somewhere else
impl<'a, T: Display> Display for DisplayVec<'a, T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut s = String::new();
        let mut iter = self.0.iter();
        if let Some(t) = iter.next() {
            s.push_str(&format!("{}", t));
        }
        for t in iter {
            s.push_str(", ");
            s.push_str(&format!("{}", t));
        }
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone)]
pub enum Type {
    Void,
    Integer(IntegerType),
    Float(FloatType),
    Struct(StructID),
    Union(UnionID),
    Enum(EnumID),
    Function{ parameters: Vec<QualifiedType>, variadic: bool, returns: Box<QualifiedType> },
    Array{ inner: Box<QualifiedType>, size: Option<ast::AssignmentExpression> }, // TODO: convert the size into something more useful
    Pointer(Box<QualifiedType>)
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        use Type::*;
        match self {
            Void => write!(f, "void"),
            Integer(int) => write!(f, "{}", int),
            Float(float) => write!(f, "{}", float),
            Struct(_) => write!(f, "struct"), // TODO: how should this be displayed?
            Union(_) => write!(f, "union"), // TODO: how should this be displayed?
            Enum(_) => write!(f, "enum"), // TODO: how should this be displayed?
            Function { parameters, variadic, returns }  => {
                if *variadic {
                    write!(f, "fn({}, ...) -> {}", DisplayVec(&parameters), returns)
                }
                else {
                    write!(f, "fn({}) -> {}", DisplayVec(&parameters), returns)
                }
            },
            Array { inner, size } => write!(f, "array of {}", inner), // TODO: implement display for size
            Pointer(inner) => write!(f, "pointer to {}", inner)
        }
    }
}

// TODO: handle enum/struct declaration
// TODO: get enum/struct ids from symbol table
impl Type {
    pub fn make_type(specifier_list: &Node<ast::DeclarationSpecifiers>) -> Result<Type, InvalidTypeSpecifierCombination> {
        use ast::TypeSpecifier::*;

        let mut void: Option<&ast::TypeSpecifier> = None;
        let mut char_: Option<&ast::TypeSpecifier> = None;
        let mut short: Option<&ast::TypeSpecifier> = None;
        let mut long: Option<&ast::TypeSpecifier> = None;
        let mut long_long: Option<&ast::TypeSpecifier> = None;
        let mut int: Option<&ast::TypeSpecifier> = None;
        let mut float: Option<&ast::TypeSpecifier> = None;
        let mut double: Option<&ast::TypeSpecifier> = None;
        let mut unsigned: Option<&ast::TypeSpecifier> = None;
        let mut signed: Option<&ast::TypeSpecifier> = None;
        let mut bool_: Option<&ast::TypeSpecifier> = None;

        macro_rules! check_compatability {
            ( $specifier_list:ident, $item:ident, $specifiers_to_check:expr) => {
                for specifier in $specifiers_to_check {
                    if let Some(s) = specifier {
                        let prev: ast::TypeSpecifier = ast::TypeSpecifier::clone(s);
                        let err = InvalidTypeSpecifierCombination::new($specifier_list.clone(), $item.clone(), prev);
                        return Err(err);
                    }
                }
            };
        }

        let mut iter = specifier_list.value.iter().filter_map(|el| {
            match el {
                ast::DeclarationSpecifier::TypeSpecifier(node) => Some(node),
                _ => None
            }
        });

        for item in iter {
            match item.value {
                Void => {
                    // can't combine void specifier with any others
                    check_compatability!(specifier_list, item, &[&void, &char_, &short, &long, &long_long, &int, &float, &double, &unsigned, &signed, &bool_]);
                    void = Some(&item.value);
                },
                Char => {
                    // char can only be combined with unsigned and signed specifiers
                    check_compatability!(specifier_list, item, &[&void, &short, &long, &long_long, &float, &double, &int, &char_]);
                    char_ = Some(&item.value);
                },
                Short => {
                    // 'short' can only be combined with itself, 'int', 'unsigned', and 'signed' type specifiers.
                    check_compatability!(specifier_list, item, &[&void, &char_, &long, &long_long, &float, &double, &bool_]);
                    // TODO: warn if multiple short specifiers
                    short = Some(&item.value);
                },
                Int => {
                    // 'int' can only be combined with 'short', 'long', 'long long', 'unsigned', and 'signed' type specifiers.
                    check_compatability!(specifier_list, item, &[&void, &char_, &float, &double, &bool_]);
                    int = Some(&item.value);
                },
                Long => {
                    // 'long' can only be combined with 'int', 'unsigned', 'signed', and 'double'.
                    check_compatability!(specifier_list, item, &[&void, &char_, &short, &long_long, &float, &bool_]);
                    if long.is_some() {
                        long = None;
                        // 'long long' is compatible with the same type specifiers as 'long' with the exception of 'double'.
                        check_compatability!(specifier_list, item, &[&double]);
                        long_long = Some(&item.value); // FIXME: convert to TypeSpecifier::LongLong
                    }
                    else {
                        long = Some(&item.value);
                    }
                },
                Float => {
                    // 'float' can't be combined with any other type specifiers.
                    check_compatability!(specifier_list, item, &[&void, &char_, &short, &long, &long_long, &int, &unsigned, &signed, &bool_, &float, &double]);
                    float = Some(&item.value);
                },
                Double => {
                    // 'double' can only be combined with 'long'.
                    check_compatability!(specifier_list, item, &[&void, &char_, &short, &long_long, &int, &unsigned, &signed, &bool_, &float, &double]);
                    double = Some(&item.value);
                },
                Unsigned => {
                    // 'unsigned' can be combined with itself and the 'char', 'short', and 'int' type specifiers.
                    check_compatability!(specifier_list, item, &[&void, &bool_, &signed, &float, &double]);
                    unsigned = Some(&item.value);
                },
                Signed => {
                    // 'signed' can be combined with itself, and the 'char', 'short', and 'int' type specifiers.
                    check_compatability!(specifier_list, item, &[&void, &bool_, &unsigned, &float, &double]);
                    signed = Some(&item.value);
                },
                Bool => {
                    // 'bool' can't be combined with any other type specifiers.
                    check_compatability!(specifier_list, item, &[&void, &bool_, &char_, &unsigned, &signed, &int, &short, &long, &long_long, &float, &double]);
                    bool_ = Some(&item.value);
                },
                _ => unimplemented!() // structs/enums
            }
        }

        let t = if void.is_some() {
            Type::Void
        }
        else if float.is_some() {
            Type::Float(FloatType::Float)
        }
        else if double.is_some() {
            if long.is_some() {
                Type::Float(FloatType::LongDouble)
            }
            else {
                Type::Float(FloatType::Double)
            }
        }
        else if bool_.is_some() {
            Type::Integer(IntegerType::Bool)
        }
        else if short.is_some() {
            let int_type = if unsigned.is_some() {
                IntegerType::U16
            }
            else {
                IntegerType::I16
            };
            Type::Integer(int_type)
        }
        else if char_.is_some() {
            let char_type = if unsigned.is_some() {
                IntegerType::U8
            }
            else {
                IntegerType::I8
            };
            Type::Integer(char_type)
        }
        else if long_long.is_some() {
            let int_type = if unsigned.is_some() {
                IntegerType::U128
            }
            else {
                IntegerType::I128
            };
            Type::Integer(int_type)
        }
        else if long.is_some() {
            let int_type = if unsigned.is_some() {
                IntegerType::U64
            }
            else {
                IntegerType::I64
            };
            Type::Integer(int_type)
        }
        else {
            let int_type = if unsigned.is_some() {
                IntegerType::U32
            }
            else {
                IntegerType::I32
            };
            Type::Integer(int_type)
        };

        Ok(t)
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum IntegerType {
    Bool,
    U8,
    I8,
    U16,
    I16,
    U32,
    I32,
    U64,
    I64,
    U128,
    I128
}

impl IntegerType {
    pub fn bits(&self) -> usize {
        use IntegerType::*;
        match self {
            Bool => 1,
            U8 | I8 => 8,
            U16 | I16 => 16,
            U32 | I32 => 32,
            U64 | I64 => 64,
            U128 | I128 => 128
        }
    }
}

impl Display for IntegerType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        use IntegerType::*;
        let s = match self {
            Bool => "_Bool",
            U8 => "unsigned char",
            I8 => "char",
            U16 => "unsigned short int",
            I16 => "short int",
            U32 => "unsigned int",
            I32 => "int",
            U64 => "unsigned long int",
            I64 => "long int",
            U128 => "unsigned long long int",
            I128 => "long long int"
        };
        write!(f, "{}", s)
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum FloatType {
    Float,
    Double,
    LongDouble // unsupported for now
}

impl Display for FloatType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        use FloatType::*;
        match self {
            Float => write!(f, "float"),
            Double => write!(f, "double"),
            LongDouble => write!(f, "long double")
        }
    }
}

pub enum StructField {
    Bitfield{ size: usize },
    Field { type_: Type }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UnionID(i32);

// TODO
pub struct Union {
    name: String,
    id: UnionID
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EnumID(i32);

// TODO
pub struct Enum {
    name: String,
    id: EnumID
}
