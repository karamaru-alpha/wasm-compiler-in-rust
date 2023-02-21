use crate::ast::{Ident, Expression, Program, Statement, Infix, Literal};
use std::{fs::File, io::prelude::*};
use std::collections::HashMap;

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
    I32Const = 0x41,
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
        let function_section = self.build_function_section();
        let export_section = self.build_export_section();
        let code_section = self.build_code_section();

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
        println!("{:?}", self.build_code_section());
    }

    fn build_function_section(&self) -> Vec<u8> {
        let function_num = self.program.statements.len() as u8;
        let mut body = vec![function_num];
        body.extend(0..function_num);
        build_section(Section::Func, body)
    }

    fn build_export_section(&self) -> Vec<u8> {
        let mut body = vec![self.program.statements.len() as u8];
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

    fn build_code_section(&self) -> Vec<u8> {
        let mut body = vec![self.program.statements.len() as u8];
        for (i, statement) in self.program.statements.iter().enumerate() {
            if let Statement::Expression(Expression::Fn {args, blocks, .. }) = statement {
                body.extend(build_code_function_section(i as u8, args, blocks));
            }
        }
        build_section(Section::Code, body)
    }
}

fn build_code_function_section(function_index: u8, args: &Vec<Ident>, blocks: &Vec<Statement>) -> Vec<u8> {
    let arg_hash: HashMap<String, u8> = args.iter().enumerate()
        .map(|(i, arg)| (arg.0.clone(), i as u8))
        .collect();
    let mut body = vec![function_index];
    for statement in blocks.iter() {
        if let Statement::Expression(Expression::Infix(infix, left, right)) = statement {
            emit_infix_expression(&mut body, &arg_hash, left.as_ref());
            emit_infix_expression(&mut body, &arg_hash, right.as_ref());
            match infix {
                Infix::Plus => body.push(Opcode::I32Add as u8),
            }
        }
    }
    body.push(Opcode::End as u8);
    body.insert(0, body.len() as u8);
    body
}

fn emit_infix_expression(body: &mut Vec<u8>, arg_hash: &HashMap<String, u8>, expr: &Expression) {
    match expr {
        Expression::Ident(ident) => {
            if let Some(&index) = arg_hash.get(&ident.0) {
                body.extend_from_slice(&[Opcode::GetLocal as u8, index]);
            }
        }
        Expression::Literal(lit) => {
            match lit {
                Literal::Int(v) => {
                    body.push(Opcode::I32Const as u8);
                    let leading_zeros = v.leading_zeros() / 8;
                    body.extend(&v.to_be_bytes()[leading_zeros as usize..]);
                }
            }
        }
        Expression::Infix(next_infix, left, right) => {
            emit_infix_expression(body, arg_hash , left.as_ref());
            emit_infix_expression(body, arg_hash, right.as_ref());
            match next_infix {
                Infix::Plus => body.push(Opcode::I32Add as u8),
            }
        }
        _ => {}
    }
}

fn build_section(section: Section, body: Vec<u8>) -> Vec<u8> {
    let mut section = vec![section as u8, body.len() as u8];
    section.extend(body);
    section
}
