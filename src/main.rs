use niosii::ElfProgram;
use niosii::Nios2DE0;

fn main() {
    // Read a .s file with source code
    // Read a string with source code
    // Might want to compile the source code into an ELF format before

    let bytes = std::fs::read("resources/loop.elf").unwrap();
    let _prog = ElfProgram::from_bytes(bytes.as_slice());

    // Memory unit that acts as the RAM for the device

    // Processing unit that accepts opcodes and arguments
    // I/O devices based on different board designs (like cpulator)

    let _sys = Nios2DE0::new();
}
