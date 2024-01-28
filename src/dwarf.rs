use crate::{
    elf::ElfFile,
    reader::{read_u16, read_u32, read_u8},
};

pub struct DwarfFile {
    pub unit_header: CompilationUnitHeader,
}

pub struct CompilationUnitHeader {
    pub length: u32,
    pub version: u16,
    pub unit_type: u8,
    pub address_size: u8,
    pub debug_abbrev_offset: u32,
}

pub fn parse(elf_file: ElfFile, contents: &Vec<u8>) -> DwarfFile {
    let mut debug_info_section = None;

    for section in elf_file.sections {
        if section.name != ".debug_info" {
            continue;
        }

        debug_info_section = Some(section);

        break;
    }

    if (debug_info_section.is_none()) {
        panic!("No .debug_info section found in elf file!");
    }

    let debug_info_section = debug_info_section.unwrap();

    let mut cursor: u64 = debug_info_section.pointer;

    let length = read_u32(&contents, &mut cursor);
    let version = read_u16(&contents, &mut cursor);
    let unit_type = read_u8(&contents, &mut cursor);
    let address_size = read_u8(&contents, &mut cursor);
    let debug_abbrev_offset = read_u32(&contents, &mut cursor);

    let unit_header = CompilationUnitHeader {
        length,
        version,
        unit_type,
        address_size,
        debug_abbrev_offset,
    };

    return DwarfFile { unit_header };
}
