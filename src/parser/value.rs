#![allow(dead_code)]

/// Value Enumeration
#[derive(Debug, Clone)]
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

/// Value Types
#[derive(Debug, Clone)]
pub struct ValueType
{
    pub references: usize,
    pub value_type: RawValueType
}

/// Raw Value Type
#[derive(Debug, Clone)]
pub enum RawValueType
{
    I8, I16, I32, I64,
    U8, U16, U32, U64,
    Struct(String)
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