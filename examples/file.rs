//Broken

use voz::*;

fn main() {
    let mut vm = Vm::new(
        vec![
            String::from("0   "),
            String::from("255 "),
            String::from("img.ppm"),
        ],
        vec![
            // Setup
            // mem[0] :: left
            // mem[1] :: up
            // mem[2] :: width
            // mem[3] :: height
            // mem[4] :: x
            // mem[5] :: y
            Op(Opcode::MovConst, 1015, 0, 0),
            Op(Opcode::MovConst, 1019, 1, 0),
            Op(Opcode::CreateFile, 2, 0, 0),
            Op(Opcode::Mov, 0, 5, 0),
            Op(Opcode::Mov, 1, 5, 0),
            Op(Opcode::Mov, 2, 5, 0),
            Op(Opcode::Mov, 3, 5, 0),
            Op(Opcode::Dup, 4, 0, 0),
            Op(Opcode::Dup, 5, 1, 0),
            // Logic
            // (x<left+width) ? x += 1 : x -= 10
            Op(Opcode::Dup, 6, 0, 0),
            Op(Opcode::Add, 6, 2, 0),
            Op(Opcode::Jl, 12, 4, 6),
            Op(Opcode::Sub, 4, 2, 0),
            Op(Opcode::Jmp, 14, 0, 0),
            Op(Opcode::Mov, 6, 1, 0),
            Op(Opcode::Add, 4, 6, 0),
            // (y<up+height) ? y += 1 : end
            Op(Opcode::Dup, 6, 1, 0),
            Op(Opcode::Add, 6, 3, 0),
            Op(Opcode::Jge, 30, 5, 6),
            Op(Opcode::Mov, 6, 1, 0),
            Op(Opcode::Add, 5, 6, 0),
            // (x > left && x < left+width && y > up && y < up+height) ? insize : outside
            // x > left
            Op(Opcode::Jle, 28, 4, 0),
            // x < left+width
            Op(Opcode::Dup, 6, 0, 0),
            Op(Opcode::Add, 6, 2, 0),
            Op(Opcode::Jge, 28, 4, 6),
            // y > up
            Op(Opcode::Jle, 28, 5, 1),
            // y < up+height
            Op(Opcode::Dup, 6, 1, 0),
            Op(Opcode::Add, 6, 3, 0),
            Op(Opcode::Jge, 28, 5, 6),
            // Inside
            Op(Opcode::Write, 3, 1019, 4), // 26
            Op(Opcode::Jmp, 7, 0, 0),
            // Outside
            Op(Opcode::Write, 3, 1015, 4), // 28
            Op(Opcode::Jmp, 7, 0, 0),
            // End
            Op(Opcode::Hlt, 0, 0, 0),
        ],
    );
    vm.eval();
}
