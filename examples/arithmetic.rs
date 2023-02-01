use voz::*;

fn main() {
    let mut vm = Vm::new(
        vec![],
        vec![
            Op(Opcode::Mov, 0, 65, 0), // 65 => mem[0]
            Op(Opcode::Mov, 1, 1, 0),  // 1 => mem[1]
            Op(Opcode::Mov, 2, 1, 0),  // 1 => mem[2]
            Op(Opcode::Add, 1, 2, 0),  // mem[1] + mem[2] => mem[1]
            Op(Opcode::Je, 6, 1, 0),   // if (mem[1] == mem[0]) jmp step[6]
            Op(Opcode::Jmp, 3, 0, 0),  // jmp step[3]
            Op(Opcode::Write, 1, 1, 1),
        ],
    );
    vm.eval().unwrap();
}
