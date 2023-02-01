use voz::*;

fn main() {
    let mut vm = Vm::new(
        vec![
            String::from("127.0.0.1:80"),
            String::from("New Connection\n"),
        ],
        vec![
            Op(Opcode::ListConn, 0, 0, 0),
            Op(Opcode::MovConst, 0, 1, 0),
            Op(Opcode::Write, 1, 0, 15),
        ],
    );
    vm.eval().unwrap();
}
