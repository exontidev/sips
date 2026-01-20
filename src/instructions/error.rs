#[derive(Debug)]
pub enum Error {
    InvalidDiscriminator,
    InvalidInstructionSize,
    InvalidInstructionData,
}
