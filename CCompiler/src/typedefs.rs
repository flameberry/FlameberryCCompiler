use std::fmt;

use crate::analysis::ast::{DeclarationSpecifier, StorageClassSpecifier, TypeName, TypeQualifier, TypeSpecifier};
use crate::analysis::node::Node;
use crate::errors::{CompilerError, CompilerErrorKind};

#[derive(Debug, PartialEq, Clone)]
pub enum IntegerType {
    Generic(i64),
    Signed(i32),
    SignedLong(i64),
    SignedLongLong(i128),

    Unsigned(u32),
    UnsignedLong(u64),
    UnsignedLongLong(u128),
}

#[derive(Debug, PartialEq, Clone)]
pub enum FloatingPointType {
    Float(f32),
    Double(f64),
    LongDouble(f64),
}

#[derive(Debug, Clone)]
pub enum Constant {
    Integer(IntegerType),
    Float(FloatingPointType),
    Character(char),
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub enum PrimitiveType {
    #[default]
    Void,

    // Simple Types
    Bool,
    Char {
        signed: bool,
    },
    Short {
        signed: bool,
    },
    Int {
        signed: bool,
    },
    Long {
        signed: bool,
    },
    LongLong {
        signed: bool,
    },
    Float,
    Double,
    LongDouble,

    // Complex Types
    Pointer {
        inner: Box<TypeInfo>,
    },
    Array {
        element_type: Box<TypeInfo>,
        size: Option<usize>,
    },
    Function {
        return_type: Box<TypeInfo>,
        parameters: Vec<TypeInfo>,
        is_variadic: bool,
    },
    Struct {
        name: String,
        fields: Vec<(String, TypeInfo)>,
    },
    Union {
        name: String,
        fields: Vec<(String, TypeInfo)>,
    },
    Enum {
        name: String,
        underlying_type: Box<TypeInfo>,
    },
    Typedef {
        name: String,
        actual_type: Box<TypeInfo>,
    },
}

impl PrimitiveType {
    /// Get the type for a constant value
    pub fn from_constant(constant: &Constant) -> TypeInfo {
        match constant {
            Constant::Integer(int_type) => {
                match int_type {
                    IntegerType::Generic(val) => {
                        // Infer the smallest type that can hold the value
                        if *val >= 0 {
                            if *val <= i32::MAX as i64 {
                                TypeInfo::new(PrimitiveType::Int { signed: true })
                            } else if *val <= i64::MAX {
                                TypeInfo::new(PrimitiveType::Long { signed: true })
                            } else {
                                TypeInfo::new(PrimitiveType::LongLong { signed: true })
                            }
                        } else {
                            if *val >= i32::MIN as i64 {
                                TypeInfo::new(PrimitiveType::Int { signed: true })
                            } else if *val >= i64::MIN {
                                TypeInfo::new(PrimitiveType::Long { signed: true })
                            } else {
                                TypeInfo::new(PrimitiveType::LongLong { signed: true })
                            }
                        }
                    }
                    IntegerType::Signed(_) => TypeInfo::new(PrimitiveType::Int { signed: true }),
                    IntegerType::SignedLong(_) => {
                        TypeInfo::new(PrimitiveType::Long { signed: true })
                    }
                    IntegerType::SignedLongLong(_) => {
                        TypeInfo::new(PrimitiveType::LongLong { signed: true })
                    }
                    IntegerType::Unsigned(_) => TypeInfo::new(PrimitiveType::Int { signed: false }),
                    IntegerType::UnsignedLong(_) => {
                        TypeInfo::new(PrimitiveType::Long { signed: false })
                    }
                    IntegerType::UnsignedLongLong(_) => {
                        TypeInfo::new(PrimitiveType::LongLong { signed: false })
                    }
                }
            }
            Constant::Float(float_type) => match float_type {
                FloatingPointType::Float(_) => TypeInfo::new(PrimitiveType::Float),
                FloatingPointType::Double(_) => TypeInfo::new(PrimitiveType::Double),
                FloatingPointType::LongDouble(_) => TypeInfo::new(PrimitiveType::LongDouble),
            },
            Constant::Character(_) => TypeInfo::new(PrimitiveType::Char { signed: true }),
        }
    }

