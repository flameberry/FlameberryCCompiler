use std::fmt;

use crate::analysis::ast::{DeclarationSpecifier, StorageClassFlags, TypeName, TypeQualifier, TypeSpecifier};
use crate::analysis::node::Node;
use crate::common::errors::{CompilerError, CompilerErrorKind};

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
pub enum BaseType {
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
        inner: Box<Type>,
    },
    Array {
        element_type: Box<Type>,
        size: Option<usize>,
    },
    Function {
        return_type: Box<Type>,
        parameters: Vec<Type>,
    },
    Struct {
        name: String,
        fields: Vec<(String, Type)>,
    },
    Union {
        name: String,
        fields: Vec<(String, Type)>,
    },
    Enum {
        name: String,
        underlying_type: Box<Type>,
    },
    Typedef {
        name: String,
        actual_type: Box<Type>,
    },
}

impl BaseType {
    /// Get the type for a constant value
    pub fn from_constant(constant: &Constant) -> Type {
        match constant {
            Constant::Integer(int_type) => {
                match int_type {
                    IntegerType::Generic(val) => {
                        // Infer the smallest type that can hold the value
                        if *val >= 0 {
                            if *val <= i32::MAX as i64 {
                                Type::new(BaseType::Int { signed: true })
                            } else if *val <= i64::MAX {
                                Type::new(BaseType::Long { signed: true })
                            } else {
                                Type::new(BaseType::LongLong { signed: true })
                            }
                        } else {
                            if *val >= i32::MIN as i64 {
                                Type::new(BaseType::Int { signed: true })
                            } else if *val >= i64::MIN {
                                Type::new(BaseType::Long { signed: true })
                            } else {
                                Type::new(BaseType::LongLong { signed: true })
                            }
                        }
                    }
                    IntegerType::Signed(_) => Type::new(BaseType::Int { signed: true }),
                    IntegerType::SignedLong(_) => Type::new(BaseType::Long { signed: true }),
                    IntegerType::SignedLongLong(_) => Type::new(BaseType::LongLong { signed: true }),
                    IntegerType::Unsigned(_) => Type::new(BaseType::Int { signed: false }),
                    IntegerType::UnsignedLong(_) => Type::new(BaseType::Long { signed: false }),
                    IntegerType::UnsignedLongLong(_) => Type::new(BaseType::LongLong { signed: false }),
                }
            }
            Constant::Float(float_type) => match float_type {
                FloatingPointType::Float(_) => Type::new(BaseType::Float),
                FloatingPointType::Double(_) => Type::new(BaseType::Double),
                FloatingPointType::LongDouble(_) => Type::new(BaseType::LongDouble),
            },
            Constant::Character(_) => Type::new(BaseType::Char { signed: true }),
        }
    }

