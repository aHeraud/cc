use std::convert::From;
use std::default::Default;

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

// FIXME: TODO: error handling
// TODO: Better error messages
// TODO: handle enum/struct declaration
// TODO: get enum/struct ids from symbol table
impl Type {
    fn make_type<Iter: Iterator<Item = ast::TypeSpecifier>>(iter: Iter) -> Type {
        use ast::TypeSpecifier::*;

        enum Float { Float, Double }
        let mut void = false;
        let mut char_ = false;
        let mut short = false;
        let mut long = false;
        let mut long_long = false;
        let mut int = false;
        let mut float: Option<Float> = None;
        let mut unsigned = false;
        let mut signed = false;
        let mut bool_ = false;

        for item in iter {
            match item {
                Void => {
                    if void {
                        panic!("can't combine void specifier with previous void declaration specifier");
                    }
                    void = true;
                },
                Char => {
                    if void || short || long || float.is_some() {
                        panic!("can't combine char specifier with previous declaration specifiers");
                    }
                    char_ = true;
                },
                Short => {
                    if void || char_ || long || float.is_some() {
                        panic!("can't combine short specifier with previous declaration specifiers");
                    }
                    // TODO: warn if multiple short specifiers
                    short = true;
                },
                Int => {
                    int = true;
                },
                Long => {
                    if void || char_ || short || float.is_some() || bool_ {
                        panic!("can't combine long specifier with previous declaration specifiers");
                    }
                    if long {
                        long = false;
                        long_long = true;
                    }
                    else {
                        long = true;
                    }
                },
                Float => {
                    if void || char_ || short || long || long_long || int || unsigned || signed || bool_ {
                        panic!("can't combine float specifier with previous declaration specifiers");
                    }
                    float = Some(Float::Float);
                },
                Double => {
                    if void || char_ || short || long_long || int || unsigned || signed || bool_ || float.is_some() {
                        panic!("can't combine double specifier with previous declaration specifiers");
                    }
                    float = Some(Float::Double);
                },
                Unsigned => {
                    if void || signed || bool_ || float.is_some() {
                        panic!("can't combine unsigned specifier with previous declaration specifiers");
                    }
                    unsigned = true;
                },
                Signed => {
                    if void || unsigned || bool_ || float.is_some() {
                        panic!("can't combine signed specifier with previous declaration specifiers");
                    }
                    signed = true;
                },
                Bool => {
                    if void || unsigned || bool_ || signed || long || long_long || char_ || short || int || float.is_some() {
                        panic!("can't combine _Bool specifier with previous declaration specifiers");
                    }
                    bool_ = true;
                },
                _ => unimplemented!() // structs/enums
            }
        }

        if void {
            Type::Void
        }
        else if let Some(float) = float {
            let float_type = match float {
                Float::Float => FloatType::Float,
                Float::Double => {
                    if long {
                        FloatType::Double
                    }
                    else {
                        FloatType::LongDouble
                    }
                }
            };
            Type::Float(float_type)
        }
        else if bool_ {
            Type::Bool
        }
        else if short {
            let int_type = if unsigned {
                IntegerType::U16
            }
            else {
                IntegerType::I16
            };
            Type::Integer(int_type)
        }
        else if char_ {
            let char_type = if unsigned {
                IntegerType::U8
            }
            else {
                IntegerType::I8
            };
            Type::Integer(char_type)
        }
        else if long_long {
            let int_type = if unsigned {
                IntegerType::U128
            }
            else {
                IntegerType::I128
            };
            Type::Integer(int_type)
        }
        else if long {
            let int_type = if unsigned {
                IntegerType::U64
            }
            else {
                IntegerType::I64
            };
            Type::Integer(int_type)
        }
        else {
            let int_type = if unsigned {
                IntegerType::U32
            }
            else {
                IntegerType::I32
            };
            Type::Integer(int_type)
        }
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
