use elf::ElfBytes;
use elf::endian::LittleEndian; // Nios II is little-endian

#[test]
fn test_load_elf_into_memory() {
    let elf_path = "tests/fixtures/loop.elf";
    let mem_size = 64 * 1024 * 1024; // 64 MB
    let memory = load_elf_into_memory(elf_path, mem_size).unwrap();
    let entry = get_entry_point(elf_path).unwrap();

    let address = entry as usize;
    let instruction_bytes = memory[address..address + 4].try_into().unwrap();
    let instruction = u32::from_le_bytes(instruction_bytes);
    let opcode = instruction & 0b0011_1111;

    println!("memory loaded: {} bytes total", memory.len());
    println!("entry point: 0x{entry:08x}");
    println!("first instruction: 0x{instruction:08x} opcode: 0x{opcode:02x}");

    assert_eq!(instruction, 0x0040_0044);
}

fn load_elf_into_memory(
    path: &str,
    mem_size: usize,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    // Read the raw file
    let file_data = std::fs::read(path)?;
    let elf = ElfBytes::<LittleEndian>::minimal_parse(&file_data)?;

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
                let segment_data = &file_data[offset..offset + file_size];
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

fn get_entry_point(path: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let file_data = std::fs::read(path)?;
    let elf = ElfBytes::<LittleEndian>::minimal_parse(&file_data)?;
    Ok(elf.ehdr.e_entry as u32)
}
