use crate::reader;

pub struct ElfFile {
    pub file_header: FileHeader,
    pub section_headers: Vec<SectionHeader>,
    pub sections: Vec<Section>,
}

pub struct FileHeader {
    pub x64: bool,
    pub little_endian: bool,
    pub executeable_type: u16,
    pub entry_point: u64,
    pub header_size: u16,

    pub program_header_table_pointer: u64,
    pub program_header_table_entry_size: u16,
    pub program_header_table_entries: u16,

    pub section_header_table_pointer: u64,
    pub section_header_table_entry_size: u16,
    pub section_header_table_entries: u16,

    pub section_header_names_index: u16,
}

pub struct SectionHeader {
    pub name_offset: u32,
    pub section_type: u32,

    pub pointer: u64,
    pub size: u64,
}

pub struct Section {
    pub name: String,
    pub pointer: u64,
}

pub fn parse(contents: &Vec<u8>) -> ElfFile {
    let mut cursor: u64 = 0;

    reader::read_ascii(&contents, &mut cursor, 4);

    let x64 = reader::read_u8(&contents, &mut cursor) == 0x02;
    let little_endian = reader::read_u8(&contents, &mut cursor) == 0x01;

    cursor += 10;

    let executeable_type = reader::read_u16(&contents, &mut cursor);

    cursor += 6;

    let entry_point = reader::read_u64(&contents, &mut cursor);

    let program_header_table_pointer = reader::read_u64(&contents, &mut cursor);
    let section_header_table_pointer = reader::read_u64(&contents, &mut cursor);

    cursor += 4;

    let header_size = reader::read_u16(&contents, &mut cursor);

    let program_header_table_entry_size = reader::read_u16(&contents, &mut cursor);
    let program_header_table_entries = reader::read_u16(&contents, &mut cursor);

    let section_header_table_entry_size = reader::read_u16(&contents, &mut cursor);
    let section_header_table_entries = reader::read_u16(&contents, &mut cursor);

    let section_header_names_index = reader::read_u16(&contents, &mut cursor);

    let file_header = FileHeader {
        x64,
        little_endian,
        executeable_type,
        entry_point,
        header_size,

        program_header_table_pointer,
        program_header_table_entry_size,
        program_header_table_entries,

        section_header_table_pointer,
        section_header_table_entry_size,
        section_header_table_entries,

        section_header_names_index,
    };

    cursor = section_header_table_pointer;

    let mut section_headers: Vec<SectionHeader> = Vec::new();

    for _index in 0..section_header_table_entries {
        let name_offset = reader::read_u32(&contents, &mut cursor);
        let section_type = reader::read_u32(&contents, &mut cursor);

        cursor += 16;

        let pointer = reader::read_u64(&contents, &mut cursor);
        let size = reader::read_u64(&contents, &mut cursor);

        cursor += 24;

        section_headers.push(SectionHeader {
            name_offset,
            section_type,

            pointer,
            size,
        })
    }

    let mut sections: Vec<Section> = Vec::new();

    let name_section_pointer = section_headers[section_header_names_index as usize].pointer;

    for index in 0..section_header_table_entries {
        let section_header = &section_headers[index as usize];

        cursor = name_section_pointer + section_header.name_offset as u64;

        let name = reader::read_null_terminated_ascii(&contents, &mut cursor);

        let section = Section {
            name,
            pointer: section_header.pointer,
        };

        sections.push(section);
    }

    return ElfFile {
        file_header,
        section_headers,
        sections,
    };
}
