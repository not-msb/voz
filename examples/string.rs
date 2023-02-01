use voz::*;

fn main() {
    let mut vm = Vm::new(
        vec![String::from("Hello World!\n")],
        vec![Op(Opcode::MovConst, 0, 0, 0), Op(Opcode::Write, 1, 0, 13)],
    );
    vm.eval().unwrap();
}
