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

    let magic = &f[0..4];
    let version = &f[4..8];
    let type_section = &f[8..9];
    let mut type_section_size = &f[9..10];

    let courser = 8;
    for byte in courser..f.len() {
        println!("{:#02X?}", byte)
    }

    // let num = u8::from_le_bytes(<[u8; 1]>::try_from(type_section_size).unwrap());

    // println!("magic = {:#02X?}", magic);
    // println!("version = {:#02X?}", version);
    // println!("type section = {:#02X?}", type_section);
    // println!("type section_size = {:#02X?}", type_section_size);
    // dbg!(f.len());

    todo!()
}
