#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub enum DataType {
    Pointer(Box<DataType>),
    Primitive(PrimitiveType),
    Structure(/* TODO */),
    Enumeration(),
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub enum PrimitiveType {
    U32,
    U64,
    Usize,
    I32,
    I64,
    F32,
    F64,
    Char,
}

impl TryFrom<&str> for PrimitiveType {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(match value {
            "u32" => PrimitiveType::U32,
            "u64" => PrimitiveType::U64,
            "usize" => PrimitiveType::Usize,
            "i32" => PrimitiveType::I32,
            "i64" => PrimitiveType::I64,
            "f32" => PrimitiveType::F32,
            "f64" => PrimitiveType::F64,
            "char" => PrimitiveType::Char,
            _ => return Err(())
        })
    }
}