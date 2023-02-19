use crate::ast::{Expression, Program, Statement};
use std::{fs::File, io::prelude::*};

// https://webassembly.github.io/spec/core/binary/modules.html#sections
enum Section {
    Type = 0x01,
    Func = 0x03,
    Export = 0x07,
    Code = 0x0a,
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
const FUNCTION_TYPE: u8 = 0x60;

// https://webassembly.github.io/spec/core/binary/instructions.html
enum Opcode {
    GetLocal = 0x20,
    // I32Const = 0x41,
    I32Add = 0x6a,
    End = 0x0b,
}

pub struct Emitter {
    program: Program,
}

impl Emitter {
    pub fn new(program: Program) -> Self {
        Self { program }
    }

    pub fn emit(&self) {
        let magic_module_header = vec![0x00, 0x61, 0x73, 0x6d];
        let module_version = vec![0x01, 0x00, 0x00, 0x00];

        let type_section = vec![
            Section::Type as u8,
            0x07,          // section size
            0x01,          // num function types
            FUNCTION_TYPE, // func
            0x02,          // num params
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
        let export_section = self.build_export_section();
        // let export_section = vec![
        //     Section::Export as u8,
        //     0x07,                   // section size
        //     0x01,                   // num exports
        //     0x03,                   // string length
        //     0x61,                   // a
        //     0x64,                   // d
        //     0x64,                   // d
        //     ExportType::Func as u8, // export type
        //     0x00,                   // export func index
        // ];
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

    fn build_export_section(&self) -> Vec<u8> {
        let mut body = Vec::new();
        body.push(self.program.statements.len() as u8);
        for (i, statement) in self.program.statements.iter().enumerate() {
            if let Statement::Expression(Expression::Fn { ident, .. }) = statement {
                body.push(ident.0.len() as u8);
                body.extend_from_slice(ident.0.as_bytes());
                body.push(ExportType::Func as u8);
                body.push(i as u8);
            }
        }
        build_section(Section::Export, body)
    }
}

fn build_section(section: Section, body: Vec<u8>) -> Vec<u8> {
    let mut section = vec![section as u8, body.len() as u8];
    section.extend(body);
    section
}
