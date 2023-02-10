mod asm_string;

use std::{
    fmt::Debug,
    io::{Read, Write}, fs::File,
};
use asm_string::*;

#[repr(usize)]
#[derive(Debug, Clone, Copy)]
pub enum Opcode {
    /// ()
    Hlt,
    /// ()
    Ret,
    /// (dst, val)
    Mov,
    /// (dst, src)
    Dup,
    /// (dst)
    Inc,
    /// (dst)
    Dec,
    /// (dst, src)
    Add,
    /// (dst, src)
    Sub,
    /// (dst, src)
    Mul,
    /// (dst, src)
    Div,
    /// (dst)
    Call,
    /// (dst)
    Jmp,
    /// (dst, src, src)
    Je,
    /// (dst, src, src)
    Jne,
    /// (dst, src, src)
    Jg,
    /// (dst, src, src)
    Jge,
    /// (dst, src, src)
    Jl,
    /// (dst, src, src)
    Jle,
    /// (dst, src)
    MovConst,
    /// (src)
    CreateFile,
    /// (src)
    LoadFile,
    /// (src)
    LoadConn,
    /// (src)
    ListConn,
    /// (val, src, val)
    Write,
    /// (val, dst, val)
    Read,
}

impl From<u32> for Opcode {
    fn from(value: u32) -> Self {
        match value {
            0 => Opcode::Hlt,
            1 => Opcode::Ret,
            2 => Opcode::Mov,
            3 => Opcode::Dup,
            4 => Opcode::Inc,
            5 => Opcode::Dec,
            6 => Opcode::Add,
            7 => Opcode::Sub,
            8 => Opcode::Mul,
            9 => Opcode::Div,
            10 => Opcode::Call,
            11 => Opcode::Jmp,
            12 => Opcode::Je,
            13 => Opcode::Jne,
            14 => Opcode::Jg,
            15 => Opcode::Jge,
            16 => Opcode::Jl,
            17 => Opcode::Jle,
            18 => Opcode::MovConst,
            20 => Opcode::CreateFile,
            21 => Opcode::LoadFile,
            22 => Opcode::LoadConn,
            23 => Opcode::ListConn,
            24 => Opcode::Write,
            25 => Opcode::Read,
            _ => unimplemented!(),
        }
    }
}

impl Opcode {
    fn as_str(&self) -> &str {
        use Opcode::*;
        match self {
            Inc => "inc",
            Dec => "dec",
            Jmp => "jmp",
            Je => "je",
            Jne => "jne",
            Jg => "jg",
            Jge => "jge",
            Jl => "jl",
            Jle => "jle",
            _ => unimplemented!()
        }
    }
}

// Op :: opcode, operand1, operand2, operand3
#[derive(Debug, Clone, Copy)]
pub struct Op(pub Opcode, pub u32, pub u32, pub u32);

trait Buffer: Write + Read {}
impl<T> Buffer for T where T: Write + Read {}

pub struct Compiler {
    constants: Vec<String>,
    jumps: Vec<u32>,
    program: Vec<Op>,
    pointer: usize,
}

impl Compiler {
    pub fn new(constants: Vec<String>, program: Vec<Op>) -> Compiler {
        Compiler {
            constants,
            jumps: vec![],
            program,
            pointer: 0,
        }
    }

    pub fn compile(&mut self) {
        use Opcode::*;

        let mut file = File::create("build/out.asm").unwrap();
        let mut buffer = vec![];

        file.write_all("global _start\nsection .text\n_start:\npush rbp\nmov rbp, rsp\n".as_bytes()).unwrap();

        loop {
            if self.pointer >= self.program.len() {
                break;
            }

            let op = &self.program[self.pointer];

            let line = match op.0 {
                Hlt => "jmp exit".to_string(),
                Ret => "ret".to_string(),
                Mov => format!("push {}", op.2),
                Dup => format!("mov rax, [rbp-{}]\nmov [rbp-{}], rax", op.2*8+8, op.1*8+8),
                Inc | Dec => format!("mov rax, [rbp-{}]\n{} rax\nmov [rbp-{}], rax", op.1*8+8, op.0.as_str(), op.1*8+8),
                Add => format!("mov rax, [rbp-{}]\nadd rax, [rbp-{}]\nmov [rbp-{}], rax", op.1*8+8, op.2*8+8, op.1*8+8),
                Sub => format!("mov rax, [rbp-{}]\nsub rax, [rbp-{}]\nmov [rbp-{}], rax", op.1*8+8, op.2*8+8, op.1*8+8),
                Mul => format!("mov rax, [rbp-{}]\nmul [rbp-{}]\nmov [rbp-{}], rax", op.1*8+8, op.2*8+8, op.1*8+8),
                Div => format!("mov rdx, 0\nmov rax, [rbp-{}]\ndiv [rbp-{}]\nmov [rbp-{}], rax", op.1*8+8, op.2*8+8, op.1*8+8),
                Call => format!("call {}", self.constants[op.1 as usize]),
                Jmp | Je | Jne | Jg | Jge | Jl | Jle => {
                    if !self.jumps.contains(&op.1) {
                        self.jumps.push(op.1);
                    }
                    format!("mov rax, [rbp-{}]\ncmp rax, [rbp-{}]\n{} l{}", op.2*8+8, op.3*8+8, op.0.as_str(), op.1)
                }
                MovConst => format!("mov rax, c{}\nmov [rbp-{}], rax", op.2, op.1*8+8),
                CreateFile => format!("mov rax, 85\nmov rdi, c{}\nmov rsi, 0o666\nsyscall\nsub rbp, {}\nmov [rbp], rax\nadd rbp, {}", op.1, op.2*8+8, op.2*8+8),
                LoadFile => format!("mov rax, 2\nmov rdi, c{}\nmov rsi, 0o2\nmov rdx, 0o666\nsyscall\nsub rbp, {}\nmov [rbp], rax\nadd rbp, {}", op.1, op.2*8+8, op.2*8+8),
                LoadConn => todo!(),
                ListConn => todo!(),
                Write => {
                    if op.1 < 3 {
                        format!("mov rax, 1\nmov rdi, {}\nmov rsi, [rbp-{}]\nmov rdx, {}\nsyscall", op.1, op.2*8+8, op.3)
                    } else {
                        format!("mov rax, 1\nmov rdi, [rbp-{}]\nmov rsi, [rbp-{}]\nmov rdx, {}\nsyscall", (op.1-3)*8+8, op.2*8+8, op.3)
                    }
                },
                Read => todo!(),
            };

            buffer.push(line);

            self.pointer += 1;
        }

        self.jumps.sort();
        for (offset, jump) in self.jumps.iter().map(|u| *u as usize).enumerate() {
            buffer.insert(jump+offset, format!("l{jump}:"));
        }

        let mut connected = buffer.join("\n");
        connected.push('\n');
        file.write_all(connected.as_bytes()).unwrap();
        
        file.write_all("exit:\nmov rsp, rbp\npop rbp\nmov rax, 60\nmov rdi, 0\nsyscall\n".as_bytes()).unwrap();

        file.write_all("section .data\n".as_bytes()).unwrap();
        for (i, constant) in self.constants.iter().enumerate() {
            let astr = AsmString(constant);
            file.write_all(format!("c{i} db {astr}\nc{i}_len equ $-c{i}\n").as_bytes()).unwrap();
        }
    }
}
