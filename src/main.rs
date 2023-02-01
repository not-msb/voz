use std::env::args;
use voz::*;

fn main() {
    let mut constants = vec![];
    let mut program = vec![];
    let args: Vec<String> = args().skip(1).collect();

    for arg in &args {
        match arg.parse::<usize>() {
            Ok(_) => break,
            Err(_) => constants.push(arg),
        }
    }

    let bytecode: Vec<usize> = args.iter().skip(constants.len()).map(|arg| arg.parse().unwrap()).collect();
    for code in bytecode.chunks_exact(4) {
        program.push(Op(Opcode::from(code[0]), code[1], code[2], code[3]));
    }

    let mut vm = Vm::new(constants.into_iter().cloned().collect(), program);
    vm.eval();
    // println!("{:?}", vm.memory);
    // println!("{:?}", vm.constants);
}
