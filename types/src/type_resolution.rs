use ast::{DirectDeclaratorPart, Declarator, Pointer, AssignmentExpression,
          ParameterTypeList, AbstractDeclarator, ParameterDeclaration};
use errors::CompilationError;
use crate::*;

enum DeclaratorPartialType {
    Pointer(Pointer),
    Array(Option<AssignmentExpression>),
    Function(ParameterTypeList)
}

pub trait ResolveDeclarator {
    fn resolve(&self, initial_type: QualifiedType) -> (QualifiedType, Option<String>);
}

pub trait ResolveAbstractDeclarator {
    fn resolve(&self, initial_type: QualifiedType) -> QualifiedType;
}

fn resolve(initial_type: QualifiedType, identifier: Option<String>, mut stack: Vec<DeclaratorPartialType>) -> (QualifiedType, Option<String>) {
    let mut t = initial_type;

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
                    let (resolved_param_list, variadic) = resolve_parameter_type_list(param_list).unwrap();
                    QualifiedType::new(TypeQualifiers::default(),
                              Type::Function{ parameters: resolved_param_list, variadic, returns: Box::new(t) })
                }
            };
        }

        (t, identifier)
}

impl ResolveDeclarator for Declarator {
    fn resolve(&self, initial_type: QualifiedType) -> (QualifiedType, Option<String>) {
        let mut identifier = None;
        let stack = self.build_stack(&mut identifier);
        resolve(initial_type, identifier, stack)
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
                    &mut $right
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

impl ResolveAbstractDeclarator for AbstractDeclarator {
    fn resolve(&self, initial_type: QualifiedType) -> QualifiedType {
        let mut stack = build_abstract_declarator_type_stack(self);
        resolve(initial_type, None, stack).0
    }
}

fn build_abstract_declarator_type_stack(declarator: &AbstractDeclarator) -> Vec<DeclaratorPartialType> {
    use ast::DirectAbstractDeclaratorPart;

    match declarator {
        AbstractDeclarator::Pointer(ptr_list) => {
            let mut stack = Vec::new();

            for ptr in ptr_list.iter() {
                stack.push(DeclaratorPartialType::Pointer(ptr.clone()));
            }
            
            stack
        },
        AbstractDeclarator::DirectAbstractDeclarator{ pointer, direct_abstract_declarator } => {
            let mut stack: Vec<DeclaratorPartialType> = Vec::new();

            if let Some(ptr_list) = pointer {
                for ptr in ptr_list.iter() {
                    stack.push(DeclaratorPartialType::Pointer(ptr.clone()));
                }
            }

            for part in direct_abstract_declarator.iter() {
                match part {
                    DirectAbstractDeclaratorPart::Parens(dec) => {
                        let mut decl_stack = build_abstract_declarator_type_stack(&dec);
                        while let Some(element) = decl_stack.pop() {
                            stack.push(element);
                        }
                    },
                    DirectAbstractDeclaratorPart::Array(size) => stack.push(DeclaratorPartialType::Array(size.clone())),
                    DirectAbstractDeclaratorPart::VLA => stack.push(DeclaratorPartialType::Array(None)),
                    DirectAbstractDeclaratorPart::ParameterTypeList(type_list) => {
                        stack.push(DeclaratorPartialType::Function(*type_list.clone()))
                    }
                }
            }

            stack
        }
    }
}

fn resolve_parameter_type_list<'a>(param_list: ParameterTypeList) -> Result<(Vec<QualifiedType>, bool), CompilationError<'a>> {
    let mut params = Vec::new();
    for param in param_list.parameter_list {
        params.push(resolve_parameter_declaration(&param)?);
    }
    Ok((params, param_list.variadic))
}

fn resolve_parameter_declaration<'a>(declaration: &ParameterDeclaration) -> Result<QualifiedType, CompilationError<'a>> {
    use ast::ParameterDeclarator;

    let base_type = QualifiedType::from_declaration_specifier_list(&declaration.declaration_specifier_list)?;

    match &declaration.declarator {
        ParameterDeclarator::Declarator(declarator) => {
            Ok(declarator.resolve(base_type).0)
        },
        ParameterDeclarator::AbstractDeclarator(Some(declarator)) => {
            Ok(declarator.resolve(base_type))
        },
        ParameterDeclarator::AbstractDeclarator(None) => Ok(base_type)
    }
}
