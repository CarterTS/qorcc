#![allow(dead_code)]

/// Value Enumeration
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Value
{
    pub value: u64,
    pub value_type: ValueType
}

impl Value
{
    pub fn code_constant(value: u32) -> Self
    {
        Self
        {
            value: value as u64,
            value_type: RawValueType::I32.into()
        }
    }
}

impl std::fmt::Display for Value
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        write!(f, "{} {}", self.value, self.value_type)
    }
}

/// Value Types
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValueType
{
    pub references: usize,
    pub value_type: RawValueType
}

impl std::fmt::Display for ValueType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        write!(f, "{}", self.value_type)?;

        for _ in 0..self.references
        {
            write!(f, "*")?;
        }

        Ok(())
    }
}

/// Raw Value Type
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RawValueType
{
    Void,
    I8, I16, I32, I64,
    U8, U16, U32, U64,
    Struct(String)
}

impl RawValueType
{
    pub fn make_signed(&self) -> Self
    {
        match self
        {
            RawValueType::I8 => RawValueType::I8,
            RawValueType::I16 => RawValueType::I16,
            RawValueType::I32 => RawValueType::I32,
            RawValueType::I64 => RawValueType::I64,
            RawValueType::U8 => RawValueType::I8,
            RawValueType::U16 => RawValueType::I16,
            RawValueType::U32 => RawValueType::I32,
            RawValueType::U64 => RawValueType::I64,
            _ => panic!(),
        }
    }

    pub fn make_unsigned(&self) -> Self
    {
        match self
        {
            RawValueType::I8 => RawValueType::U8,
            RawValueType::I16 => RawValueType::U16,
            RawValueType::I32 => RawValueType::U32,
            RawValueType::I64 => RawValueType::U64,
            RawValueType::U8 => RawValueType::U8,
            RawValueType::U16 => RawValueType::U16,
            RawValueType::U32 => RawValueType::U32,
            RawValueType::U64 => RawValueType::U64,
            _ => panic!(),
        }
    }
}

impl std::fmt::Display for RawValueType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        match self
        {
            RawValueType::Void => write!(f, "void"),
            RawValueType::I8 => write!(f, "i8"),
            RawValueType::I16 => write!(f, "i16"),
            RawValueType::I32 => write!(f, "i32"),
            RawValueType::I64 => write!(f, "i64"),
            RawValueType::U8 => write!(f, "u8"),
            RawValueType::U16 => write!(f, "u16"),
            RawValueType::U32 => write!(f, "u32"),
            RawValueType::U64 => write!(f, "u64"),
            RawValueType::Struct(name) => write!(f, "struct {}", name),
        }
    }

}

impl std::convert::From<RawValueType> for ValueType
{
    fn from(raw: RawValueType) -> Self
    {
        Self
        {
            references: 0,
            value_type: raw
        }
    }
}