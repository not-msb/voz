use voz::*;

fn main() {
    let mut vm = Vm::new(
        vec![
            String::from("img.ppm"),
            String::from("P1 2 2\n"),
            String::from("0 "),
            String::from("1 "),
        ],
        vec![
            Op(Opcode::CreateFile, 0, 0, 0),
            Op(Opcode::MovConst, 1010, 1, 0),
            Op(Opcode::Write, 3, 1010, 7),
            Op(Opcode::MovConst, 1008, 2, 0),
            Op(Opcode::MovConst, 1006, 3, 0),
            Op(Opcode::Write, 3, 1008, 2),
            Op(Opcode::Write, 3, 1006, 2),
            Op(Opcode::Write, 3, 1008, 2),
            Op(Opcode::Write, 3, 1006, 2),
        ],
    );
    vm.eval().unwrap();
}
