use elf::{ElfBytes, abi::PT_LOAD, endian::LittleEndian};
use std::error::Error;

pub struct Program;

impl Program {
    pub fn from_elf(bytes: &[u8]) -> Result<Self, Box<dyn Error + 'static>> {
        let file = ElfBytes::<LittleEndian>::minimal_parse(bytes)?;

        // let (shdrs_opt, strtab_opt) = file.section_headers_with_strtab()?;
        // let (shdrs, strtab) = (shdrs_opt.unwrap(), strtab_opt.unwrap());
        //
        // for section in shdrs {
        //     println!("{}", strtab.get(section.sh_name as usize)?);
        //     println!("{:#?}", section);
        //     println!("");
        // }

        let hdr = file.section_header_by_name(".text")?.unwrap();
        println!("{:#?}", hdr);

        let (data, _cmp) = file.section_data(&hdr)?;
        println!("{}", format_bytes(data));

        // for segment in file.segments().unwrap() {
        //     println!("{:#?}", segment.p_type == PT_LOAD);
        //     println!("{:#?}", segment);
        //
        //     let data = file.segment_data(&segment)?;
        //
        //     let mut cursor = 0;
        //     let hex = hex::encode(data);
        //     while cursor < hex.len() {
        //         let end = cursor + 8;
        //         println!("{}", &hex[cursor..end]);
        //         cursor = end;
        //     }
        // }

        Ok(Self)
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
