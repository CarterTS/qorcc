
/// RISC-V Registers
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Register
{
    A0,
    A1,
    A2, 
    A3,
    A4,
    A5,
    A6,
    A7,
    T0,
    T1,
    T2,
    T3,
    T4,
    T5
}

impl std::fmt::Display for Register
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result
    {
        match self
        {
            Register::A0 => write!(f, "a0"),
            Register::A1 => write!(f, "a1"),
            Register::A2 => write!(f, "a2"),
            Register::A3 => write!(f, "a3"),
            Register::A4 => write!(f, "a4"),
            Register::A5 => write!(f, "a5"),
            Register::A6 => write!(f, "a6"),
            Register::A7 => write!(f, "a7"),
            Register::T0 => write!(f, "t0"),
            Register::T1 => write!(f, "t1"),
            Register::T2 => write!(f, "t2"),
            Register::T3 => write!(f, "t3"),
            Register::T4 => write!(f, "t4"),
            Register::T5 => write!(f, "t5"),
        }
    }
}