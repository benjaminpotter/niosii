use elf::{ElfBytes, endian::LittleEndian};
use std::path::Path;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ProgramError {
    #[error("failed to parse elf file: {0}")]
    InvalidElf(#[from] elf::ParseError),
}

pub(crate) trait Program {
    fn try_load(&self, mem_size: usize) -> Result<Vec<u8>, ProgramError>;
    fn entry_point(&self) -> Result<u32, ProgramError>;
}

pub struct ElfProgram {
    bytes: Vec<u8>,
}

impl ElfProgram {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        Self {
            bytes: bytes.to_vec(),
        }
    }

    pub fn from_path<P: AsRef<Path>>(path: P) -> Result<Self, std::io::Error> {
        let bytes = std::fs::read(path)?;
        Ok(Self::from_bytes(&bytes))
    }
}

impl Program for ElfProgram {
    fn try_load(&self, mem_size: usize) -> Result<Vec<u8>, ProgramError> {
        let elf = ElfBytes::<LittleEndian>::minimal_parse(&self.bytes)?;

        // Allocate flat memory buffer (size should cover your simulated RAM)
        let mut memory = vec![0u8; mem_size];

        // Iterate over program headers (segments), not sections
        if let Some(segments) = elf.segments() {
            for phdr in segments {
                // Only load PT_LOAD segments (type 1)
                if phdr.p_type == elf::abi::PT_LOAD {
                    let offset = phdr.p_offset as usize;
                    let file_size = phdr.p_filesz as usize;
                    let mem_addr = phdr.p_paddr as usize; // Use physical address for bare-metal

                    // Validate bounds
                    if mem_addr + file_size > memory.len() {
                        eprintln!("warning: segment at 0x{mem_addr:08x} exceeds memory bounds");
                        continue;
                    }

                    // Copy segment data from file into simulated memory
                    let segment_data = &self.bytes[offset..offset + file_size];
                    memory[mem_addr..mem_addr + file_size].copy_from_slice(segment_data);

                    println!(
                        "Loaded segment: paddr=0x{:08x}, filesz=0x{:x}, memsz=0x{:x}",
                        phdr.p_paddr, phdr.p_filesz, phdr.p_memsz
                    );

                    // Zero-fill BSS (memsz > filesz means zero-init region)
                    let mem_size_seg = phdr.p_memsz as usize;
                    if mem_size_seg > file_size {
                        let bss_start = mem_addr + file_size;
                        let bss_end = mem_addr + mem_size_seg;
                        if bss_end <= memory.len() {
                            memory[bss_start..bss_end].fill(0);
                        }
                    }
                }
            }
        }

        Ok(memory)
    }

    fn entry_point(&self) -> Result<u32, ProgramError> {
        let elf = ElfBytes::<LittleEndian>::minimal_parse(&self.bytes)?;
        Ok(elf.ehdr.e_entry as u32)
    }
}

fn format_bytes(bytes: &[u8]) -> String {
    let mut cursor = 0;
    let hex = hex::encode(bytes);
    let mut buffer = String::new();
    while cursor < hex.len() {
        let end = cursor + 8;
        buffer += &format!("{} ", &hex[cursor..end]);
        cursor = end;
    }

    buffer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_elf() {
        let elf_path = "tests/fixtures/loop.elf";
        let mem_size = 64 * 1024 * 1024; // 64 MB

        let elf = ElfProgram::from_path(elf_path).unwrap();
        let memory = elf.try_load(mem_size).unwrap();
        let entry = elf.entry_point().unwrap();

        let address = entry as usize;
        let instruction_bytes = memory[address..address + 4].try_into().unwrap();
        let instruction = u32::from_le_bytes(instruction_bytes);
        let opcode = instruction & 0b0011_1111;

        println!("memory loaded: {} bytes total", memory.len());
        println!("entry point: 0x{entry:08x}");
        println!("first instruction: 0x{instruction:08x} opcode: 0x{opcode:02x}");

        assert_eq!(instruction, 0x0040_0044);
    }
}
