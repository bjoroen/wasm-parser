use std::io::Read;

/// ## Sections
/// ### Each section consists of:
/// - one-byte section id,
/// - the size of the contents, in bytes,
/// - the actual contents, whose structure is dependent on the section id.
///
/// Every section is optional; an omitted section is equivalent to the section being present with empty contents.
#[derive(Debug, PartialEq)]
enum WasmSection {
    Custom,
    Type,
    Import,
    Function,
    Table,
    Memory,
    Global,
    Export,
    Start,
    Element,
    Code,
    Data,
    DataCount,
}

fn main() -> Result<(), std::io::Error> {
    let filename = String::from("./main.wasm");
    let f = std::fs::read(&filename)?;

    match parse_wasm_section(f) {
        Ok(v) => println!("{:#?}", v),
        Err(e) => panic!("Error: {}", e),
    };

    Ok(())
}

fn parse_wasm_section(f: Vec<u8>) -> Result<Vec<WasmSection>, std::io::Error> {
    let mut bytes = f.bytes().skip(8);
    let mut sections: Vec<WasmSection> = vec![];

    while let Some(courser) = bytes.next() {
        let section = match courser? {
            0 => WasmSection::Custom,
            1 => WasmSection::Type,
            2 => WasmSection::Import,
            3 => WasmSection::Function,
            4 => WasmSection::Table,
            5 => WasmSection::Memory,
            6 => WasmSection::Global,
            7 => WasmSection::Export,
            8 => WasmSection::Start,
            9 => WasmSection::Element,
            10 => WasmSection::Code,
            11 => WasmSection::Data,
            12 => WasmSection::DataCount,
            s => panic!("Unknown section id, {:?}", s),
        };

        let number_to_skip = bytes.next().unwrap().unwrap();

        bytes.nth(usize::try_from(number_to_skip).unwrap() - 1);

        sections.push(section);
        println!("Section: {:#?}", sections);
    }

    Ok(sections)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_section() {
        let wasm: Vec<u8> = vec![00, 00, 00, 00, 00, 00, 00, 00, 01, 05, 00, 00, 00, 00, 00];
        let result = parse_wasm_section(wasm).unwrap();

        assert_eq!(result, vec![WasmSection::Type])
    }
}
