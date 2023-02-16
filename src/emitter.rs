use crate::ast::Program;
use std::fs::File;
use std::io::prelude::*;

// https://webassembly.github.io/spec/core/binary/modules.html#sections
enum Section {
    Type = 0x01,
    Func = 0x03,
    Export = 0x07,
    Code = 0x0a
    ,
}

// https://webassembly.github.io/spec/core/binary/types.html
enum Type {
    I32 = 0x7f,
}

// http://webassembly.github.io/spec/core/binary/modules.html#export-section
enum ExportType {
    Func = 0x00,
}

// http://webassembly.github.io/spec/core/binary/types.html#function-types
const function_type: u8 = 0x60;

// https://webassembly.github.io/spec/core/binary/instructions.html
enum Opcode {
    GetLocal = 0x20,
    // I32Const = 0x41,
    I32Add = 0x6a,
    End = 0x0b,
}

pub(crate) fn emit(program: Program) {
    println!("{:?}", program.statements);

    let magic_module_header = vec![0x00, 0x61, 0x73, 0x6d];
    let module_version = vec![0x01, 0x00, 0x00, 0x00];

    let type_section = vec![
        Section::Type as u8,
        0x07, // section size
        0x01, // num function types
        function_type, // func
        0x02, // num params
        Type::I32 as u8,
        Type::I32 as u8,
        0x01, // num results
        Type::I32 as u8,
    ];
    let function_section = vec![
        Section::Func as u8,
        0x02, // section size
        0x01, // num functions
        0x00, // function 0 signature index
    ];
    let export_section = vec![
        Section::Export as u8,
        0x07,                   // section size
        0x01,                   // num exports
        0x03,                   // string length
        0x61,                   // a
        0x64,                   // d
        0x64,                   // d
        ExportType::Func as u8, // export type
        0x00,                   // export func index
    ];
    let code_section = vec![
        Section::Code as u8,
        0x09,                   // section size
        0x01,                   // num functions
        0x07,                   // func body size
        0x00,                   // local decl count
        Opcode::GetLocal as u8, // local.get
        0x00,                   // index
        Opcode::GetLocal as u8, // local.get
        0x01,                   // index
        Opcode::I32Add as u8,
        Opcode::End as u8,
    ];

    let wasm: &[u8] = [
        magic_module_header,
        module_version,
        type_section,
        function_section,
        export_section,
        code_section,
    ]
    .concat()
    .leak();

    let mut file = File::create("a.wasm").expect("err file create");
    file.write_all(wasm).expect("err file write");
    println!("wasm file is output! {wasm:?}");
}
