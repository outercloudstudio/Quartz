pub fn read_u8(memory: &Vec<u8>, cursor: &mut u64) -> u8 {
    let result = memory[*cursor as usize];

    *cursor += 1;

    return result;
}

pub fn read_u16(memory: &Vec<u8>, cursor: &mut u64) -> u16 {
    let result = (memory[*cursor as usize + 1] as u16) << 8 | memory[*cursor as usize] as u16;

    *cursor += 2;

    return result;
}

pub fn read_u32(memory: &Vec<u8>, cursor: &mut u64) -> u32 {
    let result = (memory[*cursor as usize + 3] as u32) << 24
        | (memory[*cursor as usize + 2] as u32) << 16
        | (memory[*cursor as usize + 1] as u32) << 8
        | memory[*cursor as usize] as u32;

    *cursor += 4;

    return result;
}

pub fn read_u64(memory: &Vec<u8>, cursor: &mut u64) -> u64 {
    let mut result = memory[*cursor as usize] as u64;
    result |= (memory[*cursor as usize + 1] as u64) << 8;
    result |= (memory[*cursor as usize + 2] as u64) << 16;
    result |= (memory[*cursor as usize + 3] as u64) << 24;
    result |= (memory[*cursor as usize + 4] as u64) << 32;
    result |= (memory[*cursor as usize + 5] as u64) << 40;
    result |= (memory[*cursor as usize + 6] as u64) << 48;
    result |= (memory[*cursor as usize + 7] as u64) << 52;

    *cursor += 8;

    return result;
}

pub fn read_i8(memory: &Vec<u8>, cursor: &mut u64) -> i8 {
    let result = memory[*cursor as usize] as i8;

    *cursor += 1;

    return result;
}

pub fn read_i32(memory: &Vec<u8>, cursor: &mut u64) -> i32 {
    let result = (memory[*cursor as usize + 3] as i32) << 24
        | (memory[*cursor as usize + 2] as i32) << 16
        | (memory[*cursor as usize + 1] as i32) << 8
        | memory[*cursor as usize] as i32;

    *cursor += 4;

    return result;
}

pub fn read_ascii(memory: &Vec<u8>, cursor: &mut u64, length: u64) -> String {
    let result =
        std::str::from_utf8(&memory[*cursor as usize..(*cursor + length) as usize]).unwrap();

    *cursor += length;

    return String::from(result);
}

pub fn read_null_terminated_ascii(memory: &Vec<u8>, cursor: &mut u64) -> String {
    let mut result = String::new();

    while (memory[*cursor as usize] != 0x00) {
        result += std::str::from_utf8(&memory[*cursor as usize..(*cursor as usize + 1)]).unwrap();

        *cursor += 1;
    }

    *cursor += 1;

    return result;
}
