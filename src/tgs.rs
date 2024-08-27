#![allow(non_snake_case)]

use std::num::Wrapping;

use crate::{op_code::OpCode, program::Program, registers::Register};

#[derive(Debug)]
pub struct Tgs {
    /// R0 to R7 Regisers
    R: [Wrapping<u8>; 8],
    /// BA & BB Regisers
    B: [Wrapping<u8>; 2],
    /// D0 to D3 Regisers
    D: [Wrapping<u8>; 4],
    /// Program Counter
    PC: Wrapping<u8>,
    /// Comparison Result
    CR: Wrapping<u8>,
}

impl Tgs {
    pub fn new() -> Self {
        Self {
            R: [Wrapping(0); 8],
            B: [Wrapping(0); 2],
            D: [Wrapping(0); 4],
            PC: Wrapping(0),
            CR: Wrapping(0),
        }
    }

    pub fn tgs_display(&self) -> [Wrapping<u8>; 4] {
        self.D
    }

    pub fn register(&self, register: Register) -> Wrapping<u8> {
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

    pub fn register_ref(&self, register: Register) -> &Wrapping<u8> {
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

    pub fn register_mut_ref(&mut self, register: Register) -> &mut Wrapping<u8> {
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
        let v = self.register(target).0 as i8 - self.register(source).0 as i8;
        let v = u8::from_le(v.to_le_bytes()[0]);
        self.CR = Wrapping(v);
    }

    fn store_cr_rv(&mut self, target: Register, source: u8) {
        let t = self.register(target).0 as i8;
        let s = source as i8;
        let v = t.wrapping_sub(s);
        let v = u8::from_le_bytes(v.to_le_bytes());
        self.CR = Wrapping(v);
    }

    fn get_cr_as_i8(&self) -> i8 {
        i8::from_le_bytes(self.CR.0.to_le_bytes())
    }

    fn increment_pc(&mut self) {
        self.PC += 1;
    }

    /// returns true if program_counter should be incremented
    pub fn process_instruction(&mut self, op_code: OpCode) {
        match op_code {
            OpCode::ADD(t, sr) => {let reg=self.register(sr);*self.register_mut_ref(t) += reg}
            OpCode::SUB(t, sr) => {let reg=self.register(sr);*self.register_mut_ref(t) -= reg}
            OpCode::LSH(t, sr) => {let reg=self.register(sr);*self.register_mut_ref(t) <<= reg.0 as usize},
            OpCode::RSH(t, sr) => {let reg=self.register(sr);*self.register_mut_ref(t) >>= reg.0 as usize},
            OpCode::AND(t, sr) => {let reg=self.register(sr);*self.register_mut_ref(t) &= reg}
            OpCode::OR(t, sr) => {let reg=self.register(sr);*self.register_mut_ref(t) |= reg}
            OpCode::XOR(t, sr) => {let reg = self.register(sr);*self.register_mut_ref(t) ^= reg},
            OpCode::CMP(t, sr) => self.store_cr_rr(t, sr),
            OpCode::MOV(t, sr) => *self.register_mut_ref(t) = self.register(sr),

            OpCode::ADDV(t, sv) => *self.register_mut_ref(t) += sv,
            OpCode::SUBV(t, sv) => *self.register_mut_ref(t) -= sv,
            OpCode::LSHV(t, sv) => *self.register_mut_ref(t) <<= sv as usize,
            OpCode::RSHV(t, sv) => *self.register_mut_ref(t) >>= sv as usize,
            OpCode::ANDV(t, sv) => *self.register_mut_ref(t) &= sv,
            OpCode::ORV(t, sv) => *self.register_mut_ref(t) |= sv,
            OpCode::XORV(t, sv) => *self.register_mut_ref(t) ^= sv,
            OpCode::CMPV(t, sv) => self.store_cr_rv(t, sv),
            OpCode::MOVV(t, sv) => *self.register_mut_ref(t) = Wrapping(sv),

            OpCode::BR(v) => {
                self.PC = Wrapping(v);
                return;
            }
            OpCode::BE(v) => {
                if self.get_cr_as_i8() == 0 {
                    self.PC = Wrapping(v);
                    return;
                }
            }
            OpCode::BNE(v) => {
                if self.get_cr_as_i8() != 0 {
                    self.PC = Wrapping(v);
                    return;
                }
            }
            OpCode::BG(v) => {
                if self.get_cr_as_i8() > 0 {
                    self.PC = Wrapping(v);
                    return;
                }
            }
            OpCode::BL(v) => {
                if self.get_cr_as_i8() < 0 {
                    self.PC = Wrapping(v);
                    return;
                }
            }
        };

        self.increment_pc();
    }

    pub fn run_program(&mut self, program: &Program) {
        while let Some(instruction) = program.get_ins(self.PC.0 as usize) {
            std::thread::sleep(std::time::Duration::from_secs(1));
            self.process_instruction(*instruction)
        }
    }
}

impl Default for Tgs {
    fn default() -> Self {
        Self::new()
    }
}
