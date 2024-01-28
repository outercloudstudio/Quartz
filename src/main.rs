use std::fs;

mod dwarf;
mod elf;
mod reader;

fn main() {
    let path: String = String::from("bedrock_server_symbols.debug");

    let contents: Vec<u8> = fs::read(path).unwrap();

    let elf_file = elf::parse(&contents);

    for section in &elf_file.sections {
        println!("Section {} at {:#X}", section.name, section.pointer);
    }

    let dwarf_file = dwarf::parse(elf_file, &contents);

    println!(
        "Length {:#X} Version {} Type {}",
        dwarf_file.unit_header.length,
        dwarf_file.unit_header.version,
        dwarf_file.unit_header.unit_type
    )

    // let debug_str_section = &elf_file.section_headers[35];
    // let debug_str_data = &contents[debug_str_section.pointer as usize
    //     ..(debug_str_section.pointer + debug_str_section.size) as usize];

    // fs::write("debug_str.txt", debug_str_data).unwrap();
}
