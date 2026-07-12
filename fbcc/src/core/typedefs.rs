use std::fmt;

use crate::analysis::ast::{DeclarationSpecifier, StorageClassFlags, TypeName, TypeQualifier, TypeSpecifier};
use crate::analysis::node::Node;
use crate::core::errors::{CompilerError, CompilerErrorKind};

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
pub enum DataType {
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

impl DataType {
    // Check if this type can represent the given constant
    pub fn can_represent(&self, constant: &Constant) -> bool {
        match (self, constant) {
            (DataType::Int { signed }, Constant::Integer(IntegerType::Generic(val))) => {
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
            DataType::Bool
                | DataType::Char { .. }
                | DataType::Short { .. }
                | DataType::Int { .. }
                | DataType::Long { .. }
                | DataType::LongLong { .. }
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
    ImplicitConversion { datatype: DataType },
    Incompatible,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct Type {
    pub datatype: DataType,
    pub qualifiers: TypeQualifiers,
}

impl Type {
    pub fn new(datatype: DataType) -> Self {
        Type {
            datatype,
            qualifiers: TypeQualifiers::default(),
        }
    }

    pub fn from_typename(typename: TypeName) -> Self {
        for specqual in &typename.specifier_qualifier_list {
            println!("{:?}", specqual);
        }

        todo!()
    }

    pub fn size(&self) -> Result<usize, CompilerError> {
        let size = match &self.datatype {
            DataType::Void => 0,
            DataType::Bool => 1,
            DataType::Char { .. } => 1,
            DataType::Short { .. } => 2,
            DataType::Int { .. } => 4,
            DataType::Long { .. } => 8,
            DataType::LongLong { .. } => 8,
            DataType::Float => 4,
            DataType::Double => 8,
            DataType::LongDouble => 8, // Apple arm64: long double == double
            DataType::Pointer { .. } => 8,
            other => {
                return Err(CompilerError {
                    kind: CompilerErrorKind::InternalError,
                    message: format!("size() is not yet implemented for type: {other:?}"),
                    location: None,
                });
            }
        };
        Ok(size)
    }

    pub fn align(&self) -> Result<usize, CompilerError> {
        let size = match &self.datatype {
            DataType::Void => 0,
            DataType::Bool => 1,
            DataType::Char { .. } => 1,
            DataType::Short { .. } => 2,
            DataType::Int { .. } => 4,
            DataType::Long { .. } => 8,
            DataType::LongLong { .. } => 8,
            DataType::Float => 4,
            DataType::Double => 8,
            DataType::LongDouble => 8, // Apple arm64: long double == double
            DataType::Pointer { .. } => 8,
            other => {
                return Err(CompilerError {
                    kind: CompilerErrorKind::InternalError,
                    message: format!("alignment() is not yet implemented for type: {other:?}"),
                    location: None,
                });
            }
        };
        Ok(size)
    }

    pub fn from_declaration_specifiers(
        declaration_specifiers: &[Node<DeclarationSpecifier>],
    ) -> Result<(Self, StorageClassFlags), CompilerError> {
        let mut signed_keyword = false;
        let mut unsigned_keyword = false;
        let mut data_type_encountered = false;
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
                        // This means that data type keyword like int, float, double, char has been
                        // encountered, because without this the type is incomplete
                        data_type_encountered = true;

                        match specifier {
                            TypeSpecifier::Void => typeinfo.datatype = DataType::Void,
                            TypeSpecifier::Bool => typeinfo.datatype = DataType::Bool,
                            TypeSpecifier::Char => {
                                typeinfo.datatype = DataType::Char {
                                    signed: !unsigned_keyword,
                                }
                            }
                            TypeSpecifier::Short => {
                                typeinfo.datatype = DataType::Short {
                                    signed: !unsigned_keyword,
                                }
                            }
                            TypeSpecifier::Float => typeinfo.datatype = DataType::Float,

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
                                        typeinfo.datatype = DataType::Int {
                                            signed: !unsigned_keyword,
                                        }
                                    }

                                    TypeSpecifier::Double => {
                                        is_double = true;
                                        typeinfo.datatype = DataType::Double
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
            typeinfo.datatype = match long_count {
                0 => {
                    if is_double {
                        DataType::Double // double x;
                    } else {
                        DataType::Int {
                            signed: !unsigned_keyword,
                        } // int x;
                    }
                }
                1 => {
                    if is_double {
                        DataType::LongDouble // long double x;
                    } else {
                        DataType::Long {
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
                        DataType::LongLong {
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

        if data_type_encountered {
            Ok((typeinfo, storageclass))
        } else {
            Err(CompilerError {
                kind: CompilerErrorKind::SemanticError,
                message: "Missing primitive type specifier".to_string(),
                location: Some(declaration_specifiers.first().unwrap().span.start),
            })
        }
    }

    pub fn from_constant(constant: &Constant) -> Type {
        match constant {
            Constant::Integer(int_type) => {
                match int_type {
                    IntegerType::Generic(val) => {
                        // Infer the smallest type that can hold the value
                        if *val >= 0 {
                            if *val <= i32::MAX as i64 {
                                Type::new(DataType::Int { signed: true })
                            } else if *val <= i64::MAX {
                                Type::new(DataType::Long { signed: true })
                            } else {
                                Type::new(DataType::LongLong { signed: true })
                            }
                        } else {
                            if *val >= i32::MIN as i64 {
                                Type::new(DataType::Int { signed: true })
                            } else if *val >= i64::MIN {
                                Type::new(DataType::Long { signed: true })
                            } else {
                                Type::new(DataType::LongLong { signed: true })
                            }
                        }
                    }
                    IntegerType::Signed(_) => Type::new(DataType::Int { signed: true }),
                    IntegerType::SignedLong(_) => Type::new(DataType::Long { signed: true }),
                    IntegerType::SignedLongLong(_) => Type::new(DataType::LongLong { signed: true }),
                    IntegerType::Unsigned(_) => Type::new(DataType::Int { signed: false }),
                    IntegerType::UnsignedLong(_) => Type::new(DataType::Long { signed: false }),
                    IntegerType::UnsignedLongLong(_) => Type::new(DataType::LongLong { signed: false }),
                }
            }
            Constant::Float(float_type) => match float_type {
                FloatingPointType::Float(_) => Type::new(DataType::Float),
                FloatingPointType::Double(_) => Type::new(DataType::Double),
                FloatingPointType::LongDouble(_) => Type::new(DataType::LongDouble),
            },
            Constant::Character(_) => Type::new(DataType::Char { signed: true }),
        }
    }

    pub fn compare(x: &Type, y: &Type) -> TypeCompatibility {
        if x == y {
            TypeCompatibility::Identical
        } else {
            // Check if the types are compatible or not --------------------------------------------------
            // Rule 1: If one of the types x, y is a pointer to a type T, and other is an array to the
            // same type T, then x and y are compatible
            if let (DataType::Pointer { inner: typeinfo }, DataType::Array { element_type, size: _ })
            | (DataType::Array { element_type, size: _ }, DataType::Pointer { inner: typeinfo }) =
                (&x.datatype, &y.datatype)
            {
                if typeinfo == element_type {
                    return TypeCompatibility::Compatible;
                }
            }

            // Check if types are can be converted implicitly to match each other ------------------------
            // Type Promotion Rules
            if x.datatype == DataType::LongDouble || y.datatype == DataType::LongDouble {
                return TypeCompatibility::ImplicitConversion {
                    datatype: DataType::LongDouble,
                };
            }

            if x.datatype == DataType::Double || y.datatype == DataType::Double {
                return TypeCompatibility::ImplicitConversion {
                    datatype: DataType::Double,
                };
            }

            if x.datatype == DataType::Float || y.datatype == DataType::Float {
                return TypeCompatibility::ImplicitConversion {
                    datatype: DataType::Float,
                };
            }

            if x.datatype.is_integer_type() && y.datatype.is_integer_type() {
                // Integer Promotion Rules
                match (&x.datatype, &y.datatype) {
                    // This statement matches signed long long with unsigned long long and promotes
                    // ...expression to unsigned long long
                    (DataType::LongLong { signed: true }, DataType::LongLong { signed: false })
                    | (DataType::LongLong { signed: false }, DataType::LongLong { signed: true }) => {
                        TypeCompatibility::ImplicitConversion {
                            datatype: DataType::LongLong { signed: false },
                        }
                    }

                    // This statement matches signed long with unsigned long and promotes
                    // ...expression to unsigned long
                    (DataType::Long { signed: true }, DataType::Long { signed: false })
                    | (DataType::Long { signed: false }, DataType::Long { signed: true }) => {
                        TypeCompatibility::ImplicitConversion {
                            datatype: DataType::Long { signed: false },
                        }
                    }

                    // This statement matches signed int with unsigned int and promotes
                    // ...expression to unsigned int
                    (DataType::Int { signed: true }, DataType::Int { signed: false })
                    | (DataType::Int { signed: false }, DataType::Int { signed: true }) => {
                        TypeCompatibility::ImplicitConversion {
                            datatype: DataType::Int { signed: false },
                        }
                    }

                    // NOTE: Big flaw this has is that _ matches with all other types, that means if
                    // two types are compared where one being let's say a function and other being
                    // a float, then they are labelled as convertible to each other using a cast, which
                    // is wrong.

                    // This statement matches long long with any lower type and promotes
                    // ...expression to long long
                    (DataType::LongLong { signed }, _) | (_, DataType::LongLong { signed }) => {
                        TypeCompatibility::ImplicitConversion {
                            datatype: DataType::LongLong { signed: *signed },
                        }
                    }

                    // This statement matches long with any lower type and promotes
                    // ...expression to long
                    (DataType::Long { signed }, _) | (_, DataType::Long { signed }) => {
                        TypeCompatibility::ImplicitConversion {
                            datatype: DataType::Long { signed: *signed },
                        }
                    }

                    // This statement matches int with any lower type and promotes
                    // ...expression to int
                    (DataType::Int { signed }, _) | (_, DataType::Int { signed }) => {
                        TypeCompatibility::ImplicitConversion {
                            datatype: DataType::Int { signed: *signed },
                        }
                    }

                    // This statement promotes any type combinations which are below int to int
                    (DataType::Char { signed }, _)
                    | (_, DataType::Char { signed })
                    | (DataType::Short { signed }, _)
                    | (_, DataType::Short { signed }) => TypeCompatibility::ImplicitConversion {
                        datatype: DataType::Int { signed: *signed },
                    },

                    // No operation can be done on void types without explicitly casting them
                    (DataType::Void, _) | (_, DataType::Void) => TypeCompatibility::Incompatible,

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
            datatype: DataType::Pointer { inner: Box::new(self) },
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

impl fmt::Display for DataType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DataType::Void => write!(f, "void"),
            DataType::Bool => write!(f, "bool"),
            DataType::Char { signed } => {
                write!(f, "{}char", if *signed { "signed " } else { "unsigned " })
            }
            DataType::Short { signed } => {
                write!(f, "{}short", if *signed { "signed " } else { "unsigned " })
            }
            DataType::Int { signed } => {
                write!(f, "{}int", if *signed { "signed " } else { "unsigned " })
            }
            DataType::Long { signed } => {
                write!(f, "{}long", if *signed { "signed " } else { "unsigned " })
            }
            DataType::LongLong { signed } => {
                write!(f, "{}long long", if *signed { "signed " } else { "unsigned " })
            }
            DataType::Float => write!(f, "float"),
            DataType::Double => write!(f, "double"),
            DataType::LongDouble => write!(f, "long double"),
            DataType::Pointer { inner } => write!(f, "*{}", inner),
            DataType::Array { element_type, size } => match size {
                Some(s) => write!(f, "{}[{}]", element_type, s),
                None => write!(f, "{}[]", element_type),
            },
            DataType::Function {
                return_type,
                parameters,
            } => {
                let params: Vec<String> = parameters.iter().map(|p| format!("{}", p)).collect();
                write!(f, "{}({})", return_type, params.join(", "))
            }
            DataType::Struct { name, fields } => {
                let fields_str: Vec<String> = fields.iter().map(|(n, t)| format!("{}: {}", n, t)).collect();
                write!(f, "struct {} {{ {} }}", name, fields_str.join("; "))
            }
            DataType::Union { name, fields } => {
                let fields_str: Vec<String> = fields.iter().map(|(n, t)| format!("{}: {}", n, t)).collect();
                write!(f, "union {} {{ {} }}", name, fields_str.join("; "))
            }
            DataType::Enum { name, underlying_type } => write!(f, "enum {} : {}", name, underlying_type),
            DataType::Typedef { name, actual_type } => {
                write!(f, "typedef {} = {}", name, actual_type)
            }
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.qualifiers.is_const || self.qualifiers.is_volatile {
            write!(f, "{} {}", self.qualifiers, self.datatype)
        } else {
            write!(f, "{}", self.datatype)
        }
    }
}
