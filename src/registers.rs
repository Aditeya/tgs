use strum::{AsRefStr, EnumIter, FromRepr};

use crate::error::Error;

#[derive(Copy, Clone, Debug, AsRefStr, EnumIter, strum::Display, FromRepr)]
#[repr(u8)]
pub enum Register {
    R0 = 0b0000_0000,
    R1 = 0b0000_0001,
    R2 = 0b0000_0010,
    R3 = 0b0000_0011,
    R4 = 0b0000_0100,
    R5 = 0b0000_0101,
    R6 = 0b0000_0110,
    R7 = 0b0000_0111,

    BA = 0b0001_0000,
    BB = 0b0001_0001,

    D0 = 0b0001_0010,
    D1 = 0b0001_0011,
    D2 = 0b0001_0100,
    D3 = 0b0001_0101,

    PC = 0b0001_0110,
    CR = 0b0001_0111,
}

impl Register {
    pub fn get_addr(&self) -> u8 {
        *self as u8
    }

    pub fn get_addr_as_bin_str(&self) -> String {
        let byte = self.get_addr();
        let binary_str = format!("{:08b}", byte);
        binary_str
            .chars()
            .enumerate()
            .fold(String::with_capacity(9), |mut acc, (i, c)| {
                if i == 4 {
                    acc.push(' ');
                }
                acc.push(c);
                acc
            })
    }

    pub fn get_addr_as_hex_str(&self) -> String {
        let byte = self.get_addr();
        format!("{:02X}", byte)
    }
}

impl TryFrom<u8> for Register {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_repr(value).ok_or(Error::InvalidRegisterAddress(value))
    }
}
