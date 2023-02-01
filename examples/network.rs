use voz::*;

fn main() {
    let mut vm = Vm::new(
        vec![],
        vec![
            Op(Opcode::LoadConn, 0, 0, 0),
            Op(Opcode::Mov, 0, 5, 0),
            Op(Opcode::Write, 3, 0, 1)
        ]
    );
    vm.eval();
}