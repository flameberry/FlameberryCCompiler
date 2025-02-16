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