    // Check if this type can represent the given constant
    pub fn can_represent(&self, constant: &Constant) -> bool {
        match (self, constant) {
            (BaseType::Int { signed }, Constant::Integer(IntegerType::Generic(val))) => {
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

    fn is_integer_type(&self) -> bool {
        matches!(
            self,
            BaseType::Bool
                | BaseType::Char { .. }
                | BaseType::Short { .. }
                | BaseType::Int { .. }
                | BaseType::Long { .. }
                | BaseType::LongLong { .. }
        )
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
    ImplicitConversion { base_type: BaseType },
    Incompatible,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct Type {
    pub base_type: BaseType,
    pub qualifiers: TypeQualifiers,
}

impl Type {
    pub fn new(base_type: BaseType) -> Self {
        Type {
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
    ) -> Result<(Self, StorageClassFlags), CompilerError> {
        let mut signed_keyword = false;
        let mut unsigned_keyword = false;
        let mut base_type_encountered = false;
        let mut typeinfo = Self::default();
        let mut storageclass: u8 = 0;
        let mut is_type_long_compatible = false;
        let mut long_count = 0;
        let mut is_double = false;

        for decl_spec in declaration_specifiers.iter() {
            match &decl_spec.node {
                DeclarationSpecifier::TypeQualifier(qualifier) => match qualifier {
                    TypeQualifier::Const => typeinfo.qualifiers.is_const = true,
                    TypeQualifier::Volatile => typeinfo.qualifiers.is_volatile = true,
                    TypeQualifier::Atomic => typeinfo.qualifiers.is_atomic = true,
                    TypeQualifier::Restrict => typeinfo.qualifiers.is_restrict = true,
                },
                DeclarationSpecifier::StorageClassSpecifier(storage_class_specifier) => {
                    storageclass |= *storage_class_specifier as u8;
                }
                DeclarationSpecifier::TypeSpecifier(specifier) => match specifier {
                    TypeSpecifier::Signed => {
                        // Disallow multiple signed keywords
                        if signed_keyword || unsigned_keyword {
                            return Err(CompilerError {
                                kind: CompilerErrorKind::SemanticError,
                                message: "Cannot combine `signed` keyword with previous declaration specifier"
                                    .to_string(),
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
                                message: "Cannot combine `unsigned` keyword with previous declaration specifier"
                                    .to_string(),
                                location: Some(decl_spec.span.start),
                            });
                        }
                        unsigned_keyword = true;
                    }

                    _ => {
                        // This means that base type keyword like int, float, double, char has been
                        // encountered, because without this the type is incomplete
                        base_type_encountered = true;

                        match specifier {
                            TypeSpecifier::Void => typeinfo.base_type = BaseType::Void,
                            TypeSpecifier::Bool => typeinfo.base_type = BaseType::Bool,
                            TypeSpecifier::Char => {
                                typeinfo.base_type = BaseType::Char {
                                    signed: !unsigned_keyword,
                                }
                            }
                            TypeSpecifier::Short => {
                                typeinfo.base_type = BaseType::Short {
                                    signed: !unsigned_keyword,
                                }
                            }
                            TypeSpecifier::Float => typeinfo.base_type = BaseType::Float,

                            // This is never possible so basically dead code
                            TypeSpecifier::Signed | TypeSpecifier::Unsigned => unreachable!(),

                            // Yet to handle type specifiers like Complex
                            TypeSpecifier::Complex => todo!(),

                            // Long Compatible types are handled specially as there are different
                            // variations to long it which are allowed by C language like having:
                            // int long long var;
                            // long int long var;
                            // long long int var;
                            TypeSpecifier::Int | TypeSpecifier::Long | TypeSpecifier::Double => {
                                is_type_long_compatible = true;

                                match specifier {
                                    TypeSpecifier::Long => long_count += 1,

                                    TypeSpecifier::Int => {
                                        typeinfo.base_type = BaseType::Int {
                                            signed: !unsigned_keyword,
                                        }
                                    }

                                    TypeSpecifier::Double => {
                                        is_double = true;
                                        typeinfo.base_type = BaseType::Double
                                    }
                                    _ => unreachable!(),
                                }
                            }
                        }
                    }
                },
                _ => todo!(),
            }
        }

        if is_type_long_compatible {
            typeinfo.base_type = match long_count {
                0 => {
                    if is_double {
                        BaseType::Double // double x;
                    } else {
                        BaseType::Int {
                            signed: !unsigned_keyword,
                        } // int x;
                    }
                }
                1 => {
                    if is_double {
                        BaseType::LongDouble // long double x;
                    } else {
                        BaseType::Long {
                            signed: !unsigned_keyword,
                        } // long int x;
                    }
                }
                2 => {
                    if is_double {
                        return Err(CompilerError {
                            kind: CompilerErrorKind::SemanticError,
                            message: "long long double is an invalid type.".to_string(),
                            location: Some(declaration_specifiers.first().unwrap().span.start),
                        }); // long long double x; <-- Not Allowed
                    } else {
                        BaseType::LongLong {
                            signed: !unsigned_keyword,
                        } // long long int x;
                    }
                }
                _ => {
                    return Err(CompilerError {
                        kind: CompilerErrorKind::SemanticError,
                        message: "Invalid declaration containing more than 2 long specifiers.".to_string(),
                        location: Some(declaration_specifiers.first().unwrap().span.start),
                    }); // long long long x;
                }
            }
        }

        if base_type_encountered {
            Ok((typeinfo, storageclass))
        } else {
            Err(CompilerError {
                kind: CompilerErrorKind::SemanticError,
                message: "Missing primitive type specifier".to_string(),
                location: Some(declaration_specifiers.first().unwrap().span.start),
            })
        }
    }

    pub fn compare(x: &Type, y: &Type) -> TypeCompatibility {
        if x == y {
            TypeCompatibility::Identical
        } else {
            // Check if the types are compatible or not --------------------------------------------------
            // Rule 1: If one of the types x, y is a pointer to a type T, and other is an array to the
            // same type T, then x and y are compatible
            if let (BaseType::Pointer { inner: typeinfo }, BaseType::Array { element_type, size: _ })
            | (BaseType::Array { element_type, size: _ }, BaseType::Pointer { inner: typeinfo }) =
                (&x.base_type, &y.base_type)
            {
                if typeinfo == element_type {
                    return TypeCompatibility::Compatible;
                }
            }

            // Check if types are can be converted implicitly to match each other ------------------------
            // Type Promotion Rules
            if x.base_type == BaseType::LongDouble || y.base_type == BaseType::LongDouble {
                return TypeCompatibility::ImplicitConversion {
                    base_type: BaseType::LongDouble,
                };
            }

            if x.base_type == BaseType::Double || y.base_type == BaseType::Double {
                return TypeCompatibility::ImplicitConversion {
                    base_type: BaseType::Double,
                };
            }

            if x.base_type == BaseType::Float || y.base_type == BaseType::Float {
                return TypeCompatibility::ImplicitConversion {
                    base_type: BaseType::Float,
                };
            }

            if x.base_type.is_integer_type() && y.base_type.is_integer_type() {
                // Integer Promotion Rules
                match (&x.base_type, &y.base_type) {
                    // This statement matches signed long long with unsigned long long and promotes
                    // ...expression to unsigned long long
                    (BaseType::LongLong { signed: true }, BaseType::LongLong { signed: false })
                    | (BaseType::LongLong { signed: false }, BaseType::LongLong { signed: true }) => {
                        TypeCompatibility::ImplicitConversion {
                            base_type: BaseType::LongLong { signed: false },
                        }
                    }

                    // This statement matches signed long with unsigned long and promotes
                    // ...expression to unsigned long
                    (BaseType::Long { signed: true }, BaseType::Long { signed: false })
                    | (BaseType::Long { signed: false }, BaseType::Long { signed: true }) => {
                        TypeCompatibility::ImplicitConversion {
                            base_type: BaseType::Long { signed: false },
                        }
                    }

                    // This statement matches signed int with unsigned int and promotes
                    // ...expression to unsigned int
                    (BaseType::Int { signed: true }, BaseType::Int { signed: false })
                    | (BaseType::Int { signed: false }, BaseType::Int { signed: true }) => {
                        TypeCompatibility::ImplicitConversion {
                            base_type: BaseType::Int { signed: false },
                        }
                    }

                    // NOTE: Big flaw this has is that _ matches with all other types, that means if
                    // two types are compared where one being let's say a function and other being
                    // a float, then they are labelled as convertible to each other using a cast, which
                    // is wrong.

                    // This statement matches long long with any lower type and promotes
                    // ...expression to long long
                    (BaseType::LongLong { signed }, _) | (_, BaseType::LongLong { signed }) => {
                        TypeCompatibility::ImplicitConversion {
                            base_type: BaseType::LongLong { signed: *signed },
                        }
                    }

                    // This statement matches long with any lower type and promotes
                    // ...expression to long
                    (BaseType::Long { signed }, _) | (_, BaseType::Long { signed }) => {
                        TypeCompatibility::ImplicitConversion {
                            base_type: BaseType::Long { signed: *signed },
                        }
                    }

                    // This statement matches int with any lower type and promotes
                    // ...expression to int
                    (BaseType::Int { signed }, _) | (_, BaseType::Int { signed }) => {
                        TypeCompatibility::ImplicitConversion {
                            base_type: BaseType::Int { signed: *signed },
                        }
                    }

                    // This statement promotes any type combinations which are below int to int
                    (BaseType::Char { signed }, _)
                    | (_, BaseType::Char { signed })
                    | (BaseType::Short { signed }, _)
                    | (_, BaseType::Short { signed }) => TypeCompatibility::ImplicitConversion {
                        base_type: BaseType::Int { signed: *signed },
                    },

                    // No operation can be done on void types without explicitly casting them
                    (BaseType::Void, _) | (_, BaseType::Void) => TypeCompatibility::Incompatible,

                    // According to the current implementation the types are not compatible ----------------------
                    _ => unreachable!(),
                }
            } else {
                TypeCompatibility::Incompatible
            }
        }
    }

    // Helper method to create a pointer to this type
    pub fn pointer_to(self) -> Type {
        Type {
            base_type: BaseType::Pointer { inner: Box::new(self) },
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

impl fmt::Display for BaseType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BaseType::Void => write!(f, "void"),
            BaseType::Bool => write!(f, "bool"),
            BaseType::Char { signed } => {
                write!(f, "{}char", if *signed { "signed " } else { "unsigned " })
            }
            BaseType::Short { signed } => {
                write!(f, "{}short", if *signed { "signed " } else { "unsigned " })
            }
            BaseType::Int { signed } => {
                write!(f, "{}int", if *signed { "signed " } else { "unsigned " })
            }
            BaseType::Long { signed } => {
                write!(f, "{}long", if *signed { "signed " } else { "unsigned " })
            }
            BaseType::LongLong { signed } => {
                write!(f, "{}long long", if *signed { "signed " } else { "unsigned " })
            }
            BaseType::Float => write!(f, "float"),
            BaseType::Double => write!(f, "double"),
            BaseType::LongDouble => write!(f, "long double"),
            BaseType::Pointer { inner } => write!(f, "*{}", inner),
            BaseType::Array { element_type, size } => match size {
                Some(s) => write!(f, "{}[{}]", element_type, s),
                None => write!(f, "{}[]", element_type),
            },
            BaseType::Function {
                return_type,
                parameters,
            } => {
                let params: Vec<String> = parameters.iter().map(|p| format!("{}", p)).collect();
                write!(f, "{}({})", return_type, params.join(", "))
            }
            BaseType::Struct { name, fields } => {
                let fields_str: Vec<String> = fields.iter().map(|(n, t)| format!("{}: {}", n, t)).collect();
                write!(f, "struct {} {{ {} }}", name, fields_str.join("; "))
            }
            BaseType::Union { name, fields } => {
                let fields_str: Vec<String> = fields.iter().map(|(n, t)| format!("{}: {}", n, t)).collect();
                write!(f, "union {} {{ {} }}", name, fields_str.join("; "))
            }
            BaseType::Enum { name, underlying_type } => write!(f, "enum {} : {}", name, underlying_type),
            BaseType::Typedef { name, actual_type } => {
                write!(f, "typedef {} = {}", name, actual_type)
            }
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.qualifiers.is_const || self.qualifiers.is_volatile {
            write!(f, "{} {}", self.qualifiers, self.base_type)
        } else {
            write!(f, "{}", self.base_type)
        }
    }
}