    // Check if this type can represent the given constant
    pub fn can_represent(&self, constant: &Constant) -> bool {
        match (self, constant) {
            (PrimitiveType::Int { signed }, Constant::Integer(IntegerType::Generic(val))) => {
                if *signed {
                    *val >= i32::MIN as i64 && *val <= i32::MAX as i64
                } else {
                    *val >= 0 && *val <= u32::MAX as i64
                }
            }
            // Add more cases for other type combinations
            _ => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct TypeQualifiers {
    pub is_const: bool,
    pub is_volatile: bool,
    pub is_restrict: bool,
    pub is_atomic: bool,
}

pub enum TypeCompatibility {
    Identical,
    Compatible,
    ImplicitConversion {
        source: TypeInfo,
        target: TypeInfo,
        potential_data_loss: bool,
    },
    Incompatible,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct TypeInfo {
    pub base_type: PrimitiveType,
    pub qualifiers: TypeQualifiers,
}

impl TypeInfo {
    pub fn new(base_type: PrimitiveType) -> Self {
        TypeInfo {
            base_type,
            qualifiers: TypeQualifiers::default(),
        }
    }

    pub fn from_typename(typename: TypeName) -> Self {
        for specqual in &typename.specifier_qualifier_list {
            println!("{:?}", specqual);
        }

        todo!()
    }

    pub fn from_declaration_specifiers(
        declaration_specifiers: &[Node<DeclarationSpecifier>],
    ) -> Result<Self, CompilerError> {
        let mut signed_keyword = false;
        let mut unsigned_keyword = false;
        let mut typeinfo = Self::default();

        for (i, decl_spec) in declaration_specifiers.iter().enumerate() {
            match &decl_spec.node {
                DeclarationSpecifier::TypeQualifier(qualifier) => match qualifier {
                    TypeQualifier::Const => typeinfo.qualifiers.is_const = true,
                    TypeQualifier::Volatile => typeinfo.qualifiers.is_volatile = true,
                    TypeQualifier::Atomic => typeinfo.qualifiers.is_atomic = true,
                    TypeQualifier::Restrict => typeinfo.qualifiers.is_restrict = true,
                },
                DeclarationSpecifier::TypeSpecifier(specifier) => match specifier {
                    TypeSpecifier::Signed => {
                        // Disallow multiple signed keywords
                        if signed_keyword || unsigned_keyword {
                            return Err(CompilerError {
                                kind: CompilerErrorKind::SemanticError,
                                message: "Cannot combine `signed` keyword with previous declaration specifier".to_string(),
                                location: Some(decl_spec.span.start),
                            });
                        }
                        signed_keyword = true;
                    }
                    TypeSpecifier::Unsigned => {
                        // Disallow multiple unsigned keywords
                        if unsigned_keyword || signed_keyword {
                            return Err(CompilerError {
                                kind: CompilerErrorKind::SemanticError,
                                message: "Cannot combine `unsigned` keyword with previous declaration specifier".to_string(),
                                location: Some(decl_spec.span.start),
                            });
                        }
                        unsigned_keyword = true;
                    }

                    _ => {
                        match specifier {
                            TypeSpecifier::Void => typeinfo.base_type = PrimitiveType::Void,
                            TypeSpecifier::Bool => typeinfo.base_type = PrimitiveType::Bool,
                            TypeSpecifier::Char => {
                                typeinfo.base_type = PrimitiveType::Char {
                                    signed: !unsigned_keyword,
                                }
                            }
                            TypeSpecifier::Short => {
                                typeinfo.base_type = PrimitiveType::Short {
                                    signed: !unsigned_keyword,
                                }
                            }
                            TypeSpecifier::Int => {
                                typeinfo.base_type = PrimitiveType::Int {
                                    signed: !unsigned_keyword,
                                }
                            }
                            TypeSpecifier::Long => {
                                typeinfo.base_type = PrimitiveType::Long {
                                    signed: !unsigned_keyword,
                                }
                            }
                            TypeSpecifier::Float => typeinfo.base_type = PrimitiveType::Float,
                            TypeSpecifier::Double => typeinfo.base_type = PrimitiveType::Double,

                            // This is never possible so basically dead code
                            TypeSpecifier::Signed | TypeSpecifier::Unsigned => unreachable!(),

                            // Yet to handle type specifiers like Complex
                            _ => todo!(),
                        }

                        if i != declaration_specifiers.len() - 1 {
                            return Err(CompilerError {
                                kind: CompilerErrorKind::SemanticError,
                                message: format!("Unexpected declaration specifier after the base type specifier: {:?}", typeinfo.base_type),
                                location: Some(declaration_specifiers[i + 1].span.start) 
                            })  
                        }
                        else {
                            return Ok(typeinfo);
                        }
                    }
                },
                _ => todo!(),
            }
        }

        Err(CompilerError {
            kind: CompilerErrorKind::SemanticError,
            message: "Missing primitive type specifier".to_string(),
            location: Some(declaration_specifiers.first().unwrap().span.start)
        })
    }

    pub fn compare(x: &TypeInfo, y: &TypeInfo) -> TypeCompatibility {
        if x == y {
            TypeCompatibility::Identical
        } else {
            // Check if the types are compatible or not
            // TODO: Implement checking for possible compatibility and implicit conversions, but
            // for now the types must be exactly identical.
            TypeCompatibility::Incompatible
        }
    }

    // Helper method to create a pointer to this type
    pub fn pointer_to(self) -> TypeInfo {
        TypeInfo {
            base_type: PrimitiveType::Pointer {
                inner: Box::new(self),
            },
            qualifiers: TypeQualifiers::default(),
        }
    }
}


// ----------------------------------------- Display Implementations for the above structs -----------------------------------------

impl fmt::Display for TypeQualifiers {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut qualifiers = vec![];
        if self.is_const {
            qualifiers.push("const");
        }
        if self.is_volatile {
            qualifiers.push("volatile");
        }
        if self.is_restrict {
            qualifiers.push("restrict");
        }
        if self.is_atomic {
            qualifiers.push("atomic");
        }
        write!(f, "{}", qualifiers.join(" "))
    }
}

impl fmt::Display for PrimitiveType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PrimitiveType::Void => write!(f, "void"),
            PrimitiveType::Bool => write!(f, "bool"),
            PrimitiveType::Char { signed } => write!(f, "{}char", if *signed { "signed " } else { "unsigned " }),
            PrimitiveType::Short { signed } => write!(f, "{}short", if *signed { "signed " } else { "unsigned " }),
            PrimitiveType::Int { signed } => write!(f, "{}int", if *signed { "signed " } else { "unsigned " }),
            PrimitiveType::Long { signed } => write!(f, "{}long", if *signed { "signed " } else { "unsigned " }),
            PrimitiveType::LongLong { signed } => write!(f, "{}long long", if *signed { "signed " } else { "unsigned " }),
            PrimitiveType::Float => write!(f, "float"),
            PrimitiveType::Double => write!(f, "double"),
            PrimitiveType::LongDouble => write!(f, "long double"),
            PrimitiveType::Pointer { inner } => write!(f, "*{}", inner),
            PrimitiveType::Array { element_type, size } => match size {
                Some(s) => write!(f, "{}[{}]", element_type, s),
                None => write!(f, "{}[]", element_type),
            },
            PrimitiveType::Function { return_type, parameters, is_variadic } => {
                let params: Vec<String> = parameters.iter().map(|p| format!("{}", p)).collect();
                let params_str = if *is_variadic {
                    format!("{}, ...", params.join(", "))
                } else {
                    params.join(", ")
                };
                write!(f, "{}({})", return_type, params_str)
            }
            PrimitiveType::Struct { name, fields } => {
                let fields_str: Vec<String> = fields.iter().map(|(n, t)| format!("{}: {}", n, t)).collect();
                write!(f, "struct {} {{ {} }}", name, fields_str.join("; "))
            }
            PrimitiveType::Union { name, fields } => {
                let fields_str: Vec<String> = fields.iter().map(|(n, t)| format!("{}: {}", n, t)).collect();
                write!(f, "union {} {{ {} }}", name, fields_str.join("; "))
            }
            PrimitiveType::Enum { name, underlying_type } => write!(f, "enum {} : {}", name, underlying_type),
            PrimitiveType::Typedef { name, actual_type } => write!(f, "typedef {} = {}", name, actual_type),
        }
    }
}

impl fmt::Display for TypeInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.qualifiers.is_const || self.qualifiers.is_volatile {
            write!(f, "{} {}", self.qualifiers, self.base_type)
        } else {
            write!(f, "{}", self.base_type)
        }
    }
}
