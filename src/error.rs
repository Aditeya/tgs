
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("The value {0:08b} is not a valid Register Address")]
    InvalidRegisterAddress(u8),
    #[error("The value {0:08b} is not a valid Branching OpCode")]
    InvalidBranchOpCode(u8),
    #[error("The value {0:08b} is not a valid Src Val OpCode")]
    InvalidSrcValueOpCode(u8),
    #[error("The value {0:08b} is not a valid OpCode")]
    InvalidOpCode(u8),
    #[error("The program is invalid")]
    InvalidProgram,
}
