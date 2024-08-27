use std::{fs, path::Path};

use crate::{error::{Error,Result}, op_code::OpCode};

pub struct Program {
    name: String,
    op_codes: Vec<OpCode>,
}

impl Program {
    pub fn new(name: &str, bytes: &[u8]) -> Result<Self> {
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
        
        Ok(Program {
            name: name.into(),
            op_codes,
        })
    }

    pub fn from_path(path: &Path) -> Result<Self> {
        let name = path.file_name()
            .map(|f| f.to_string_lossy())
            .unwrap_or("UNKNOWN".into());
        let bytes = fs::read(path)?;
        Self::new(&name, &bytes)
    }

    pub fn get_readable_program(&self) -> String {
        // len() * 000: ADD RA XX
        let len = self.op_codes.len();
        self.op_codes
            .iter()
            .fold(String::with_capacity(len*14), |mut acc, ins| {
                acc.push_str(&ins.to_string());
                acc.push('\n');
                acc
            })
    }

    pub fn get_readable_program_witn_line_num(&self) -> String {
        // len() * 000: ADD RA XX
        let len = self.op_codes.len();
        self.op_codes
            .iter()
            .enumerate()
            .fold(String::with_capacity(len*14), |mut acc, (i, ins)| {
                acc.push_str(&format!("{:03}: {}\n", i + 1, ins));
                acc
            })
    }

    pub fn get_ins(&self, i: usize) -> Option<&OpCode> {
        self.op_codes.get(i)
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}
