use crate::ast::{Ident, Expression, Program, Statement, Infix, Literal};
use std::collections::{HashSet, HashMap};

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
    I32Sub = 0x6b,
    I32Mul = 0x6c,
    I32Div = 0x6d,
    End = 0x0b,
}

pub struct Emitter {
    program: Program,
    signature_map: HashMap<usize, usize>, // arg_count: index
}

impl Emitter {
    pub fn new(program: Program) -> Self {
        Self {
            program,
            signature_map: HashMap::new(),
        }
    }

    pub fn emit(&mut self) -> Vec<u8> {
        if !self.program.statements.iter().any(|s| matches!(s, Statement::Expression(Expression::Fn { .. }))) {
            panic!("function not found");
        }

        let magic_module_header = vec![0x00, 0x61, 0x73, 0x6d];
        let module_version = vec![0x01, 0x00, 0x00, 0x00];
        let type_section = self.build_type_section();
        let function_section = self.build_function_section();
        let export_section = self.build_export_section();
        let code_section = self.build_code_section();

        [
            magic_module_header,
            module_version,
            type_section,
            function_section,
            export_section,
            code_section,
        ].concat()
    }

    fn build_type_section(&mut self) -> Vec<u8> {
        let mut args_count_set = HashSet::new();
        for statement in &self.program.statements {
            if let Statement::Expression(Expression::Fn { args, .. }) = statement {
                args_count_set.insert(args.len());
            }
        }
        let mut body = vec![args_count_set.len() as u8];
        for args_count in args_count_set {
            body.extend_from_slice(&[
                FUNCTION_TYPE,
                args_count as u8,
            ]);
            for _ in 0..args_count{
                body.push(Type::I32 as u8);
            }
            body.extend_from_slice(&[
                1,
                Type::I32 as u8,
            ]);
            let index = self.signature_map.len();
            self.signature_map.entry(args_count).or_insert(index);
        }
        build_section(Section::Type, body)
    }

    fn build_function_section(&self) -> Vec<u8> {
        let mut body = Vec::new();
        let mut function_count = 0;
        for statement in &self.program.statements {
            if let Statement::Expression(Expression::Fn { args, .. }) = statement {
                if let Some(&index) = self.signature_map.get(&args.len()) {
                    body.push(index as u8);
                    function_count += 1;
                }
            }
        }
        body.insert(0, function_count as u8);
        build_section(Section::Func, body)
    }

    fn build_export_section(&self) -> Vec<u8> {
        let mut body = Vec::new();
        let mut function_count = 0;
        for (i, statement) in self.program.statements.iter().enumerate() {
            if let Statement::Expression(Expression::Fn { ident, .. }) = statement {
                body.push(ident.0.len() as u8);
                body.extend_from_slice(ident.0.as_bytes());
                body.push(ExportType::Func as u8);
                body.push(i as u8);
                function_count += 1;
            }
        }
        body.insert(0, function_count as u8);
        build_section(Section::Export, body)
    }

    fn build_code_section(&self) -> Vec<u8> {
        let mut body = Vec::new();
        let mut function_count = 0;
        for statement in &self.program.statements {
            if let Statement::Expression(Expression::Fn {args, blocks, .. }) = statement {
                body.extend(build_code_function_section(args, blocks));
                function_count += 1;
            }
        }
        body.insert(0, function_count as u8);
        build_section(Section::Code, body)
    }
}

fn build_code_function_section(args: &[Ident], blocks: &[Statement]) -> Vec<u8> {
    let arg_hash: HashMap<String, u8> = args.iter().enumerate().map(|(i, arg)| (arg.0.clone(), i as u8)).collect();
    let mut body = vec![0]; // 関数内変数は使用しない
    for statement in blocks {
        match statement {
            Statement::Expression(Expression::Infix(infix, left, right)) => {
                emit_expression(&mut body, &arg_hash, left.as_ref());
                emit_expression(&mut body, &arg_hash, right.as_ref());
                match infix {
                    Infix::Plus => body.push(Opcode::I32Add as u8),
                    Infix::Minus => body.push(Opcode::I32Sub as u8),
                    Infix::Asterisk => body.push(Opcode::I32Mul as u8),
                    Infix::Slash => body.push(Opcode::I32Div as u8),
                }
            }
            Statement::Expression(Expression::Literal(Literal::Int(v))) => {
                body.push(Opcode::I32Const as u8);
                let leading_zeros = v.leading_zeros() / 8;
                body.extend(&v.to_be_bytes()[leading_zeros as usize..]);
            }
            _ => {}
        }
    }
    body.push(Opcode::End as u8);
    body.insert(0, body.len() as u8);
    body
}

fn emit_expression(body: &mut Vec<u8>, arg_hash: &HashMap<String, u8>, expr: &Expression) {
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
            emit_expression(body, arg_hash, left.as_ref());
            emit_expression(body, arg_hash, right.as_ref());
            match next_infix {
                Infix::Plus => body.push(Opcode::I32Add as u8),
                Infix::Minus => body.push(Opcode::I32Sub as u8),
                Infix::Asterisk => body.push(Opcode::I32Mul as u8),
                Infix::Slash => body.push(Opcode::I32Div as u8),
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
