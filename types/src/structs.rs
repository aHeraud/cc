use std::collections::HashMap;

use crate::{Type, IntegerType};
use ast::Location;
use errors::{CompilationError, BitFieldSizeExceedsTypeWidth, DuplicateStructMember, NonIntegralBitfieldType};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StructID(i32);

pub struct Struct {
    name: String,
    id: StructID,
    fields: HashMap<String, Field>,
    bytes: usize
}

pub enum Field {
    Field { type_: Type, offset: usize },
    BitField { type_: Type, offset: usize, bits: usize, bit_offset: usize }
}

pub struct StructBuilder {
    name: String,
    id: StructID,
    fields: HashMap<String, StructBuilderField>
}

enum StructBuilderField {
    Field { type_: Type },
    BitField { type_: IntegerType, bits: usize }
}

impl StructBuilder {
    pub fn new(name: Option<String>, id: StructID) -> StructBuilder {
        StructBuilder {
            name: name.unwrap_or(format!("anonymous struct #{}", id.0)),
            id,
            fields: HashMap::new()
        }
    }

    pub fn add_field(&mut self, name: String, type_: Type) -> Result<(), ()> {
        if self.fields.contains_key(&name) {
            // a field with this name already exists
            // TODO: return error
            Err(())
        }
        else {
            self.fields.insert(name, StructBuilderField::Field{ type_ });
            Ok(())
        }
    }

    pub fn add_bit_field(&mut self, location: (Location, Location), name: String, type_: Type, bits: usize) -> Result<(), CompilationError> {
        if let Type::Integer(int_type) = type_ {
            if bits <= int_type.bits() {
                if self.fields.contains_key(&name) {
                    // a field with this name already exists
                    // TODO: return error
                    Err(DuplicateStructMember::new(location, name).into())
                }
                else {
                    self.fields.insert(name, StructBuilderField::BitField{ type_: int_type, bits });
                    Ok(())
                }
            }
            else {
                // bitfield size larger than containing type
                Err(BitFieldSizeExceedsTypeWidth::new(location, name, bits, int_type.bits()).into())
            }
        }
        else {
            // bitfield type must be an integer type
            Err(NonIntegralBitfieldType::new(location, name).into())
        }
    }

    pub fn build(self) -> Struct {
        unimplemented!()
    }
}
