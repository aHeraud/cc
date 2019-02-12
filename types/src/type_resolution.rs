use ast::{Declaration, DirectDeclaratorPart, DirectDeclarator, Declarator, Pointer, AssignmentExpression,
          ParameterTypeList};
use crate::types::*;

enum DeclaratorPartialType {
    Pointer(Pointer),
    Array(Option<AssignmentExpression>),
    Function(ParameterTypeList)
}

pub trait ResolveDeclarator {
    fn resolve(&self, declaration_specifiers: QualifiedType) -> QualifiedType;
}

impl ResolveDeclarator for Declarator {
    fn resolve(&self, decl_spec: QualifiedType) -> QualifiedType {
        let mut identifier = None;
        let mut stack = self.build_stack(&mut identifier);
        let mut t = decl_spec;

        while let Some(partial_type) = stack.pop() {
            t = match partial_type {
                DeclaratorPartialType::Pointer(ptr) => {
                    let qualifiers: TypeQualifiers = ptr.qualifiers.into_iter().into();
                    let ptr = QualifiedType::new(qualifiers, Type::Pointer(Box::new(t)));
                    ptr
                },
                DeclaratorPartialType::Array(size) => {
                    QualifiedType::new(TypeQualifiers::default(), 
                              Type::Array{ inner: Box::new(t), size })
                },
                DeclaratorPartialType::Function(param_list) => {
                    panic!()
                    //QualifiedType::new(TypeQualifiers::default(),
                    //          Type::Function{ parameters: param_list, returns: Box::new(t) })
                }
            };
        }

        t
    }
}

trait BuildDeclaratorTypeStack {
    fn build_stack(&self, identifier: &mut Option<String>) -> Vec<DeclaratorPartialType>;
}

impl BuildDeclaratorTypeStack for Declarator {
    fn build_stack(&self, identifier: &mut Option<String>) -> Vec<DeclaratorPartialType> {
        let mut left: Vec<DeclaratorPartialType> = Vec::new();
        let mut right: Vec<DeclaratorPartialType> = Vec::new();

        macro_rules! active_stack {
            ($identifier:expr, $left:expr, $right:expr) => {
                if $identifier.is_none() {
                    &mut $left
                }
                else {
                    &mut right
                }
            };
        }

        if let Some(ref ptr_list) = self.pointer {
            let active_stack = active_stack!(identifier, left, right);

            for ptr in ptr_list.iter() {
                active_stack.push(DeclaratorPartialType::Pointer(ptr.clone()));
            }
        }

        for part in self.direct_declarator.iter() {

            match part {
                DirectDeclaratorPart::Identifier(ident) => {
                    if identifier.is_none() {
                        *identifier = Some(ident.clone());
                    }
                    else {
                        // TODO: properly handle error
                        panic!("multiple identifiers in declarator");
                    }
                },
                DirectDeclaratorPart::Parens(dec) => {
                    let mut stack = dec.build_stack(identifier);
                    let active_stack = active_stack!(identifier, left, right);
                    while let Some(element) = stack.pop() {
                        active_stack.push(element);
                    }
                },
                DirectDeclaratorPart::Array(size) => active_stack!(identifier, left, right).push(DeclaratorPartialType::Array(size.clone())),
                DirectDeclaratorPart::VLA => active_stack!(identifier, left, right).push(DeclaratorPartialType::Array(None)),
                DirectDeclaratorPart::ParameterTypeList(type_list) => active_stack!(identifier, left, right).push(DeclaratorPartialType::Function(type_list.clone())),
                DirectDeclaratorPart::IdentifierList(ident_list) => { 
                        /* TODO: convert identifier list to type_list (assume implicit int?) */
                        /* For now, this just assumes that every identifier in the identifier list represents an integer argument (implicit int),
                         * but i'm not 100% sure that this is the correct behaviour. 
                         * TODO: should we assume that identifiers in the identifier list should represent an integer argument?
                         * TODO: warn on implicit integer arguments in identifier list (basically just emit a warning for any non empty identifier-list here
                         */
                        let type_list = ParameterTypeList::new(vec![], false);
                        if ident_list.len() > 0 {
                            // TODO
                            panic!("declarators that include an identifier list are unimplemented");
                        }
                        active_stack!(identifier, left, right).push(DeclaratorPartialType::Function(type_list));
                    }
            };
        }

        while let Some(part) = left.pop() {
            right.push(part);
        }

        right
    }
}
