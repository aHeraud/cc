use std::convert::From;
use std::default::Default;

use ast::{Location, Node};
use errors::InvalidTypeSpecifierCombination;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct TypeQualifiers {
    pub constant: bool,
    pub volatile: bool,
    pub restrict: bool
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

impl<Iter: Iterator<Item=ast::TypeQualifier>> From<Iter> for TypeQualifiers {
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

impl StorageClass {
    // temporarily not using TryFrom because it doesn't work on the nightly version
    // of the rust compiler im using.
    fn try_from<Iter: Iterator<Item=ast::StorageClassSpecifier>>(storage_class_iter: Iter) -> Result<StorageClass, ()> {
        let mut current: Option<StorageClass> = None;
        for class in storage_class_iter {
            if current.is_none() {
                current = Some(class.into());
            }
            else if current != Some(class.into()) {
                // a declaration can only have 1 storage class specifier (duplicates are okay though)
                return Err(())
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
}

#[derive(Debug, Clone)]
pub enum Type {
    Void,
    Bool,
    Integer(IntegerType),
    Float(FloatType),
    Struct(StructID),
    Union(UnionID),
    Enum(EnumID),
    Function{ parameters: Vec<QualifiedType>, variadic: bool, returns: Box<QualifiedType> },
    Array{ inner: Box<QualifiedType>, size: Option<ast::AssignmentExpression> }, // TODO: convert the size into something more useful
    Pointer(Box<QualifiedType>)
}

// TODO: handle enum/struct declaration
// TODO: get enum/struct ids from symbol table
impl Type {
    fn make_type<Iter: Iterator<Item = Node<ast::TypeSpecifier>>>(iter: Iter, declaration: &Node<ast::Declaration>) -> Result<Type, InvalidTypeSpecifierCombination> {
        use ast::TypeSpecifier::*;

        let mut void: Option<ast::TypeSpecifier> = None;
        let mut char_: Option<ast::TypeSpecifier> = None;
        let mut short: Option<ast::TypeSpecifier> = None;
        let mut long: Option<ast::TypeSpecifier> = None;
        let mut long_long: Option<ast::TypeSpecifier> = None;
        let mut int: Option<ast::TypeSpecifier> = None;
        let mut float: Option<ast::TypeSpecifier> = None;
        let mut double: Option<ast::TypeSpecifier> = None;
        let mut unsigned: Option<ast::TypeSpecifier> = None;
        let mut signed: Option<ast::TypeSpecifier> = None;
        let mut bool_: Option<ast::TypeSpecifier> = None;

        macro_rules! check_compatability {
            ( $declaration:ident, $item:ident, $specifiers_to_check:expr) => {
                for specifier in $specifiers_to_check {
                    if let Some(s) = specifier {
                        let prev: ast::TypeSpecifier = ast::TypeSpecifier::clone(s);
                        let err = InvalidTypeSpecifierCombination::new($declaration.clone(), $item.clone(), prev);
                        return Err(err);
                    }
                }
            };
        }

        for item in iter {
            match item.value {
                Void => {
                    // can't combine void specifier with any others
                    check_compatability!(declaration, item, &[&void, &char_, &short, &long, &long_long, &int, &float, &double, &unsigned, &signed, &bool_]);
                    void = Some(item.value);
                },
                Char => {
                    // char can only be combined with unsigned and signed specifiers
                    check_compatability!(declaration, item, &[&void, &short, &long, &long_long, &float, &double, &int, &char_]);
                    char_ = Some(item.value);
                },
                Short => {
                    // 'short' can only be combined with itself, 'int', 'unsigned', and 'signed' type specifiers.
                    check_compatability!(declaration, item, &[&void, &char_, &long, &long_long, &float, &double, &bool_]);
                    // TODO: warn if multiple short specifiers
                    short = Some(item.value);
                },
                Int => {
                    // 'int' can only be combined with 'short', 'long', 'long long', 'unsigned', and 'signed' type specifiers.
                    check_compatability!(declaration, item, &[&void, &char_, &float, &double, &bool_]);
                    int = Some(item.value);
                },
                Long => {
                    // 'long' can only be combined with 'int', 'unsigned', 'signed', and 'double'.
                    check_compatability!(declaration, item, &[&void, &char_, &short, &long_long, &float, &bool_]);
                    if long.is_some() {
                        long = None;
                        // 'long long' is compatible with the same type specifiers as 'long' with the exception of 'double'.
                        check_compatability!(declaration, item, &[&double]);
                        long_long = Some(item.value);
                    }
                    else {
                        long = Some(item.value);
                    }
                },
                Float => {
                    // 'float' can't be combined with any other type specifiers.
                    check_compatability!(declaration, item, &[&void, &char_, &short, &long, &long_long, &int, &unsigned, &signed, &bool_, &float, &double]);
                    float = Some(item.value);
                },
                Double => {
                    // 'double' can only be combined with 'long'.
                    check_compatability!(declaration, item, &[&void, &char_, &short, &long_long, &int, &unsigned, &signed, &bool_, &float, &double]);
                    double = Some(item.value);
                },
                Unsigned => {
                    // 'unsigned' can be combined with itself and the 'char', 'short', and 'int' type specifiers.
                    check_compatability!(declaration, item, &[&void, &bool_, &signed, &float, &double]);
                    unsigned = Some(item.value);
                },
                Signed => {
                    // 'signed' can be combined with itself, and the 'char', 'short', and 'int' type specifiers.
                    check_compatability!(declaration, item, &[&void, &bool_, &unsigned, &float, &double]);
                    signed = Some(item.value);
                },
                Bool => {
                    // 'bool' can't be combined with any other type specifiers.
                    check_compatability!(declaration, item, &[&void, &bool_, &char_, &unsigned, &signed, &int, &short, &long, &long_long, &float, &double]);
                    bool_ = Some(item.value);
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
            Type::Bool
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

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum FloatType {
    Float,
    Double,
    LongDouble // unsupported for now
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StructID(i32);

// TODO
pub struct Struct {
    name: String,
    id: StructID
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
