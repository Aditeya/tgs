use std::fmt::Display;

use crate::{
    error::{Error, Result},
    registers::Register,
};

#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum OpCode {
    ADD(Register, Register) = 0b0001_0000,
    ADDV(Register, u8) = 0b0001_0001,
    SUB(Register, Register) = 0b0001_0010,
    SUBV(Register, u8) = 0b0001_0011,

    LSH(Register, Register) = 0b0010_0000,
    LSHV(Register, u8) = 0b0010_0001,
    RSH(Register, Register) = 0b0010_0010,
    RSHV(Register, u8) = 0b0010_0011,
    AND(Register, Register) = 0b0011_0000,
    ANDV(Register, u8) = 0b0011_0001,
    OR(Register, Register) = 0b0011_0010,
    ORV(Register, u8) = 0b0011_0011,
    XOR(Register, Register) = 0b0011_0100,
    XORV(Register, u8) = 0b0011_0101,

    CMP(Register, Register) = 0b0100_0000,
    CMPV(Register, u8) = 0b0100_0001,
    BR(u8) = 0b0101_0000,
    BE(u8) = 0b0101_0010,
    BNE(u8) = 0b0101_0100,
    BG(u8) = 0b0101_0110,
    BL(u8) = 0b0101_1000,

    MOV(Register, Register) = 0b0110_0000,
    MOVV(Register, u8) = 0b0110_0001,
}

impl OpCode {
    pub fn is_src_value(byte: u8) -> bool {
        const SRC_VALUE: u8 = 0b0000_0001;
        (byte & SRC_VALUE) == SRC_VALUE
    }

    pub fn is_branching(byte: u8) -> bool {
        const BRANCHING: u8 = 0b0101_0000;
        (byte & BRANCHING) == BRANCHING
    }

    pub fn process_bytes_to_instructions(bytes: &[u8]) -> Result<Vec<OpCode>> {
        if bytes.len() % 3 != 0 || bytes.is_empty() {
            return Err(Error::InvalidProgram);
        }

        let mut op_codes = Vec::with_capacity(bytes.len() / 3);
        for window in bytes.chunks_exact(3) {
            if let (Some(ins), Some(tar), Some(src)) = (window.first(), window.get(1), window.get(2))
            {
                op_codes.push([*ins, *tar, *src].try_into()?);
            } else {
                return Err(Error::InvalidProgram);
            }
        }
        
        Ok(op_codes)
    }
}

impl Display for OpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            OpCode::ADD(r1, r2) => write!(f, "ADD %{r1}, %{r2}"),
            OpCode::SUB(r1, r2) => write!(f, "SUB %{r1}, %{r2}"),
            OpCode::LSH(r1, r2) => write!(f, "LSH %{r1}, %{r2}"),
            OpCode::RSH(r1, r2) => write!(f, "RSH %{r1}, %{r2}"),
            OpCode::AND(r1, r2) => write!(f, "AND %{r1}, %{r2}"),
            OpCode::OR(r1, r2)  => write!(f, "OR  %{r1}, %{r2}"),
            OpCode::XOR(r1, r2) => write!(f, "XOR %{r1}, %{r2}"),
            OpCode::CMP(r1, r2) => write!(f, "CMP %{r1}, %{r2}"),
            OpCode::MOV(r1, r2) => write!(f, "MOV %{r1}, %{r2}"),

            OpCode::ADDV(r, v) => write!(f, "ADD %{r}, ${v}"),
            OpCode::SUBV(r, v) => write!(f, "SUB %{r}, ${v}"),
            OpCode::LSHV(r, v) => write!(f, "LSH %{r}, ${v}"),
            OpCode::RSHV(r, v) => write!(f, "RSH %{r}, ${v}"),
            OpCode::ANDV(r, v) => write!(f, "AND %{r}, ${v}"),
            OpCode::ORV(r, v)  => write!(f, "OR  %{r}, ${v}"),
            OpCode::XORV(r, v) => write!(f, "XOR %{r}, ${v}"),
            OpCode::CMPV(r, v) => write!(f, "CMP %{r}, ${v}"),
            OpCode::MOVV(r, v) => write!(f, "MOV %{r}, ${v}"),

            OpCode::BR(v)  => write!(f, "BR  ${v}"),
            OpCode::BE(v)  => write!(f, "BE  ${v}"),
            OpCode::BNE(v) => write!(f, "BNE ${v}"),
            OpCode::BG(v)  => write!(f, "BG  ${v}"),
            OpCode::BL(v)  => write!(f, "BL  ${v}"),
        }
    }
}

impl TryFrom<[u8; 3]> for OpCode {
    type Error = Error;

    fn try_from([ins, target, src]: [u8; 3]) -> std::result::Result<Self, Self::Error> {
        let val = if Self::is_branching(ins) {
            match ins {
                0b0101_0000 => OpCode::BR(target),
                0b0101_0010 => OpCode::BE(target),
                0b0101_0100 => OpCode::BNE(target),
                0b0101_0110 => OpCode::BG(target),
                0b0101_1000 => OpCode::BL(target),
                _ => return Err(Error::InvalidRegisterAddress(ins)),
            }
        } else if Self::is_src_value(ins) {
            let target: Register = target.try_into()?;
            match ins {
                0b0001_0001 => Self::ADDV(target, src),
                0b0001_0011 => Self::SUBV(target, src),
                0b0010_0001 => Self::LSHV(target, src),
                0b0010_0011 => Self::RSHV(target, src),
                0b0011_0001 => Self::ANDV(target, src),
                0b0011_0011 => Self::ORV(target, src),
                0b0011_0101 => Self::XORV(target, src),
                0b0100_0001 => Self::CMPV(target, src),
                0b0110_0001 => Self::MOVV(target, src),
                _ => return Err(Error::InvalidSrcValueOpCode(ins)),
            }
        } else {
            let target: Register = target.try_into()?;
            let src: Register = src.try_into()?;
            match ins {
                0b0001_0000 => Self::ADD(target, src),
                0b0001_0010 => Self::SUB(target, src),
                0b0010_0000 => Self::LSH(target, src),
                0b0010_0010 => Self::RSH(target, src),
                0b0011_0000 => Self::AND(target, src),
                0b0011_0010 => Self::OR(target, src),
                0b0011_0100 => Self::XOR(target, src),
                0b0100_0000 => Self::CMP(target, src),
                0b0110_0000 => Self::MOV(target, src),
                _ => return Err(Error::InvalidOpCode(ins)),
            }
        };

        Ok(val)
    }
}
