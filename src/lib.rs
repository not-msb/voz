#![feature(box_syntax)]

use std::{
    fmt::Debug,
    fs::File,
    io::{stderr, stdout, Read, Write}, net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream},
};

#[repr(usize)]
#[derive(Debug, Clone, Copy)]
pub enum Opcode {
    /// (val)
    Hlt,
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
    /// (src, val)
    LoadConst,
    /// (src)
    CreateFile,
    /// (src)
    LoadFile,
    /// (src)
    LoadConn,
    /// (val, src, val)
    Write,
}

impl From<usize> for Opcode {
    fn from(value: usize) -> Self {
        match value {
            0 => Opcode::Hlt,
            1 => Opcode::Mov,
            2 => Opcode::Dup,
            3 => Opcode::Inc,
            4 => Opcode::Dec,
            5 => Opcode::Add,
            6 => Opcode::Sub,
            7 => Opcode::Mul,
            8 => Opcode::Div,
            9 => Opcode::Jmp,
            10 => Opcode::Je,
            11 => Opcode::Jne,
            12 => Opcode::Jg,
            13 => Opcode::Jge,
            14 => Opcode::Jl,
            15 => Opcode::Jle,
            16 => Opcode::MovConst,
            17 => Opcode::LoadConst,
            18 => Opcode::CreateFile,
            19 => Opcode::LoadFile,
            20 => Opcode::LoadConn,
            21 => Opcode::Write,
            _ => unimplemented!(),
        }
    }
}

// Op :: opcode, operand1, operand2, operand3
#[derive(Debug, Clone, Copy)]
pub struct Op(pub Opcode, pub usize, pub usize, pub usize);

trait Buffer: Write + Read {}
impl<T> Buffer for T where T: Write + Read {}

pub struct Vm {
    pub memory: [usize; 1024],
    pub constants: Vec<Vec<usize>>,
    buffers: Vec<Box<dyn Buffer>>,
    program: Vec<Op>,
    pointer: usize,
}

impl Vm {
    pub fn new(constants: Vec<String>, program: Vec<Op>) -> Vm {
        Vm {
            memory: [0; 1024],
            constants: constants
                .iter()
                .map(|c| c.chars().into_iter().map(|c| c as usize).collect())
                .collect(),
            buffers: vec![],
            program,
            pointer: 0,
        }
    }

    pub fn eval(&mut self) -> usize {
        loop {
            if self.pointer >= self.program.len() {
                break 0;
            }

            let mut offset = 1;
            let op = &self.program[self.pointer];
            match op.0 {
                Opcode::Hlt => break op.1,
                Opcode::Mov => self.memory[op.1] = op.2,
                Opcode::Dup => self.memory[op.1] = self.memory[op.2],
                Opcode::Inc => self.memory[op.1] += 1,
                Opcode::Dec => self.memory[op.1] -= 1,
                Opcode::Add => self.memory[op.1] += self.memory[op.2],
                Opcode::Sub => self.memory[op.1] -= self.memory[op.2],
                Opcode::Mul => self.memory[op.1] *= self.memory[op.2],
                Opcode::Div => self.memory[op.1] /= self.memory[op.2],
                Opcode::Jmp => {
                    offset = 0;
                    self.pointer = op.1;
                }
                Opcode::Je if self.memory[op.2] == self.memory[op.3] => {
                    offset = 0;
                    self.pointer = op.1;
                }
                Opcode::Jne if self.memory[op.2] != self.memory[op.3] => {
                    offset = 0;
                    self.pointer = op.1;
                }
                Opcode::Jg if self.memory[op.2] > self.memory[op.3] => {
                    offset = 0;
                    self.pointer = op.1;
                }
                Opcode::Jge if self.memory[op.2] >= self.memory[op.3] => {
                    offset = 0;
                    self.pointer = op.1;
                }
                Opcode::Jl if self.memory[op.2] < self.memory[op.3] => {
                    offset = 0;
                    self.pointer = op.1;
                }
                Opcode::Jle if self.memory[op.2] <= self.memory[op.3] => {
                    offset = 0;
                    self.pointer = op.1;
                }
                Opcode::MovConst => {
                    let src = &self.constants[op.2];
                    self.memory[op.1..op.1 + src.len()].copy_from_slice(src);
                }
                Opcode::LoadConst => {
                    let src = &self.memory[op.1..op.1 + op.2];
                    self.constants.push(src.to_vec());
                }
                Opcode::CreateFile => {
                    let filename: String = self.constants[op.1]
                        .iter()
                        .map(|u| char::from_u32(*u as u32).unwrap())
                        .collect();
                    let file = File::create(filename).unwrap();
                    self.buffers.push(box file);
                }
                Opcode::LoadFile => {
                    let filename: String = self.constants[op.1]
                        .iter()
                        .map(|u| char::from_u32(*u as u32).unwrap())
                        .collect();
                    let file = File::open(filename).unwrap();
                    self.buffers.push(box file);
                }
                Opcode::LoadConn => {
                    let stream = TcpStream::connect("127.0.0.1:80").unwrap();
                    self.buffers.push(box stream);
                }
                Opcode::Write => {
                    let mut stdout = stdout();
                    let mut stderr = stderr();
                    let buffer: &mut dyn Write = match op.1 {
                        0 => panic!("Can't write to stdin!"),
                        1 => &mut stdout,
                        2 => &mut stderr,
                        n => &mut self.buffers[n - 3],
                    };
                    let src: Vec<u8> = self.memory[op.2..op.2 + op.3]
                        .iter()
                        .map(|u| *u as u8)
                        .collect();
                    buffer.write_all(&src).unwrap()
                }
                _ => {}
            }

            self.pointer += offset;
        }
    }
}
