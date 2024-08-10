#![allow(non_snake_case)]

use crate::{op_code::OpCode, registers::Register};

#[derive(Debug)]
pub struct Tgs {
    /// R0 to R7 Regisers
    R: [u8; 8],
    /// BA & BB Regisers
    B: [u8; 2],
    /// D0 to D3 Regisers
    D: [u8; 4],
    /// Program Counter
    PC: u8,
    /// Comparison Result
    CR: u8,
}

impl Tgs {
    pub fn register(&self, register: Register) -> u8 {
        match register {
            Register::R0 => self.R[0],
            Register::R1 => self.R[1],
            Register::R2 => self.R[2],
            Register::R3 => self.R[3],
            Register::R4 => self.R[4],
            Register::R5 => self.R[5],
            Register::R6 => self.R[6],
            Register::R7 => self.R[7],

            Register::BA => self.B[0],
            Register::BB => self.B[1],

            Register::D0 => self.D[0],
            Register::D1 => self.D[1],
            Register::D2 => self.D[2],
            Register::D3 => self.D[3],

            Register::PC => self.PC,
            Register::CR => self.CR,
        }
    }

    pub fn register_ref(&self, register: Register) -> &u8 {
        match register {
            Register::R0 => &self.R[0],
            Register::R1 => &self.R[1],
            Register::R2 => &self.R[2],
            Register::R3 => &self.R[3],
            Register::R4 => &self.R[4],
            Register::R5 => &self.R[5],
            Register::R6 => &self.R[6],
            Register::R7 => &self.R[7],

            Register::BA => &self.B[0],
            Register::BB => &self.B[1],

            Register::D0 => &self.D[0],
            Register::D1 => &self.D[1],
            Register::D2 => &self.D[2],
            Register::D3 => &self.D[3],

            Register::PC => &self.PC,
            Register::CR => &self.CR,
        }
    }

    pub fn register_mut_ref(&mut self, register: Register) -> &mut u8 {
        match register {
            Register::R0 => &mut self.R[0],
            Register::R1 => &mut self.R[1],
            Register::R2 => &mut self.R[2],
            Register::R3 => &mut self.R[3],
            Register::R4 => &mut self.R[4],
            Register::R5 => &mut self.R[5],
            Register::R6 => &mut self.R[6],
            Register::R7 => &mut self.R[7],

            Register::BA => &mut self.B[0],
            Register::BB => &mut self.B[1],

            Register::D0 => &mut self.D[0],
            Register::D1 => &mut self.D[1],
            Register::D2 => &mut self.D[2],
            Register::D3 => &mut self.D[3],

            Register::PC => &mut self.PC,
            Register::CR => &mut self.CR,
        }
    }

    fn store_cr_rr(&mut self, target: Register, source: Register) {
        let v = self.register(target) as i8 - self.register(source) as i8;
        self.CR = u8::from_le(v.to_le_bytes()[0]);
    }

    fn store_cr_rv(&mut self, target: Register, source: u8) {
        let v = self.register(target) as i8 - source as i8;
        self.CR = u8::from_le_bytes(v.to_le_bytes());
    }

    fn get_cr_as_i8(&self) -> i8 {
        i8::from_le_bytes(self.CR.to_le_bytes())
    }

    pub fn process_instruction(&mut self, op_code: OpCode) {
        match op_code {
            OpCode::ADD(t, sr) => *self.register_mut_ref(t) += *self.register_ref(sr),
            OpCode::SUB(t, sr) => *self.register_mut_ref(t) -= *self.register_ref(sr),
            OpCode::LSH(t, sr) => *self.register_mut_ref(t) <<= *self.register_ref(sr),
            OpCode::RSH(t, sr) => *self.register_mut_ref(t) >>= *self.register_ref(sr),
            OpCode::AND(t, sr) => *self.register_mut_ref(t) &= *self.register_ref(sr),
            OpCode::OR(t, sr) => *self.register_mut_ref(t) |= *self.register_ref(sr),
            OpCode::XOR(t, sr) => *self.register_mut_ref(t) ^= *self.register_ref(sr),
            OpCode::CMP(t, sr) => self.store_cr_rr(t, sr),
            OpCode::MOV(t, sr) => *self.register_mut_ref(t) = self.register(sr),

            OpCode::ADDV(t, sv) => *self.register_mut_ref(t) += sv,
            OpCode::SUBV(t, sv) => *self.register_mut_ref(t) -= sv,
            OpCode::LSHV(t, sv) => *self.register_mut_ref(t) <<= sv,
            OpCode::RSHV(t, sv) => *self.register_mut_ref(t) >>= sv,
            OpCode::ANDV(t, sv) => *self.register_mut_ref(t) &= sv,
            OpCode::ORV(t, sv) => *self.register_mut_ref(t) |= sv,
            OpCode::XORV(t, sv) => *self.register_mut_ref(t) ^= sv,
            OpCode::CMPV(t, sv) => self.store_cr_rv(t, sv),
            OpCode::MOVV(t, sv) => *self.register_mut_ref(t) = sv,

            OpCode::BR(v) => self.PC = v,
            OpCode::BE(v) => {
                if self.get_cr_as_i8() == 0 {
                    self.PC = v
                }
            }
            OpCode::BNE(v) => {
                if self.get_cr_as_i8() != 0 {
                    self.PC = v
                }
            }
            OpCode::BG(v) => {
                if self.get_cr_as_i8() > 0 {
                    self.PC = v
                }
            }
            OpCode::BL(v) => {
                if self.get_cr_as_i8() < 0 {
                    self.PC = v
                }
            }
        }
    }
}
