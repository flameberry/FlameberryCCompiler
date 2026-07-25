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

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum IntegerRank {
    Bool,
    Char,
    Short,
    Int,
    Long,
    LongLong,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub enum DataType {
    #[default]
    Void,

    Integer {
        rank: IntegerRank,
        signed: bool,
    },

    Float,
    Double,
    LongDouble,

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

#[derive(Debug)]
pub enum AssignmentConversionResult {
    Identical, // source already has target type — insert nothing
    Cast,      // legal but needs an ImplicitCast to `target`

    // this way of casting can alter the behaviour of the program
    // ...hence a warning to the user must be given
    CastWithWarning(String),
}

impl DataType {
    pub fn new_integer(rank: IntegerRank, signed: bool) -> Self {
        Self::Integer { rank, signed: signed }
    }

    pub fn is_integer(&self) -> bool {
        matches!(self, DataType::Integer { .. })
    }

    pub fn is_arithmetic(&self) -> bool {
        self.is_integer() || matches!(self, DataType::Float | DataType::Double | DataType::LongDouble)
    }

    pub fn is_scalar(&self) -> bool {
        self.is_arithmetic() || matches!(self, DataType::Pointer { .. })
    }

    pub fn is_pointer(&self) -> bool {
        matches!(self, DataType::Pointer { .. })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct TypeQualifiers {
    pub is_const: bool,
    pub is_volatile: bool,
    pub is_restrict: bool,
    pub is_atomic: bool,
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
            DataType::Integer {
                rank: subtype,
                signed: _,
            } => match subtype {
                IntegerRank::Bool => 1,
                IntegerRank::Char { .. } => 1,
                IntegerRank::Short { .. } => 2,
                IntegerRank::Int { .. } => 4,
                IntegerRank::Long { .. } => 8,
                IntegerRank::LongLong { .. } => 8,
            },
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
            DataType::Integer {
                rank: subtype,
                signed: _,
            } => match subtype {
                IntegerRank::Bool => 1,
                IntegerRank::Char { .. } => 1,
                IntegerRank::Short { .. } => 2,
                IntegerRank::Int { .. } => 4,
                IntegerRank::Long { .. } => 8,
                IntegerRank::LongLong { .. } => 8,
            },
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
                            TypeSpecifier::Bool => typeinfo.datatype = DataType::new_integer(IntegerRank::Bool, false),

                            TypeSpecifier::Char => {
                                typeinfo.datatype = DataType::new_integer(IntegerRank::Char, !unsigned_keyword)
                            }

                            TypeSpecifier::Short => {
                                typeinfo.datatype = DataType::new_integer(IntegerRank::Short, !unsigned_keyword)
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
                                        typeinfo.datatype = DataType::new_integer(IntegerRank::Int, !unsigned_keyword)
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
                        DataType::new_integer(IntegerRank::Int, !unsigned_keyword)
                    }
                }
                1 => {
                    if is_double {
                        DataType::LongDouble // long double x;
                    } else {
                        DataType::new_integer(IntegerRank::Long, !unsigned_keyword)
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
                        // long long int
                        DataType::new_integer(IntegerRank::LongLong, !unsigned_keyword)
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
                                Type::new(DataType::new_integer(IntegerRank::Int, true))
                            } else if *val <= i64::MAX {
                                Type::new(DataType::new_integer(IntegerRank::Long, true))
                            } else {
                                Type::new(DataType::new_integer(IntegerRank::LongLong, true))
                            }
                        } else {
                            if *val >= i32::MIN as i64 {
                                Type::new(DataType::new_integer(IntegerRank::Int, true))
                            } else if *val >= i64::MIN {
                                Type::new(DataType::new_integer(IntegerRank::Long, true))
                            } else {
                                Type::new(DataType::new_integer(IntegerRank::LongLong, true))
                            }
                        }
                    }
                    IntegerType::Signed(_) => Type::new(DataType::new_integer(IntegerRank::Int, true)),
                    IntegerType::SignedLong(_) => Type::new(DataType::new_integer(IntegerRank::Long, true)),
                    IntegerType::SignedLongLong(_) => Type::new(DataType::new_integer(IntegerRank::LongLong, true)),
                    IntegerType::Unsigned(_) => Type::new(DataType::new_integer(IntegerRank::Int, false)),
                    IntegerType::UnsignedLong(_) => Type::new(DataType::new_integer(IntegerRank::Long, false)),
                    IntegerType::UnsignedLongLong(_) => Type::new(DataType::new_integer(IntegerRank::LongLong, false)),
                }
            }
            Constant::Float(float_type) => match float_type {
                FloatingPointType::Float(_) => Type::new(DataType::Float),
                FloatingPointType::Double(_) => Type::new(DataType::Double),
                FloatingPointType::LongDouble(_) => Type::new(DataType::LongDouble),
            },
            Constant::Character(_) => Type::new(DataType::new_integer(IntegerRank::Char, true)),
        }
    }

    /// uac: usual arithmetic conversions
    pub fn common_datatype_for_uac(x: &Type, y: &Type) -> Result<DataType, CompilerError> {
        if !x.datatype.is_arithmetic() || !y.datatype.is_arithmetic() {
            return Err(CompilerError {
                kind: CompilerErrorKind::SemanticError,
                message: format!("expected arithmetic operands, instead got {}, {}", x, y),
                location: None,
            });
        }

        // floating point types take priority over integers
        if x.datatype == DataType::LongDouble || y.datatype == DataType::LongDouble {
            return Ok(DataType::LongDouble);
        }

        if x.datatype == DataType::Double || y.datatype == DataType::Double {
            return Ok(DataType::Double);
        }

        if x.datatype == DataType::Float || y.datatype == DataType::Float {
            return Ok(DataType::Float);
        }

        // Integer type promotion rules according to C23 standard:
        // 1. If both operands have the same type, then no further conversion is needed.
        //
        // 2. Otherwise, if both operands have signed integer types or both have unsigned integer
        // types, the operand with the type of lesser integer conversion rank is converted to the type
        // of the operand with greater rank.
        //
        // 3. Otherwise, if the operand that has unsigned integer type has rank greater or equal to
        // the rank of the type of the other operand, then the operand with signed integer type is
        // converted to the type of the operand with unsigned integer type.
        //
        // 4. Otherwise, if the type of the operand with signed integer type can represent all the values
        // of the type of the operand with unsigned integer type, then the operand with unsigned
        // integer type is converted to the type of the operand with signed integer type.
        //
        // 5. Otherwise, both operands are converted to the unsigned integer type corresponding to
        // the type of the operand with signed integer type.

        if let (
            DataType::Integer {
                rank: x_rank,
                signed: x_signed,
            },
            DataType::Integer {
                rank: y_rank,
                signed: y_signed,
            },
        ) = (x.datatype.clone(), y.datatype.clone())
        {
            let promote = |rank, signed| {
                if rank < IntegerRank::Int {
                    (IntegerRank::Int, true)
                } else {
                    (rank, signed)
                }
            };

            let (x_rank, x_signed) = promote(x_rank, x_signed);
            let (y_rank, y_signed) = promote(y_rank, y_signed);

            if x_signed == y_signed {
                Ok(DataType::new_integer(std::cmp::max(x_rank, y_rank), x_signed))
            } else if !x_signed && x_rank > y_rank {
                Ok(DataType::new_integer(x_rank, false))
            } else if !y_signed && y_rank > x_rank {
                Ok(DataType::new_integer(y_rank, false))
            } else if x_signed && x.size()? > y.size()? {
                Ok(DataType::new_integer(x_rank, true))
            } else if y_signed && y.size()? > x.size()? {
                Ok(DataType::new_integer(y_rank, true))
            } else if x_signed {
                Ok(DataType::new_integer(x_rank, false))
            } else if y_signed {
                Ok(DataType::new_integer(y_rank, false))
            } else {
                unreachable!()
            }
        } else {
            Err(CompilerError {
                kind: CompilerErrorKind::InternalError,
                message: format!("common_type_for_uac: during integer promotion types of operands should've been integers, instead are {}, {}", x, y),
                location: None
            })
        }
    }

    pub fn check_assignment_conversion(
        target: &Type,
        source: &Type,
    ) -> Result<AssignmentConversionResult, CompilerError> {
        if target == source {
            return Ok(AssignmentConversionResult::Identical);
        } else if target.datatype.is_arithmetic() && source.datatype.is_arithmetic() {
            if target.size()? < source.size()? {
                return Ok(AssignmentConversionResult::CastWithWarning(format!(
                    "narrowing down types from {} to {}, possible precision loss",
                    source, target
                )));
            } else {
                return Ok(AssignmentConversionResult::Cast);
            }
        } else {
            todo!()
        }
    }

    pub fn is_boolean_compatible(ty: &Type) -> bool {
        if ty.datatype.is_scalar() {
            true
        } else {
            todo!()
        }
    }
}

// ---------------------------------------------
// Display Implementations for the above structs
// ---------------------------------------------

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
            DataType::Integer { rank: subtype, signed } => {
                let sign = if *signed { "signed " } else { "unsigned " };
                match subtype {
                    IntegerRank::Bool => write!(f, "bool"),
                    IntegerRank::Char => write!(f, "{}char", sign),
                    IntegerRank::Short => write!(f, "{}short", sign),
                    IntegerRank::Int => write!(f, "{}int", sign),
                    IntegerRank::Long => write!(f, "{}long", sign),
                    IntegerRank::LongLong => write!(f, "{}long long", sign),
                }
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
