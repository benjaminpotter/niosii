use crate::bus::Bus;
use std::fmt;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub(crate) enum ProcessorError {
    #[error("instruction could not be decoded")]
    InvalidInstruction(#[from] DecodeError),
}

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct Processor {
    pc: u32,
    rf: [u32; 32],
}

impl Processor {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub(crate) fn step(self, bus: &mut Bus) -> Result<Self, ProcessorError> {
        let word = Processor::fetch(self.pc, bus);
        let instruction = Instruction::from_word(word)?;
        self.execute(instruction, bus)
    }

    fn fetch(addr: u32, bus: &Bus) -> u32 {
        todo!()
    }

    fn execute(mut self, instruction: Instruction, bus: &mut Bus) -> Result<Self, ProcessorError> {
        match instruction {
            Instruction::Addi(ra, rb, imm16) => {
                self.rf[rb] = (self.rf[ra] as i32).wrapping_add(imm16 as i32) as u32;
            }
            _ => todo!(),
        }

        Ok(self)
    }
}

#[derive(Error, Debug, PartialEq)]
pub(crate) struct DecodeError {
    word: u32,
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "invalid word '{}'",
            hex::encode(&self.word.to_le_bytes())
        )
    }
}

#[derive(Debug, PartialEq)]
pub(crate) enum Instruction {
    Addi(usize, usize, i16),
    Br(i16),
}

impl Instruction {
    fn from_word(word: u32) -> Result<Self, DecodeError> {
        match word & 0x3f {
            0x00 => Instruction::from_j_type(word),
            0x3a => Instruction::from_r_type(word),
            _ => Instruction::from_i_type(word),
        }
    }

    /// | rA        | rB        | IMM16    | OP      |
    /// | 31 ... 27 | 26 ... 22 | 21 ... 6 | 5 ... 0 |
    fn from_i_type(word: u32) -> Result<Self, DecodeError> {
        let imm16 = ((word >> 6) & 0xffff) as u16;
        let rb = ((word >> 22) & 0x001f) as usize;
        let ra = ((word >> 27) & 0x001f) as usize;

        match word & 0x3f {
            0x04 => Ok(Instruction::Addi(ra, rb, imm16 as i16)),
            0x06 => Ok(Instruction::Br(imm16 as i16)),
            _ => Err(DecodeError { word }),
        }
    }

    /// | rA        | rB        | rC        | OPX      | OP      |
    /// | 31 ... 27 | 26 ... 22 | 21 ... 17 | 16 ... 6 | 5 ... 0 |
    fn from_r_type(word: u32) -> Result<Self, DecodeError> {
        // 6-bit op code
        // 11-bits
        let opx = (word >> 6) & 0x07ff;
        // 5-bits
        let rc = (word >> 17) & 0x001f;
        // 5-bits
        let rb = (word >> 22) & 0x001f;
        // 5-bits
        let ra = (word >> 27) & 0x001f;

        todo!()
    }

    fn from_j_type(word: u32) -> Result<Self, DecodeError> {
        // op: u8, immed26: u32

        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(0x10800044, Instruction::Addi(2, 2, 1))]
    #[case(0x003ffe06, Instruction::Br(-8))]
    fn decode_works(#[case] word: u32, #[case] instruction: Instruction) {
        assert_eq!(Instruction::from_word(word), Ok(instruction));
    }

    #[rstest]
    #[case(0x10800048)]
    fn decode_fails(#[case] word: u32) {
        assert_eq!(Instruction::from_word(word), Err(DecodeError { word }));
    }

    #[rstest]
    #[case(0, 1, 1)]
    #[case(i32::MAX, 1, i32::MIN)]
    #[case(i32::MIN, 1, 0)]
    fn addi_works(#[case] left: i32, #[case] right: i16, #[case] sum: i32) {
        let mut bus = Bus;
        let instruction = Instruction::Addi(2, 2, right);

        let mut before = Processor::new();
        before.rf[2] = left as u32;

        let mut after = Processor::new();
        after.rf[2] = sum as u32;

        assert_eq!(before.execute(instruction, &mut bus), Ok(after));
    }
}
