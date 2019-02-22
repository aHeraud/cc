use std::collections::HashMap;
use std::rc::Rc;

use ast::Location;
use errors::{CompilationError, EnumVariantRedefinition};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EnumID(i32);

pub struct Enum {
    name: String,
    
    id: EnumID,

    /// Maps each enum variant to a value
    variants: HashMap<String, i32>
}

pub struct EnumBuilder {
    name: String,
    id: EnumID,

    /// holds each variant of the current enum.
    /// 
    /// variants that don't have a value assigned manually
    /// will automatically have a value assigned when the
    /// enum is finalized.
    variants: HashMap<String, Option<i32>>,
}

impl<'a> EnumBuilder {
    pub fn new(name: Option<String>, id: EnumID) -> EnumBuilder {
        EnumBuilder {
            name: name.unwrap_or(format!("anonymous enum #{}", id.0)),
            id,
            variants: HashMap::new(),
        }
    }

    pub fn add_variant<T: Into<String>>(&mut self, name: T, value: Option<i32>, location: (Location, Location)) -> Result<(), CompilationError<'a>> {
        let name = name.into();
        if self.variants.contains_key(&name) {
            Err(EnumVariantRedefinition::new(name, location).into())
        }
        else {
            self.variants.insert(name, value);
            Ok(())
        }
    }

    pub fn build(self) -> Enum {
        let mut value_counter = 0;

        let mut variants: HashMap<Rc<String>, i32> = HashMap::new();
        {
            let mut missing_value = Vec::new();
            let mut values: HashMap<i32, Rc<String>> = HashMap::new();

            for var in self.variants {
                let name = Rc::new(var.0);
                if let Some(value) = var.1 {
                    if let Some(other) = values.get(&value) {
                        // TODO: do something sensible with warnings
                        println!("WARNING: in enum {}, variants '{}' and '{}' share the same value", self.name, name, other);
                    }
                    variants.insert(name.clone(), value);
                    values.insert(value, name);
                }
                else {
                    missing_value.push(name)
                }
            }

            for name in missing_value {
                while values.contains_key(&value_counter) {
                    value_counter += 1; // this will overflow if you have more than 2^31 enum variants!
                    if value_counter == i32::max_value() {
                        panic!("too many enum variants")
                    }
                }
                variants.insert(name, value_counter);
                value_counter += 1;
            }
        }

        Enum {
            name: self.name,
            id: self.id,
            variants: variants.into_iter().map(|(name, value)| (Rc::try_unwrap(name).unwrap(), value)).collect()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn enum_1_variant_with_value() {
        let mut builder = EnumBuilder::new(Some("Foo".into()), EnumID(1));
        builder.add_variant("Bar", Some(1), (Location::default(), Location::default())).unwrap();
        let result = builder.build();
        let mut variants = HashMap::new();
        variants.insert("Bar".into(), 1i32);
        assert_eq!(result.variants, variants);
    }

    #[test]
    fn enum_3_variants_mixed_values() {
        let mut builder = EnumBuilder::new(Some("Foo".into()), EnumID(1));
        builder.add_variant("Bar", None, (Location::default(), Location::default())).unwrap();
        builder.add_variant("Foo", Some(0), (Location::default(), Location::default())).unwrap();
        builder.add_variant("Baz", Some(1), (Location::default(), Location::default())).unwrap();
        let result = builder.build();
        let mut variants = HashMap::new();
        variants.insert("Bar".into(), 2i32);
        variants.insert("Foo".into(), 0i32);
        variants.insert("Baz".into(), 1i32);
        assert_eq!(result.variants, variants);
    }
}
