#![feature(box_syntax)]

use std::{
    error::Error,
    fmt::Debug,
    fs::File,
    io::{stderr, stdin, stdout, Read, Write},
    net::{TcpListener, TcpStream},
};

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
    /// (src, val)
    LoadConst,
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

impl From<usize> for Opcode {
    fn from(value: usize) -> Self {
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
            19 => Opcode::LoadConst,
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

// Op :: opcode, operand1, operand2, operand3
#[derive(Debug, Clone, Copy)]
pub struct Op(pub Opcode, pub usize, pub usize, pub usize);

trait Buffer: Write + Read {}
impl<T> Buffer for T where T: Write + Read {}

pub struct Vm {
    memory: [usize; 1024],
    constants: Vec<Vec<usize>>,
    buffers: Vec<Box<dyn Buffer>>,
    calls: Vec<usize>,
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
            calls: vec![],
            program,
            pointer: 0,
        }
    }

    pub fn eval(&mut self) -> Result<(), Box<dyn Error>> {
        loop {
            if self.pointer >= self.program.len() {
                break Ok(());
            }

            let mut offset = 1;
            let op = &self.program[self.pointer];
            match op.0 {
                Opcode::Hlt => break Ok(()),
                Opcode::Ret => {
                    offset = 0;
                    self.pointer = *self.calls.last().unwrap();
                }
                Opcode::Mov => self.memory[op.1] = op.2,
                Opcode::Dup => self.memory[op.1] = self.memory[op.2],
                Opcode::Inc => self.memory[op.1] += 1,
                Opcode::Dec => self.memory[op.1] -= 1,
                Opcode::Add => self.memory[op.1] += self.memory[op.2],
                Opcode::Sub => self.memory[op.1] -= self.memory[op.2],
                Opcode::Mul => self.memory[op.1] *= self.memory[op.2],
                Opcode::Div => self.memory[op.1] /= self.memory[op.2],
                Opcode::Call => {
                    self.calls.push(self.pointer);
                    offset = 0;
                    self.pointer = op.1;
                }
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
                        .map(|u| unsafe { char::from_u32_unchecked(*u as u32) })
                        .collect();
                    let file = File::create(filename)?;
                    self.buffers.push(box file);
                }
                Opcode::LoadFile => {
                    let filename: String = self.constants[op.1]
                        .iter()
                        .map(|u| unsafe { char::from_u32_unchecked(*u as u32) })
                        .collect();
                    let file = File::open(filename)?;
                    self.buffers.push(box file);
                }
                Opcode::LoadConn => {
                    let addr: String = self.constants[op.1]
                        .iter()
                        .map(|u| unsafe { char::from_u32_unchecked(*u as u32) })
                        .collect();
                    let stream = TcpStream::connect(addr)?;
                    self.buffers.push(box stream);
                }
                Opcode::ListConn => {
                    let addr: String = self.constants[op.1]
                        .iter()
                        .map(|u| unsafe { char::from_u32_unchecked(*u as u32) })
                        .collect();
                    let listener = TcpListener::bind(addr)?;
                    let (stream, _addr) = listener.accept()?;
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
                    buffer.write_all(&src)?
                }
                Opcode::Read => {
                    let mut stdin = stdin();
                    let buffer: &mut dyn Read = match op.1 {
                        0 => &mut stdin,
                        1 => panic!("Can't read from stdout!"),
                        2 => panic!("Can't read from stderr!"),
                        n => &mut self.buffers[n - 3],
                    };
                    let mut src = vec![0; op.3];
                    buffer.read_exact(&mut src).unwrap();
                    let src: Vec<usize> = src.iter().map(|b| *b as usize).collect();
                    self.memory[op.2..op.2 + op.3].copy_from_slice(&src);
                }
                _ => {}
            }

            self.pointer += offset;
        }
    }
}
