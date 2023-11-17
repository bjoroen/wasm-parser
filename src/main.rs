/// ## Sections
/// ### Each section consists of:
/// - one-byte section id,
/// - the size of the contents, in bytes,
/// - the actual contents, whose structure is dependent on the section id.
///
/// Every section is optional; an omitted section is equivalent to the section being present with empty contents.
#[derive(Debug, PartialEq, Clone)]
enum WasmSection {
    Custom(Vec<u8>),
    Type(Vec<u8>),
    Import(Vec<u8>),
    Function(Vec<u8>),
    Table(Vec<u8>),
    Memory(Vec<u8>),
    Global(Vec<u8>),
    Export(Vec<u8>),
    Start(Vec<u8>),
    Element(Vec<u8>),
    Code(Vec<u8>),
    Data(Vec<u8>),
    DataCount(Vec<u8>),
}

fn main() -> Result<(), std::io::Error> {
    let filename = String::from("./test.wasm");
    let f = std::fs::read(&filename)?;

    match parse_wasm_section(f) {
        Ok(v) => println!("{:#?}", v),
        Err(e) => panic!("Error: {}", e),
    };

    Ok(())
}

fn parse_wasm_section(f: Vec<u8>) -> Result<Vec<WasmSection>, std::io::Error> {
    let mut cursor = 8;
    let mut sections: Vec<WasmSection> = vec![];

    while cursor < f.len() {
        let size_of_section = f[cursor + 1] as usize;

        let section = match f[cursor] as u32 {
            0 => WasmSection::Custom(f[cursor + 2..cursor + size_of_section + 2].to_vec()),
            1 => WasmSection::Type(f[cursor + 2..cursor + size_of_section + 2].to_vec()),
            2 => WasmSection::Import(f[cursor + 2..cursor + size_of_section + 2].to_vec()),
            3 => WasmSection::Function(f[cursor + 2..cursor + size_of_section + 2].to_vec()),
            4 => WasmSection::Table(f[cursor + 2..cursor + size_of_section + 2].to_vec()),
            5 => WasmSection::Memory(f[cursor + 2..cursor + size_of_section + 2].to_vec()),
            6 => WasmSection::Global(f[cursor + 2..cursor + size_of_section + 2].to_vec()),
            7 => WasmSection::Export(f[cursor + 2..cursor + size_of_section + 2].to_vec()),
            8 => WasmSection::Start(f[cursor + 2..cursor + size_of_section + 2].to_vec()),
            9 => WasmSection::Element(f[cursor + 2..cursor + size_of_section + 2].to_vec()),
            10 => WasmSection::Code(f[cursor + 2..cursor + size_of_section + 2].to_vec()),
            11 => WasmSection::Data(f[cursor + 2..cursor + size_of_section + 2].to_vec()),
            12 => WasmSection::DataCount(f[cursor + 2..cursor + size_of_section + 2].to_vec()),
            s => panic!("Unknown section id, {:?}", s),
        };

        cursor = cursor + size_of_section + 2;

        sections.push(section);
    }

    Ok(sections)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_section() {
        let wasm: Vec<u8> = vec![
            00, 00, 00, 00, 00, 00, 00, 00, 01, 05, 00, 00, 00, 00, 00, 03, 02, 01, 0x7f,
        ];
        let result = parse_wasm_section(wasm).unwrap();

        assert_eq!(
            result,
            vec![
                WasmSection::Type(vec![00, 00, 00, 00, 00]),
                WasmSection::Function(vec![01, 127])
            ]
        )
    }
}
